use tree_sitter::{Language, Node, TreeCursor};
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
        let should_parse_children: bool = self.parse_node(node, source_code, file_events)?; 

        if should_parse_children {
            let mut cursor = node.walk(); 
            for child in node.children(&mut cursor) {
                self.walk_tree(&child, source_code, file_events)?; 
            }
        }

        Ok(())
    }
}

impl PythonParser {
    fn parse_node(&self, node: &Node, source_code: &str, file_events: &mut FileEvents) -> Result<bool, Box<dyn std::error::Error>> {
        match node.kind() {
            "function_definition" => {
                if let Some(function_event) = self.parse_function(node, source_code)? {
                    file_events.add_event(function_event);
                }

                if let Some(body) = node.child_by_field_name("body") {
                    self.walk_tree(&body, source_code, file_events)?;
                }

                Ok(false)
            }
            /* 
            "class_definition" => {
                if let Some(class_event) = self.parse_class(node, source_code)? {
                    file_events.add_event(class_event);
                }
                Ok(false)
            }
            
            "assignment" => {
                if let Some(variable_event) = self.parse_variable(node, source_code)? {
                    file_events.add_event(variable_event);
                }
                Ok(false)
            }
            "import_statement" | "import_from_statement" => {
                if let Some(import_event) = self.parse_import(node, source_code)? {
                    file_events.add_event(import_event);
                }
                Ok(false)
            }
            "if_statement" => {
                if let Some(conditional_block_event) = self.parse_if_statement(node, source_code)?{
                    file_events.add_event(conditional_block_event);
                }
                Ok(false)
            }
            "match_statement" => {
                if let Some(conditional_block_event) = self.parse_match_statement(node, source_code)?{
                    file_events.add_event(conditional_block_event);
                }
                Ok(false)
            }
            "try_statement" => {
                if let Some(conditional_block_event) = self.parse_try_statement(node, source_code)?{
                    file_events.add_event(conditional_block_event);
                }
                Ok(false)
            }
            "while_statement" => {
                if let Some(control_flow_event) = self.parse_while_statement(node, source_code)?{
                    file_events.add_event(control_flow_event);
                }
                Ok(false)
            }
            "for_statement" => {
                if let Some(control_flow_event) = self.parse_for_statement(node, source_code)?{
                    file_events.add_event(control_flow_event);
                }
                Ok(false)
            }
            "parameter" => {
                if let Some(parameter_event) = self.parse_parameter(node, source_code)?{
                    file_events.add_event(parameter_event);
                }
                Ok(false)
            }
            "decorator" => {
                if let Some(decorator_event) = self.parse_decorator(node, source_code)?{
                    file_events.add_event(decorator_event);
                }
                Ok(false)
            }
            "block" => {
                if let Some(block_event) = self.parse_block(node, source_code)?{
                    file_events.add_event(block_event);
                }
                Ok(false)
            }
            "dotted_name" => {
                if let Some(dotted_name_event) = self.parse_dotted_name(node, source_code)?{
                    file_events.add_event(dotted_name_event);
                }
                Ok(false)
            }
            "expression_statement" => {
                if let Some(event) = self.parse_expression_statement(node, source_code)?{
                    file_events.add_event(event);
                }
                Ok(false)
            }
            "identifier" => {
                if let Some(event) = self.parse_identifier(node, source_code)?{
                    file_events.add_event(event);
                }
                Ok(false)
            }
            "argument_list" => {
                if let Some(event) = self.parse_argument_list(node, source_code)?{
                    file_events.add_event(event);
                }
                Ok(false)
            }
            "list" => {
                if let Some(event) = self.parse_list(node, source_code)?{
                    file_events.add_event(event);
                }
                Ok(false)
            }
            "tuple" => {
                if let Some(event) = self.parse_tuple(node, source_code)?{
                    file_events.add_event(event);
                }
                Ok(false)
            }
            "return_type" => {
                if let Some(event) = self.parse_return_type(node, source_code)?{
                    file_events.add_event(event);
                }
                Ok(false)
            }
            */
            _ => {Ok(true)}
        }
    }
    fn parse_function(&self, node: &Node, source_code: &str) -> Result<Option<ParseEvent>, Box<dyn std::error::Error>> {
        let name: String = node.child_by_field_name("name").map(|n: Node<'_>| self.node_text(n.clone(), source_code).to_string()).unwrap();
        let parameters: Vec<String> = if let Some(params_node) = node.child_by_field_name("parameters") {
            self.extract_parameters(&params_node, source_code)?
        } else {
            Vec::new()
        };

        let return_type: Option<String> = node.child_by_field_name("return_type").map(|n: Node<'_>| self.node_text(n.clone(), source_code).to_string());

        let start_line: usize = node.start_position().row + 1;
        let end_line: usize = node.end_position().row + 1;

        let is_public: bool = !name.starts_with('_');

        Ok(Some(ParseEvent::FunctionDefinition {
            name,
            start_line,
            end_line,
            parameters,
            return_type,
            is_public,
        }))
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

    //Helper functions

    fn extract_parameters(&self, params_node: &Node, source_code: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut parameters: Vec<String> = Vec::new();
        let mut cursor: TreeCursor = params_node.walk();
        
        for child in params_node.children(&mut cursor) {
            match child.kind() {
                "identifier" => {
                    parameters.push(self.node_text(child, source_code).to_string());
                }
                "typed_parameter" => {
                    if let Some(name_node) = child.child_by_field_name("name") {
                        let param_name: &str = self.node_text(name_node, source_code);
                        let param_type: String = child.child_by_field_name("type").map(|n: Node<'_>| format!(": {}", self.node_text(n, source_code))).unwrap();
                        parameters.push(format!("{}{}", param_name, param_type));
                    }
                }
                "default_parameter" => {
                    if let Some(name_node) = child.child_by_field_name("name") {
                        let param_name: &str = self.node_text(name_node, source_code);
                        let default_value: String = child.child_by_field_name("value").map(|v| format!(" = {}", self.node_text(v, source_code))).unwrap();
                        parameters.push(format!("{}{}", param_name, default_value));
                    }
                }
                "typed_default_parameter" => {
                    if let Some(name_node) = child.child_by_field_name("name") {
                        let param_name: &str = self.node_text(name_node, source_code);
                        let param_type: String = child.child_by_field_name("type").map(|n: Node<'_>| format!(": {}", self.node_text(n, source_code))).unwrap();
                        let default_value: String = child.child_by_field_name("value").map(|n: Node<'_>| format!(" = {}", self.node_text(n, source_code))).unwrap();
                        parameters.push(format!("{}{}{}", param_name, param_type, default_value));
                    }
                }
                _ => {}
            }
        }
        
        Ok(parameters)
    }
}