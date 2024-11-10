use std::borrow::Borrow;

use crate::model::{ Model, ModelHandle };

/**
 * A shared name.
 */
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct NameModel {
    pub(crate) name: String
}
impl NameModel {
  pub(crate) fn new(name: String) -> NameModel {
    NameModel { name }
  }
}
impl Model for NameModel {
}

pub type NameModelHandle = ModelHandle<NameModel>;

impl Borrow<str> for NameModelHandle {
  fn borrow(&self) -> &str {
    self.0.name.as_str()
  }
}

/**
 * A shared name path.
 */
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct NamePathModel {
    pub(crate) path: Vec<NameModelHandle>
}
impl NamePathModel {
  pub(crate) fn new(path: Vec<NameModelHandle>) -> NamePathModel {
    NamePathModel { path }
  }
}
impl Model for NamePathModel {
}

pub type NamePathModelHandle = ModelHandle<NamePathModel>;

impl Borrow<[NameModelHandle]> for NamePathModelHandle {
  fn borrow(&self) -> &[NameModelHandle] {
    self.0.path.as_slice()
  }
}
