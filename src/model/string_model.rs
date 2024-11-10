use std::borrow::Borrow;

use crate::model::{ Model, ModelHandle };

/**
 * An internal representation of the shader file.
 */
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct StringModel {
    pub(crate) name: String
}
impl StringModel {
  pub(crate) fn new(name: String) -> StringModel {
    StringModel { name }
  }
}
impl Model for StringModel {
}

pub type StringModelHandle = ModelHandle<StringModel>;

impl Borrow<str> for StringModelHandle {
  fn borrow(&self) -> &str {
    self.0.name.as_str()
  }
}
