use std::hash::{ Hash, Hasher };
use crate::model::{ Model, ModelHandle, NameModel, NamePathModel, VecDims };

/**
 * Type model representation.
 */
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TypeModel {
  Scalar(ScalarTypeModel),
  Vector(VectorTypeModel),
  Struct(StructTypeModel),
}
impl TypeModel {
  pub fn new_i32() -> TypeModel {
    TypeModel::Scalar(ScalarTypeModel::Numeric(ScalarNumericTypeModel::I32))
  }
  pub fn new_u32() -> TypeModel {
    TypeModel::Scalar(ScalarTypeModel::Numeric(ScalarNumericTypeModel::U32))
  }
  pub fn new_f32() -> TypeModel {
    TypeModel::Scalar(ScalarTypeModel::Numeric(ScalarNumericTypeModel::F32))
  }
  pub fn new_bool() -> TypeModel {
    TypeModel::Scalar(ScalarTypeModel::Symbolic(ScalarSymbolicTypeModel::Bool))
  }
  pub fn new_void() -> TypeModel {
    TypeModel::Scalar(ScalarTypeModel::Symbolic(ScalarSymbolicTypeModel::Void))
  }
  pub fn new_vector(scalar: ScalarNumericTypeModel, dims: VecDims) -> TypeModel {
    TypeModel::Vector(VectorTypeModel { scalar, dims })
  }
}
impl Model for TypeModel {}

/**
 * Scalar
 */
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ScalarTypeModel {
  Symbolic(ScalarSymbolicTypeModel),
  Numeric(ScalarNumericTypeModel),
}

/**
 * Scalar symbolic types.
 */
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ScalarSymbolicTypeModel { Bool, Void }

/**
 * A scalar type model.
 */
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ScalarNumericTypeModel { I32, U32, F32 }

/**
 * A vector type model.
 */
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VectorTypeModel {
  pub(crate) scalar: ScalarNumericTypeModel,
  pub(crate) dims: VecDims,
}

/**
 * A struct type model.
 */
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructTypeModel {
  pub(crate) name: NamePathModel,
  pub(crate) fields: Vec<StructFieldModel>,
}
impl Hash for StructTypeModel {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.name.hash(state);
  }
}

/**
 * A struct field model.
 */
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StructFieldModel {
  pub(crate) name: NameModel,
  pub(crate) ty: TypeModelHandle,
}

pub type TypeModelHandle = ModelHandle<TypeModel>;
