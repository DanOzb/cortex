use tree_sitter::{Language, Parser, Node};
use std::path::{Path};

use crate::parser::event::{FileEvents};


pub trait LanguageParser {
    fn language(&self) -> Language;
    fn language_name(&self) -> &'static str;
    fn file_extensions(&self) -> &[&'static str];
    
    fn parse_file(&self, content: &str, file_path: &Path) -> Result<FileEvents, Box<dyn std::error::Error>> {
        let mut parser = Parser::new();
        parser.set_language(self.language())?;
        
        let tree = parser.parse(content, None)
            .ok_or("Failed to parse file")?;
        
        let metadata = std::fs::metadata(file_path)?;
        let last_modified = metadata.modified()?;
        
        let mut file_events = FileEvents::new(
            file_path.to_path_buf(),
            self.language_name().to_string(),
            last_modified,
        );
        
        self.walk_tree(&tree.root_node(), content, &mut file_events)?;
        Ok(file_events)
    }
    
    fn walk_tree(&self, node: &Node, source_code: &str, file_events: &mut FileEvents) -> Result<(), Box<dyn std::error::Error>>;
    
    fn node_text<'a>(&self, node: Node, source_code: &'a str) -> &'a str {
        &source_code[node.byte_range()]
    }
}