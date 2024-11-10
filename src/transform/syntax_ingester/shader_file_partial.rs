use std::collections::HashMap;
use crate::{
  model::{ StringModelHandle, NameModelHandle },
  transform::syntax_ingester::declaration_partials::{
    EntrypointDeclPartial,
    BufferDeclPartial,
    ImportDeclPartial,
    InstanceDeclPartial,
    FuncDeclPartial,
    ModuleDeclPartial,
    StructDeclPartial,
  },
};

/**
 * Partial representation of a shader file.
 */
#[derive(Debug, Clone)]
pub struct ShaderFilePartial<'a> {
  pub(crate) path: StringModelHandle,
  pub(crate) declarations:
    HashMap<NameModelHandle, ShaderFileDeclarationPartial<'a>>,
}
impl<'a> ShaderFilePartial<'a> {
  pub(crate) fn new(path: StringModelHandle) -> ShaderFilePartial<'a> {
    ShaderFilePartial { path, declarations: HashMap::new() }
  }

  fn add(&mut self, name: NameModelHandle, decl: ShaderFileDeclarationPartial<'a>) {
    if self.declarations.contains_key(&name) {
      panic!("Duplicate declaration: {:?}", name);
    }
    self.declarations.insert(name, decl);
  }

  pub(crate) fn add_entrypoint_decl(&mut self, entrypoint_decl: EntrypointDeclPartial<'a>) {
    self.add(
      entrypoint_decl.name.clone(),
      ShaderFileDeclarationPartial::Entrypoint(entrypoint_decl)
    );
  }

  pub(crate) fn add_buffer_decl(&mut self, buffer_decl: BufferDeclPartial<'a>) {
    self.add(
      buffer_decl.name.clone(),
      ShaderFileDeclarationPartial::Buffer(buffer_decl)
    );
  }

  pub(crate) fn add_import_decl(&mut self, import_decl: ImportDeclPartial<'a>) {
    self.add(
      import_decl.name.clone(),
      ShaderFileDeclarationPartial::Import(import_decl)
    );
  }

  pub(crate) fn add_instance_decl(&mut self, instance_decl: InstanceDeclPartial<'a>) {
    self.declarations.insert(
      instance_decl.name.clone(),
      ShaderFileDeclarationPartial::Instance(instance_decl)
    );
  }

  pub(crate) fn add_func_decl(&mut self, func_decl: FuncDeclPartial<'a>) {
    self.declarations.insert(
      func_decl.name.clone(),
      ShaderFileDeclarationPartial::Func(func_decl)
    );
  }

  pub(crate) fn add_module_decl(&mut self, module_decl: ModuleDeclPartial<'a>) {
    self.declarations.insert(
      module_decl.name.clone(),
      ShaderFileDeclarationPartial::Module(module_decl)
    );
  }

  pub(crate) fn add_struct_decl(&mut self, struct_decl: StructDeclPartial<'a>) {
    self.declarations.insert(
      struct_decl.name.clone(),
      ShaderFileDeclarationPartial::Struct(struct_decl)
    );
  }
}

/**
 * A declaration in the partially instantiated shader file.
 */
#[derive(Debug, Clone)]
pub enum ShaderFileDeclarationPartial<'a> {
  Entrypoint(EntrypointDeclPartial<'a>),
  Buffer(BufferDeclPartial<'a>),
  Import(ImportDeclPartial<'a>),
  Instance(InstanceDeclPartial<'a>),
  Func(FuncDeclPartial<'a>),
  Module(ModuleDeclPartial<'a>),
  Struct(StructDeclPartial<'a>),
}
