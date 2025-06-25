use tree_sitter::{Language, Node};
use tree_sitter_python::language as python_language;

use crate::parser::{event::{FileEvents, ParseEvent}, r#trait::LanguageParser};

pub struct PythonParser;

impl LanguageParser for PythonParser {
    fn language(&self) -> Language {
        python_language()
    }

    fn language_name(&self) -> &'static str {
        "python"
    }

    fn file_extensions(&self) -> &[&'static str] {
         &["py", "pyw", "pyi"]
    }

    fn walk_tree(&self, node: &Node, source_code: &str, file_events: &mut FileEvents) -> Result<(), Box<dyn std::error::Error>> {
        self.parse_node(node, source_code, file_events)?; 

        let mut cursor = node.walk(); 
        for child in node.children(&mut cursor) {
            self.walk_tree(&child, source_code, file_events)?; 
        }

        Ok(())
    }
}

impl PythonParser {

    fn parse_node(&self, node: &Node, source_code: &str, file_events: &mut FileEvents) -> Result<(), Box<dyn std::error::Error>> {
        match node.kind() {
            "function_definition" => {
                if let Some(function_event) = self.parse_function(node, source_code)? {
                    file_events.add_event(function_event);
                }
            }
            "class_definition" => {
                if let Some(class_event) = self.parse_class(node, source_code)? {
                    file_events.add_event(class_event);
                }
            }
            "assignment" => {
                if let Some(variable_event) = self.parse_variable(node, source_code)? {
                    file_events.add_event(variable_event);
                }
            }
            "import_statement" | "import_from_statement" => {
                if let Some(import_event) = self.parse_import(node, source_code)? {
                    file_events.add_event(import_event);
                }
            }
            "if_statement" => {
                if let Some(conditional_block_event) = self.parse_if_statement(node, source_code)?{
                    file_events.add_event(conditional_block_event);
                }
            }
            "match_statement" => {
                if let Some(conditional_block_event) = self.parse_match_statement(node, source_code)?{
                    file_events.add_event(conditional_block_event);
                }
            }
            "try_statement" => {
                if let Some(conditional_block_event) = self.parse_try_statement(node, source_code)?{
                    file_events.add_event(conditional_block_event);
                }
            }
            "while_statement" => {
                if let Some(control_flow_event) = self.parse_while_statement(node, source_code)?{
                    file_events.add_event(control_flow_event);
                }
            }
            "for_statement" => {
                if let Some(control_flow_event) = self.parse_for_statement(node, source_code)?{
                    file_events.add_event(control_flow_event);
                }
            }
            "parameter" => {
                if let Some(parameter_event) = self.parse_parameter(node, source_code)?{
                    file_events.add_event(parameter_event);
                }
            }
            "decorator" => {
                if let Some(decorator_event) = self.parse_decorator(node, source_code)?{
                    file_events.add_event(decorator_event);
                }
            }
            "block" => {
                if let Some(block_event) = self.parse_block(node, source_code)?{
                    file_events.add_event(block_event);
                }
            }
            "dotted_name" => {
                if let Some(dotted_name_event) = self.parse_dotted_name(node, source_code)?{
                    file_events.add_event(dotted_name_event);
                }
            }
            "expression_statement" => {
                if let Some(event) = self.parse_expression_statement(node, source_code)?{
                    file_events.add_event(event);
                }
            }
            "identifier" => {
                if let Some(event) = self.parse_identifier(node, source_code)?{
                    file_events.add_event(event);
                }
            }
            "argument_list" => {
                if let Some(event) = self.parse_argument_list(node, source_code)?{
                    file_events.add_event(event);
                }
            }
            "list" => {
                if let Some(event) = self.parse_list(node, source_code)?{
                    file_events.add_event(event);
                }
            }
            "tuple" => {
                if let Some(event) = self.parse_tuple(node, source_code)?{
                    file_events.add_event(event);
                }
            }
            "return_type" => {
                if let Some(event) = self.parse_return_type(node, source_code)?{
                    file_events.add_event(event);
                }
            }
            _ => {}
        }
        Ok(())
    }
    fn parse_function(&self, node: &Node, source_code: &str) -> Result<Option<ParseEvent>, Box<dyn std::error::Error>> {
        todo!()
    }

    fn parse_class(&self, node: &Node, source_code: &str) -> Result<Option<ParseEvent>, Box<dyn std::error::Error>> {
        todo!()
    }

    fn parse_variable(&self, node: &Node, source_code: &str) -> Result<Option<ParseEvent>, Box<dyn std::error::Error>> {
        todo!()
    }

    fn parse_import(&self, node: &Node, source_code: &str) -> Result<Option<ParseEvent>, Box<dyn std::error::Error>> {
        todo!()
    }

    fn parse_if_statement(&self, node: &Node, source_code: &str) -> Result<Option<ParseEvent>, Box<dyn std::error::Error>> {
        todo!()
    }

    fn parse_match_statement(&self, node: &Node, source_code: &str) -> Result<Option<ParseEvent>, Box<dyn std::error::Error>> {
        todo!()
    }

    fn parse_try_statement(&self, node: &Node, source_code: &str) -> Result<Option<ParseEvent>, Box<dyn std::error::Error>> {
        todo!()
    }

    fn parse_while_statement(&self, node: &Node, source_code: &str) -> Result<Option<ParseEvent>, Box<dyn std::error::Error>> {
        todo!()
    }

    fn parse_for_statement(&self, node: &Node, source_code: &str) -> Result<Option<ParseEvent>, Box<dyn std::error::Error>> {
        todo!()
    }

    fn parse_block(&self, node: &Node, source_code: &str) -> Result<Option<ParseEvent>, Box<dyn std::error::Error>> {
        todo!()
    }

    fn parse_parameter(&self, node: &Node, source_code: &str) -> Result<Option<ParseEvent>, Box<dyn std::error::Error>> {
        todo!()
    }

    fn parse_decorator(&self, node: &Node, source_code: &str) -> Result<Option<ParseEvent>, Box<dyn std::error::Error>> {
        todo!()
    }

    fn parse_dotted_name(&self, node: &Node, source_code: &str) -> Result<Option<ParseEvent>, Box<dyn std::error::Error>> {
        todo!()
    }

    fn parse_expression_statement(&self, node: &Node, source_code: &str) -> Result<Option<ParseEvent>, Box<dyn std::error::Error>> {
        todo!()
    }

    fn parse_identifier(&self, node: &Node, source_code: &str) -> Result<Option<ParseEvent>, Box<dyn std::error::Error>> {
        todo!()
    }

    fn parse_argument_list(&self, node: &Node, source_code: &str) -> Result<Option<ParseEvent>, Box<dyn std::error::Error>> {
        todo!()
    }

    fn parse_list(&self, node: &Node, source_code: &str) -> Result<Option<ParseEvent>, Box<dyn std::error::Error>> {
        todo!()
    }

    fn parse_tuple(&self, node: &Node, source_code: &str) -> Result<Option<ParseEvent>, Box<dyn std::error::Error>> {
        todo!()
    }

    fn parse_return_type(&self, node: &Node, source_code: &str) -> Result<Option<ParseEvent>, Box<dyn std::error::Error>> {
        todo!()
    }
}