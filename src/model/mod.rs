mod dims;
mod model_handle;
mod model_space;
mod name_model;
mod shader_file_model;
mod string_model;
mod type_model;

pub use self::{
  dims::{ EntrypointDims, VecDims },
  model_handle::ModelHandle,
  model_space::ModelSpace,
  name_model::{
    NameModel,
    NameModelHandle,
    NamePathModel,
    NamePathModelHandle,
  },
  shader_file_model::{ ShaderFileModel, ShaderFileModelHandle },
  string_model::{ StringModel, StringModelHandle },
  type_model::{
    TypeModel,
    ScalarTypeModel,
    ScalarNumericTypeModel,
    ScalarSymbolicTypeModel,
    VectorTypeModel,
    StructTypeModel,
    StructFieldModel,
    TypeModelHandle,
  },
};

use std::{
  fmt::Debug,
  hash::Hash,
};

/**
 * Model instances are static objects that are cloneable and shared
 * when clone.  They act as value types.
 */
pub trait Model: 'static + Sized + Clone + Eq + PartialEq + Debug + Hash {
}
