
#[derive(Debug)]
pub struct Context{
    // 这个类是抄 cargo中的Workspace的

   //    config: &'cfg Config,

    /// Workspace-level custom metadata
    custom_metadata: Option<toml::Value>,
}

impl Context{
    pub  fn default( ) -> Self {
        Self {
            custom_metadata: None,
        }
    }
}