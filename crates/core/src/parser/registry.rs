use std::collections::HashMap;
use std::path::Path;

use crate::parser::{event::FileEvents, python::PythonParser};

use super::r#trait::{LanguageParser};

pub struct LanguageParserRegistry {
    parsers: HashMap<String, Box<dyn LanguageParser>>,
    extension_to_language: HashMap<String, String>,
}

impl LanguageParserRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            parsers: HashMap::new(),
            extension_to_language: HashMap::new(),
        };
        
        // Register built-in parsers
        registry.register_parser(Box::new(PythonParser));
        
        registry
    }
    
    pub fn register_parser(&mut self, parser: Box<dyn LanguageParser>) {
        let language_name = parser.language_name().to_string();
        
        for &ext in parser.file_extensions() {
            self.extension_to_language.insert(ext.to_string(), language_name.clone());
        }
        
        self.parsers.insert(language_name, parser);
    }
    
    pub fn get_parser_for_file(&self, file_path: &Path) -> Option<&Box<dyn LanguageParser>> {
        let extension = file_path.extension()?.to_str()?;
        let language = self.extension_to_language.get(extension)?;
        self.parsers.get(language)
    }
    
    pub fn parse_file(&self, file_path: &Path, content: &str) -> Result<Option<FileEvents>, Box<dyn std::error::Error>> {
        if let Some(parser) = self.get_parser_for_file(file_path) {
            Ok(Some(parser.parse_file(content, file_path)?))
        } else {
            Ok(None)
        }
    }
}