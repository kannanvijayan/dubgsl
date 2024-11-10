use crate::model::{ Model, ModelHandle, StringModel };

/**
 * An internal representation of the shader file.
 */
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct ShaderFileModel {
    pub(crate) path: ModelHandle<StringModel>,
}
impl ShaderFileModel {
  pub(crate) fn new(path: ModelHandle<StringModel>) -> ShaderFileModel {
    ShaderFileModel { path }
  }
}
impl Model for ShaderFileModel {
}

pub type ShaderFileModelHandle = ModelHandle<ShaderFileModel>;
