use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum ParseEvent {
    // === Core Declarations ===
    FunctionDefinition {
        name: String,
        start_line: usize,
        end_line: usize,
        parameters: Vec<String>,
        return_type: Option<String>,
        is_public: bool,
    },
    
    ClassDefinition {
        name: String,
        start_line: usize,
        end_line: usize,
        fields: Vec<String>,
        is_public: bool,
    },
    
    VariableDefinition {
        name: String,
        var_type: Option<String>,
        line: usize,
        is_public: bool,
        is_constant: bool,
    },
    
    ImportStatement {
        module: String,
        items: Vec<String>, // empty for wildcard imports
        line: usize,
        is_wildcard: bool,
    },
    
    // === Control Flow ===
    ConditionalBlock {
        condition_type: String, // "if", "elif", "else", "match", "switch"
        condition_summary: Option<String>, // None for "else"
        start_line: usize,
        end_line: usize,
    },
    
    LoopBlock {
        loop_type: String, // "for", "while", "do_while", "loop"
        iterator_variable: Option<String>,
        iterable: Option<String>,
        start_line: usize,
        end_line: usize,
    },
    
    TryBlock {
        start_line: usize,
        end_line: usize,
        exception_types: Vec<String>,
        has_finally: bool,
    },
    
    // === Relationships ===
    FunctionCall {
        caller_function: Option<String>, // None if at module level
        callee: String,
        line: usize,
        arguments: Vec<String>, // simplified argument representations
    },
    
    VariableAccess {
        variable: String,
        access_type: AccessType,
        line: usize,
        context: Option<String>, // function/class where it's accessed
    },
    
    ClassInheritance {
        child_class: String,
        parent_classes: Vec<String>,
        line: usize,
    },
    
    // Python
    PythonDecorator {
        target: String, // function/class name
        decorator: String,
        line: usize,
    },
    
    PythonAsyncFunction {
        function_name: String,
        line: usize,
    },
    
    PythonContextManager {
        variable: Option<String>,
        context_expression: String,
        line: usize,
    },
    
    PythonListComprehension {
        result_expression: String,
        iterator_variable: String,
        iterable: String,
        line: usize,
    },
    
    // === Comments and Documentation ===
    DocComment {
        target: String, // what this documents
        content: String,
        line: usize,
        doc_type: DocType,
    },
    
    Comment {
        content: String,
        line: usize,
        comment_type: CommentType,
    },
}

#[derive(Debug, Clone)]
pub enum AccessType {
    Read,
    Write,
    ReadWrite,
}

#[derive(Debug, Clone)]
pub enum DocType {
    Function,
    Class,
    Module,
    Variable,
}

#[derive(Debug, Clone)]
pub enum CommentType {
    Line,
    Block,
    Todo,
    Fixme,
}

#[derive(Debug, Clone)]
pub struct FileEvents {
    pub file_path: PathBuf,
    pub events: Vec<ParseEvent>,
    pub language: String,
    pub last_modified: std::time::SystemTime,
    pub parse_timestamp: std::time::SystemTime,
}

impl FileEvents {
       pub fn new(file_path: PathBuf, language: String, last_modified: std::time::SystemTime) -> Self {
        Self {
            file_path,
            events: Vec::new(),
            language,
            last_modified,
            parse_timestamp: std::time::SystemTime::now(),
        }
    }
    
    pub fn add_event(&mut self, event: ParseEvent) {
        self.events.push(event);
    }
    
    // Convenience methods for querying events
    pub fn functions(&self) -> impl Iterator<Item = &ParseEvent> {
        self.events.iter().filter(|e| matches!(e, ParseEvent::FunctionDefinition { .. }))
    }
    
    pub fn classes(&self) -> impl Iterator<Item = &ParseEvent> {
        self.events.iter().filter(|e| matches!(e, ParseEvent::ClassDefinition { .. }))
    }
    
    pub fn imports(&self) -> impl Iterator<Item = &ParseEvent> {
        self.events.iter().filter(|e| matches!(e, ParseEvent::ImportStatement { .. }))
    }
    
    pub fn variables(&self) -> impl Iterator<Item = &ParseEvent> {
        self.events.iter().filter(|e| matches!(e, ParseEvent::VariableDefinition { .. }))
    }
    
    pub fn function_calls(&self) -> impl Iterator<Item = &ParseEvent> {
        self.events.iter().filter(|e| matches!(e, ParseEvent::FunctionCall { .. }))
    }
    
    pub fn events_by_line(&self, line: usize) -> impl Iterator<Item = &ParseEvent> {
        self.events.iter().filter(move |e| self.event_line(e) == Some(line))
    }
    
    pub fn events_in_range(&self, start_line: usize, end_line: usize) -> impl Iterator<Item = &ParseEvent> {
        self.events.iter().filter(move |e| {
            if let Some(line) = self.event_line(e) {
                line >= start_line && line <= end_line
            } else {
                false
            }
        })
    }
    
    pub fn event_count(&self) -> usize {
        self.events.len()
    }
    
    fn event_line(&self, event: &ParseEvent) -> Option<usize> {
        match event {
            ParseEvent::FunctionDefinition { start_line, .. } => Some(*start_line),
            ParseEvent::ClassDefinition { start_line, .. } => Some(*start_line),
            ParseEvent::VariableDefinition { line, .. } => Some(*line),
            ParseEvent::ImportStatement { line, .. } => Some(*line),
            ParseEvent::ConditionalBlock { start_line, .. } => Some(*start_line),
            ParseEvent::LoopBlock { start_line, .. } => Some(*start_line),
            ParseEvent::TryBlock { start_line, .. } => Some(*start_line),
            ParseEvent::FunctionCall { line, .. } => Some(*line),
            ParseEvent::VariableAccess { line, .. } => Some(*line),
            ParseEvent::ClassInheritance { line, .. } => Some(*line),
            ParseEvent::PythonDecorator { line, .. } => Some(*line),
            ParseEvent::PythonAsyncFunction { line, .. } => Some(*line),
            ParseEvent::PythonContextManager { line, .. } => Some(*line),
            ParseEvent::PythonListComprehension { line, .. } => Some(*line),
            ParseEvent::DocComment { line, .. } => Some(*line),
            ParseEvent::Comment { line, .. } => Some(*line),
        }
    }
}