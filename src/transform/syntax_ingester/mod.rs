mod shader_file_partial;
mod declaration_partials;
mod type_partials;

pub(crate) use self::{
  shader_file_partial::ShaderFilePartial,
  declaration_partials::{
    BufferDeclPartial,
    EntrypointDeclPartial,
    FuncDeclPartial,
    FuncDeclArgPartial,
    ImportDeclPartial,
    InstanceDeclPartial,
    ModuleDeclPartial,
    StructDeclPartial,
  },
  type_partials::TypeRefPartial,
};

use std::str;
use chumsky::{ Parser, extra };
use crate::{
  model::{
    EntrypointDims,
    ModelSpace,
    ShaderFileModel,
    ShaderFileModelHandle,
    StringModelHandle,
  },
  syntax::{
    declaration::{
      EntrypointDecl,
      BufferDecl,
      ImportDecl,
      InstanceDecl,
      FuncDecl,
      ModuleDecl,
      StructDecl,
    },
    file::{ ShaderFile, ShaderFileDeclaration },
    types::TypeName,
  },
  transform::SessionConfig,
};

pub struct SyntaxIngestionError;

/**
 * Ingests the syntax (AST) and generates an internal model representation.
 */
pub struct SyntaxIngester<'a> {
  // The session configuration.
  session_config: &'a SessionConfig,

  // The model space used to instantiate new model objects.
  model_space: ModelSpace,
}
impl<'a> SyntaxIngester<'a> {
  /**
   * Create a new syntax ingester.
   */
  fn new(session_config: &'a SessionConfig) -> Self {
    SyntaxIngester {
      session_config,
      model_space: ModelSpace::new(),
    }
  }

  /**
   * Generate a ShaderFileModel for a shader file within a session config.
   */
  pub fn parse_shader_file<'x: 'a>(
    session_config: &SessionConfig,
    sub_path: &str,
    contents: &'x str,
  ) -> ShaderFileModelHandle {
    let mut ingester = SyntaxIngester::new(&session_config);

    let sub_path = ingester.model_space.intern_string(sub_path);
    ingester.ingest_shader_file_contents(sub_path, contents)
  }

  fn ingest_shader_file_contents<'x: 'a>(&mut self,
    sub_path: StringModelHandle,
    file_contents: &'x str,
  ) -> ShaderFileModelHandle {
    let shader_file =
      ShaderFile::parser::<extra::Default>()
        .parse(file_contents)
        .into_result()
        .expect(
          &format!("Failed to parse file {:?}", sub_path)
        );
    let mut shader_file_partial = ShaderFilePartial::new(sub_path.clone());

    for decl in shader_file.declarations {
      self.ingest_shader_file_declaration(&mut shader_file_partial, decl);
    }

    self.model_space.add_shader_file_model(ShaderFileModel::new(sub_path))
  }

  /**
   * Ingest a shader file declaration.
   */
  fn ingest_shader_file_declaration(&mut self,
    partial: &mut ShaderFilePartial<'a>,
    decl: ShaderFileDeclaration<'a>,
  ) {
    match decl {
      ShaderFileDeclaration::Entrypoint(entrypoint_decl) => {
        self.ingest_entrypoint_decl(partial, entrypoint_decl);
      },
      ShaderFileDeclaration::Buffer(buffer_decl) => {
        self.ingest_buffer_decl(partial, buffer_decl);
      },
      ShaderFileDeclaration::Import(import_decl) => {
        self.ingest_import_decl(partial, import_decl);
      },
      ShaderFileDeclaration::Instance(instance_decl) => {
        self.ingest_instance_decl(partial, instance_decl);
      },
      ShaderFileDeclaration::Func(func_decl) => {
        self.ingest_func_decl(partial, func_decl);
      },
      ShaderFileDeclaration::Module(module_decl) => {
        self.ingest_module_decl(partial, module_decl);
      },
      ShaderFileDeclaration::Struct(struct_decl) => {
        self.ingest_struct_decl(partial, struct_decl);
      },
    }
  }

  /**
   * Ingest an entrypoint declaration.
   */
  fn ingest_entrypoint_decl(&mut self,
    partial: &mut ShaderFilePartial<'a>,
    entrypoint_decl: EntrypointDecl<'a>,
  ) {
    let name = self.model_space.intern_name(entrypoint_decl.name.contents);
    let arg_name =
      self.model_space.intern_name(entrypoint_decl.arg_name.contents);
    let dims = EntrypointDims::from_decl_dims(entrypoint_decl.dims);
    let body = Vec::new();
    partial.add_entrypoint_decl(EntrypointDeclPartial {
      name,
      dims,
      arg_name,
      body,
    });
  }

  /**
   * Ingest a buffer declaration.
   */
  fn ingest_buffer_decl(&mut self,
    partial: &mut ShaderFilePartial<'a>,
    buffer_decl: BufferDecl<'a>,
  ) {
    let name = self.model_space.intern_name(buffer_decl.name.contents);
    let ty = self.inflate_type_reference(partial, &buffer_decl.ty);
    partial.add_buffer_decl(BufferDeclPartial { name, ty });
  }

  /**
   * Ingest an import declaration.
   */
  fn ingest_import_decl(&mut self,
    partial: &mut ShaderFilePartial<'a>,
    import_decl: ImportDecl<'a>,
  ) {
    let path = import_decl.name_path;
    let name = import_decl.maybe_alias
      .map(|alias| self.model_space.intern_name(alias.contents))
      .unwrap_or_else(||
        self.model_space.intern_name(path.parts.last().unwrap().contents)
      );
    partial.add_import_decl(ImportDeclPartial { name, path });
  }

  /**
   * Ingest an instance declaration.
   */
  fn ingest_instance_decl(&mut self,
    partial: &mut ShaderFilePartial<'a>,
    instance_decl: InstanceDecl<'a>,
  ) {
    let name = self.model_space.intern_name(instance_decl.name.contents);
    partial.add_instance_decl(
      InstanceDeclPartial { name, syntax_decl: instance_decl }
    );
  }

  /**
   * Ingest a function declaration.
   */
  fn ingest_func_decl(&mut self,
    partial: &mut ShaderFilePartial<'a>,
    func_decl: FuncDecl<'a>,
  ) {
    let name = self.model_space.intern_name(func_decl.name.contents);
    let return_ty = func_decl.return_ty.as_ref().map(|ty|
      self.inflate_type_reference(partial, ty)
    );
    let args = func_decl.arguments.iter()
      .map(|arg| {
        FuncDeclArgPartial {
          name: self.model_space.intern_name(arg.name.contents),
          ty: self.inflate_type_reference(partial, &arg.ty),
        }
      })
      .collect();
    let body = Vec::new();
    partial.add_func_decl(FuncDeclPartial { name, return_ty, args, body });
  }

  /**
   * Ingest a module declaration.
   */
  fn ingest_module_decl(&mut self,
    partial: &mut ShaderFilePartial<'a>,
    module_decl: ModuleDecl<'a>,
  ) {
    let name = self.model_space.intern_name(module_decl.name.contents);
    partial.add_module_decl(
      ModuleDeclPartial { name, syntax_decl: module_decl }
    );
  }

  /**
   * Ingest a struct declaration.
   */
  fn ingest_struct_decl(&mut self,
    partial: &mut ShaderFilePartial<'a>,
    struct_decl: StructDecl<'a>,
  ) {
    let name = self.model_space.intern_name(struct_decl.name.contents);
    partial.add_struct_decl(StructDeclPartial { name, syntax_decl: struct_decl });
  }

  /**
   * Ingest a type-reference.
   */
  fn inflate_type_reference(&mut self,
    partial: &mut ShaderFilePartial,
    ty: &TypeName<'a>,
  ) -> TypeRefPartial<'a> {
    TypeRefPartial::from_type_name(ty)
  }
}
