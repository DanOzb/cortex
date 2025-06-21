use ignore::gitignore::{Gitignore, GitignoreBuilder};
use std::path::Path;

pub struct IgnoreMatcher {
    matcher: Gitignore,
}

impl IgnoreMatcher {
    pub fn from_root_project<P: AsRef<Path>>(root: P, user_ignores: Vec<&str>) -> Self{
        let mut ignore_builder = GitignoreBuilder::new(root);

        let _ = ignore_builder.add(".gitignore");
        let _ = ignore_builder.add(".ignore");

        for file_name in user_ignores {
            let _ = ignore_builder.add_line(None, &file_name);
        }

        let matcher = ignore_builder.build().unwrap();
        Self {matcher}
    }

    pub fn is_ignored<P: AsRef<Path>>(&self, path: P) -> bool{
        self.matcher.matched(path, false).is_ignore()
    }
}