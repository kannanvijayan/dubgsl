use std::marker::PhantomData;
use crate::{
  model::{ EntrypointDims, NameModelHandle },
  syntax::{
    declaration::{
      InstanceDecl,
      ModuleDecl,
      StructDecl,
      UniformsDecl,
    },
    name::NamePath,
  },
  transform::syntax_ingester::TypeRefPartial,
};

#[derive(Debug, Clone)]
pub struct EntrypointDeclPartial<'a> {
  pub(crate) name: NameModelHandle,
  pub(crate) dims: EntrypointDims,
  pub(crate) arg_name: NameModelHandle,
  pub(crate) body: Vec<StatementBodyPartial<'a>>,
}

#[derive(Debug, Clone)]
pub struct BufferDeclPartial<'a> {
  pub(crate) name: NameModelHandle,
  pub(crate) ty: TypeRefPartial<'a>,
}

#[derive(Debug, Clone)]
pub struct ImportDeclPartial<'a> {
  pub(crate) name: NameModelHandle,
  pub(crate) path: NamePath<'a>,
}

#[derive(Debug, Clone)]
pub struct InstanceDeclPartial<'a> {
  pub(crate) name: NameModelHandle,
  pub(crate) syntax_decl: InstanceDecl<'a>,
}

#[derive(Debug, Clone)]
pub struct FuncDeclPartial<'a> {
  pub(crate) name: NameModelHandle,
  pub(crate) return_ty: Option<TypeRefPartial<'a>>,
  pub(crate) args: Vec<FuncDeclArgPartial<'a>>,
  pub(crate) body: Vec<StatementBodyPartial<'a>>,
}

#[derive(Debug, Clone)]
pub struct ModuleDeclPartial<'a> {
  pub(crate) name: NameModelHandle,
  pub(crate) syntax_decl: ModuleDecl<'a>,
}

#[derive(Debug, Clone)]
pub struct StructDeclPartial<'a> {
  pub(crate) name: NameModelHandle,
  pub(crate) syntax_decl: StructDecl<'a>,
}

#[derive(Debug, Clone)]
pub struct UniformsDeclPartial<'a> {
  pub(crate) syntax_decl: UniformsDecl<'a>,
}

#[derive(Debug, Clone)]
pub struct FuncDeclArgPartial<'a> {
  pub(crate) name: NameModelHandle,
  pub(crate) ty: TypeRefPartial<'a>,
}

#[derive(Debug, Clone)]
pub struct StatementBodyPartial<'a> {
  _phantom: PhantomData<&'a ()>
}
