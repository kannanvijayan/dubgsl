use std::{
  fmt::Debug,
  hash::{ Hash, Hasher },
  sync::Arc,
  borrow::Borrow,
};
use crate::model::Model;

/**
 * A Handle to a model in a model space.
 */
pub struct ModelHandle<M: Model>(pub(crate) Arc<M>);
impl<M: Model> ModelHandle<M> {
  /**
   * Create a new model handle.
   */
  pub fn new(model: M) -> ModelHandle<M> {
    ModelHandle(Arc::new(model))
  }
}
impl<M: Model> Debug for ModelHandle<M> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "ModelHandle({:?})", self.0)
  }
}
impl<M: Model> Clone for ModelHandle<M> {
  fn clone(&self) -> ModelHandle<M> {
    ModelHandle(self.0.clone())
  }
}
impl<M: Model> Hash for ModelHandle<M> {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.0.hash(state)
  }
}
impl<M: Model> PartialEq for ModelHandle<M> {
  fn eq(&self, other: &Self) -> bool {
    self.0.as_ref() == other.0.as_ref()
  }
}
impl<M: Model> Eq for ModelHandle<M> {
}
impl<M: Model> Borrow<M> for ModelHandle<M> {
  fn borrow(&self) -> &M {
    self.0.as_ref()
  }
}
