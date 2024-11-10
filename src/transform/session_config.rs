use std::path::PathBuf;

/**
 * The configuration across the entire session of a compile (all transforms).
 */
#[derive(Clone)]
pub struct SessionConfig {
  /** The project root directory. */
  pub project_root: PathBuf,
}


/**
 * Builder for a session configuration.
 */
pub struct SessionConfigBuilder {
  project_root: Option<PathBuf>,
}
impl SessionConfigBuilder {
  /**
   * Create a new session configuration builder.
   */
  pub fn new() -> Self {
    SessionConfigBuilder { project_root: None }
  }

  /**
   * Set the project root directory.
   */
  pub fn project_root(mut self, project_root: PathBuf) -> Self {
    self.project_root = Some(project_root);
    self
  }

  /**
   * Build the session configuration.
   */
  pub fn build(self) -> SessionConfig {
    SessionConfig {
      project_root: self.project_root.expect("project_root is required"),
    }
  }
}
