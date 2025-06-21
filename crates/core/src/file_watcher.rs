use notify::{Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::time::Duration;
use std::sync::mpsc::{channel, Receiver};
use std::vec;

use crate::debouncer::Debouncer;
use crate::extension_filter::ExtensionFilter;
use crate::ignore_matcher::IgnoreMatcher;
use crate::index_decider::{IndexDecider};


pub struct FileIndexer {
    watched_files: HashSet<PathBuf>,
    watched_dirs: HashSet<PathBuf>,
    index_decider: IndexDecider,
}

impl FileIndexer {
    pub fn from_root_project<P: AsRef<Path>>(root: P) -> Self {
        let file_extensions = vec![
            "sh", "c", "cpp", "cc", "cxx", "h", "hpp", "css", "d", "ex", "exs", "erl", "hrl", "go", 
            "hs", "html", "htm", "java", "js", "mjs", "cjs", "json", "lua", "md", "markdown", "pl", "pm", "py", 
            "rb", "rs", "toml", "ts", "tsx", "jsx", "vim", "yaml", "yml"
            ];

        let matcher = IgnoreMatcher::from_root_project(root, Vec::new()); 
        let filter = ExtensionFilter::new(file_extensions); 
        let debouncer = Debouncer::new(3, 0); 
        let decider = IndexDecider::new(matcher, filter, debouncer);

        Self {
            watched_files: HashSet::new(),
            watched_dirs: HashSet::new(),
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

    fn handle_file_creation(&mut self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        println!("File created: {}", path.display());
        
        if self.index_decider.should_index(path) {
            self.index_file(path)?;
        }
        
        Ok(())
    }

    fn handle_file_deletion(&self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        println!("File deleted: {}", path.display());
        
        // Later: remove logic here
        
        Ok(())
    }

    pub fn start_watching(&mut self) -> Result<(), Box<dyn std::error::Error>> {

        let dirs_to_watch = self.setup_dirs_to_watch();
        let (_watcher, rx) = self.setup_watcher(&dirs_to_watch)?;
    
        self.print_status();
        self.program_loop(&rx);

        Ok(())
    }

    fn setup_dirs_to_watch(&self) -> HashSet<PathBuf>{
        let mut dirs_to_watch = self.watched_dirs.clone();
        for file_path in &self.watched_files {
            if let Some(parent) = file_path.parent() {
                dirs_to_watch.insert(parent.to_path_buf());
            }
        }
       
        dirs_to_watch
    }

    fn setup_watcher(&self, dirs_to_watch: &HashSet<PathBuf>) -> Result<(RecommendedWatcher, Receiver<Result<Event, notify::Error>>), Box<dyn std::error::Error>> {
        let (tx, rx) = channel();
        
        let mut watcher = RecommendedWatcher::new(
            tx,
            Config::default().with_poll_interval(Duration::from_millis(100))
        )?;

        if dirs_to_watch.is_empty() {
            return Err("No files or directories to watch".into());
        }

        for dir in dirs_to_watch {
            println!("Watching directory: {}", dir.display());
            watcher.watch(dir, RecursiveMode::Recursive)?;
        }

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
        let file_count = self.watched_files.len();
        let dir_count = self.watched_dirs.len();
        
        if file_count > 0 {
            println!("File watcher started. Monitoring {} specific files.", file_count);
        }
        if dir_count > 0 {
            println!("File watcher started. Monitoring directories {}.", 
                dir_count
            );
        }
    }

    fn handle_event(&mut self, event: Event) -> Result<(), Box<dyn std::error::Error>> {
        match event.kind {
            EventKind::Modify(notify::event::ModifyKind::Data(_)) => {
                self.index_files(event);
            }
            EventKind::Create(_) => {
                self.create_files(event);
            }
            EventKind::Remove(_) => {
                self.remove_files(event);
            }
            EventKind::Modify(notify::event::ModifyKind::Name(_)) => {
                self.modify_files(event);
            }
            _ => {}
        }
        Ok(())
    }
    
    fn index_files(&mut self, event: Event){
        for path in event.paths {
            if self.index_decider.should_index(&path) {
                if let Err(e) = self.index_file(&path) {
                    eprintln!("Failed to index {}: {}", path.display(), e);
                }                
            }               
        }
    }

    fn create_files(&mut self, event: Event){
        for path in event.paths {
            if path.is_file() && self.index_decider.should_index(&path) {
                if let Err(e) = self.handle_file_creation(&path) {
                     eprintln!("Failed to handle creation of {}: {}", path.display(), e);
                }
            }
        }
    }

    fn remove_files(&mut self, event: Event){
        for path in event.paths {
            if let Err(e) = self.handle_file_deletion(&path) {
                    eprintln!("Failed to handle deletion of {}: {}", path.display(), e);
            }
        }
    }

    fn modify_files(&mut self, event: Event){
        for path in event.paths {
            if path.exists() {
                if self.index_decider.should_index(&path) {
                    if let Err(e) = self.handle_file_creation(&path) {
                        eprintln!("Failed to handle rename/move to {}: {}", path.display(), e);
                    }
                }
            } else {
                if self.index_decider.should_index(&path) {
                    if let Err(e) = self.handle_file_deletion(&path) {
                        eprintln!("Failed to handle rename/move from {}: {}", path.display(), e);
                    }
                }
            }
        }
    }

}
