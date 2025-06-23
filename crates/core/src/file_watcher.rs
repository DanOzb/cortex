use notify::{Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::time::Duration;
use std::sync::mpsc::{channel, Receiver};
use std::vec;

use crate::debouncer::Debouncer;
use crate::extension_filter::ExtensionFilter;
use crate::ignore_matcher::IgnoreMatcher;
use crate::index_decider:: IndexDecider;


pub struct FileIndexer {
    root_path: PathBuf,
    indexed_files: HashSet<PathBuf>,
    index_decider: IndexDecider,
}

impl FileIndexer {
    pub fn from_root_project<P: AsRef<Path>>(root: P) -> Self {
        let file_extensions = vec![
            "sh", "c", "cpp", "cc", "cxx", "h", "hpp", "css", "d", "ex", "exs", "erl", "hrl", "go", 
            "hs", "html", "htm", "java", "js", "mjs", "cjs", "json", "lua", "md", "markdown", "pl", "pm", "py", 
            "rb", "rs", "toml", "ts", "tsx", "jsx", "vim", "yaml", "yml"
            ];

        let matcher = IgnoreMatcher::from_root_project(&root, Vec::new()); 
        let filter = ExtensionFilter::new(file_extensions); 
        let debouncer = Debouncer::new(10, 0); 
        let decider = IndexDecider::new(matcher, filter, debouncer);

        Self {
            root_path: root.as_ref().to_path_buf(),
            indexed_files: HashSet::new(),
            index_decider: decider,
        }
    }

    fn index_file(&self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        println!("Indexing file: {}", path.display());
        
        if !path.exists() {
            println!("  - File no longer exists, skipping");
            return Ok(());
        }

        let _ = std::fs::read_to_string(path)?;

        //Later: Use Tree Sitter to parse file
        
        Ok(())
    }

    fn create_file(&mut self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        println!("File created: {}", path.display());
        
        if self.index_decider.should_index(path) {
            self.index_file(path)?;
        }
        
        Ok(())
    }

    fn delete_file(&self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        println!("File deleted: {}", path.display());
        
        // Later: remove logic here
        
        Ok(())
    }

    pub fn start_watching(&mut self) -> Result<(), Box<dyn std::error::Error>> {

        let root = &self.root_path.clone();
        
        self.initial_index(root)?;

        let (_watcher, rx) = self.setup_watcher()?;
        
        self.print_status();
        self.program_loop(&rx);

        Ok(())
    }

    fn initial_index(&mut self, root: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        println!("Starting initial indexing of: {}", self.root_path.display());
        
        self.walk_directory(root)?;
        
        println!("Initial indexing complete. Indexed {} files.", self.indexed_files.len());
        Ok(())
    }

    fn walk_directory(&mut self, dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
        if !dir.is_dir() {
            return Ok(());
        }

        let entries = std::fs::read_dir(dir)?;
        
        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() {
                if self.index_decider.should_index(&path) {
                    match self.index_file(&path) {
                        Ok(()) => {
                            let canonized_path = path.canonicalize()?;
                            self.indexed_files.insert(canonized_path.clone());
                            println!("Successfully indexed and tracked: {}", canonized_path.display());
                        }
                        Err(e) => {
                            eprintln!("Failed to index {}: {}", path.display(), e);
                        }
                    }
                }
            } else if path.is_dir() {
                self.walk_directory(&path)?;
            }
        }
              
        Ok(())   
    }

    fn setup_watcher(&self) -> Result<(RecommendedWatcher, Receiver<Result<Event, notify::Error>>), Box<dyn std::error::Error>> {
        let (tx, rx) = channel();
        
        let mut watcher = RecommendedWatcher::new(
            tx,
            Config::default().with_poll_interval(Duration::from_millis(100))
        )?;

        println!("Setting up recursive watch on: {}", self.root_path.display());
        watcher.watch(&self.root_path, RecursiveMode::Recursive)?;

        Ok((watcher, rx))
    }

    fn program_loop(&mut self, rx: &Receiver<Result<Event, notify::Error>>){
        loop {
            match rx.recv() {
                Ok(Ok(event)) => {
                    if let Err(e) = self.handle_event(event) {
                        eprintln!("Error handling event: {}", e);
                    }
                }
                Ok(Err(e)) => eprintln!("Watch error: {:?}", e),
                Err(e) => {
                    eprintln!("Channel error: {:?}", e);
                    break;
                }
            }
        }
    }
    
    fn print_status(&self){
        let file_count = self.indexed_files.len();
        
        if file_count > 0 {
            println!("File watcher started. Monitoring {} specific files.", file_count);
        }
    }

    fn handle_event(&mut self, event: Event) -> Result<(), Box<dyn std::error::Error>> {
        match event.kind {
                        EventKind::Modify(notify::event::ModifyKind::Name(_)) => {
                self.handle_file_rename(event);
            }
            EventKind::Modify(notify::event::ModifyKind::Data(_))
            |EventKind::Modify(notify::event::ModifyKind::Other)
            |EventKind::Modify(notify::event::ModifyKind::Any)  => {
                self.handle_file_modification(event);
            }
    
            EventKind::Create(_) => {
                self.handle_file_creation(event);
            }
            EventKind::Remove(_) => {
                self.handle_file_deletion(event);
            }

            _ => {
                println!("Unhandled event type {:?}", event.kind);
            }
        }
        Ok(())
    }
    
    fn handle_file_modification(&mut self, event: Event){
        for path in event.paths {
            let canonicolized_path = &path.canonicalize().unwrap();
            if self.indexed_files.contains(canonicolized_path) {
                if self.index_decider.should_index(canonicolized_path){
                    if let Err(e) = self.index_file(&canonicolized_path) {
                        eprintln!("Failed to index {}: {}", path.display(), e);
                    }
                } else {
                    println!("Debouncer time left {:?}", self.index_decider.debounce_duration_left(canonicolized_path))
                }            
            }      
        }   
    }
    
    fn handle_file_creation(&mut self, event: Event){
        for path in event.paths {
            if path.is_file() && self.index_decider.should_index(&path) {
                if let Err(e) = self.create_file(&path) {
                     eprintln!("Failed to handle creation of {}: {}", path.display(), e);
                }
            }
        }
    }

    fn handle_file_deletion(&mut self, event: Event){
        for path in event.paths {
            if let Err(e) = self.delete_file(&path) {
                    eprintln!("Failed to handle deletion of {}: {}", path.display(), e);
            }
        }
    }

    fn handle_file_rename(&mut self, event: Event){
        for path in event.paths {
            if path.exists() {
                if self.index_decider.should_index(&path) {
                    if let Err(e) = self.create_file(&path) {
                        eprintln!("Failed to handle rename/move to {}: {}", path.display(), e);
                    }
                }
            } else {
                if self.index_decider.should_index(&path) {
                    if let Err(e) = self.delete_file(&path) {
                        eprintln!("Failed to handle rename/move from {}: {}", path.display(), e);
                    }
                }
            }
        }
    }

}
