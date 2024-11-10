use std::collections::{ HashMap, HashSet };

use crate::model::{
  ModelHandle,
  ShaderFileModel,
  ShaderFileModelHandle,
  StringModel,
  StringModelHandle,
  NameModel,
  NameModelHandle,
  NamePathModel,
  NamePathModelHandle,
  TypeModel,
  TypeModelHandle,
};

/**
 * A model space holds all the models in compile session and provides
 * for a shareability domain.
 */
#[derive(Clone)]
pub struct ModelSpace {
  // Interned string models.
  strings: HashSet<StringModelHandle>,

  // Interned name models.
  names: HashSet<NameModelHandle>,

  // Interned name paths.
  name_paths: HashSet<NamePathModelHandle>,

  // Interned type models.
  type_models: HashSet<TypeModelHandle>,

  // Shader file models, indexed by path.
  shader_file_models_by_path: HashMap<StringModelHandle, ShaderFileModelHandle>,
}
impl ModelSpace {
  /**
   * Create a new model space.
   */
  pub(crate) fn new() -> ModelSpace {
    ModelSpace {
      strings: HashSet::new(),
      names: HashSet::new(),
      name_paths: HashSet::new(),
      type_models: HashSet::new(),
      shader_file_models_by_path: HashMap::new(),
    }
  }

  /**
   * Create a handle to shared string model.
   */
  pub(crate) fn intern_string(&mut self, string: &str) -> StringModelHandle {
    // If string exists, return the handle.
    if let Some(handle) = self.strings.get(string) {
      return handle.clone();
    }
    let model = StringModel::new(string.to_string());
    let handle =  ModelHandle::new(model);
    self.strings.insert(handle.clone());
    handle
  }

  /**
   * Create a handle to shared string model.
   */
  pub(crate) fn intern_name(&mut self, string: &str) -> NameModelHandle {
    // If string exists, return the handle.
    if let Some(handle) = self.names.get(string) {
      return handle.clone();
    }
    let model = NameModel::new(string.to_string());
    let handle =  ModelHandle::new(model);
    self.names.insert(handle.clone());
    handle
  }

  /**
   * Create a handle to shared name path model.
   */
  pub(crate) fn intern_name_path<NS>(&mut self,
    path: NS,
  ) -> NamePathModelHandle
    where NS: AsRef<[NameModelHandle]> + Into<Vec<NameModelHandle>>
  {
    // If path exists, return the handle.
    if let Some(handle) = self.name_paths.get(path.as_ref()) {
      return handle.clone();
    }
    let model = NamePathModel::new(path.into());
    let handle =  ModelHandle::new(model);
    self.name_paths.insert(handle.clone());
    handle
  }

  /**
   * Create a handle to shared type model.
   */
  pub(crate) fn intern_type(&mut self, model: TypeModel) -> TypeModelHandle {
    // If type exists, return the handle.
    if let Some(handle) = self.type_models.get(&model) {
      return handle.clone();
    }
    let handle =  ModelHandle::new(model);
    self.type_models.insert(handle.clone());
    handle
  }

  /**
   * Check if a shader file model exists.
   */
  pub(crate) fn has_shader_file_model(&self, path: &str) -> bool {
    self.shader_file_models_by_path.contains_key(path)
  }

  /**
   * Add a shader file model to the model space.
   */
  pub(crate) fn add_shader_file_model(&mut self, model: ShaderFileModel)
    -> ShaderFileModelHandle
  {
    let path = model.path.clone();
    if let Some(handle) = self.shader_file_models_by_path.get(&path) {
      panic!("Shader file model already exists: {:?}", handle);
    }
    let handle = ModelHandle::new(model);
    self.shader_file_models_by_path.insert(path, handle.clone());
    handle
  }
}
