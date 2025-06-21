use std::collections::HashSet;
use std::path::Path;

pub struct ExtensionFilter {
    supported_extensions: HashSet<String>,
}

impl ExtensionFilter {
    pub fn new(extensions: Vec<&str>) -> Self {
        let set = extensions.into_iter().map(|s| s.to_string()).collect();
        Self { supported_extensions: set }
    }

    pub fn is_supported<P: AsRef<Path>>(&self, path: P) -> bool {
        path.as_ref()
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| self.supported_extensions.contains(ext))
            .unwrap_or(false)
    }
}
