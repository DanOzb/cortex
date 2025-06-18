use notify::{Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};
use std::sync::mpsc::channel;


pub struct FileIndexer {
    watched_files: HashSet<PathBuf>,
    watched_dirs: HashSet<PathBuf>,
    debounce_duration: Duration,
    last_index_time: Option<Instant>,
    code_extensions: HashSet<String>,
    code_files_only: bool,
}

impl FileIndexer {
    pub fn new(debounce_ms: u64) -> Self {
        let mut code_extensions = HashSet::new();
        // Common code file extensions
        for ext in [
            "rs", "py", "js", "ts", "jsx", "tsx", "go", "c", "cpp", "cc", "cxx", "h", "hpp",
            "java", "kt", "scala", "cs", "php", "rb", "swift", "dart", "r", "m", "mm",
            "sh", "bash", "zsh", "fish", "ps1", "bat", "cmd", "sql", "html", "htm", "xml",
            "css", "scss", "sass", "less", "json", "yaml", "yml", "toml", "ini", "cfg",
            "conf", "proto", "graphql", "gql", "vue", "svelte", "elm", "hs", "ml", "fs",
            "clj", "cljs", "ex", "exs", "erl", "pl", "pm", "t", "lua", "vim", "md", "tex"
        ] {
            code_extensions.insert(ext.to_string());
        }

        Self {
            watched_files: HashSet::new(),
            watched_dirs: HashSet::new(),
            debounce_duration: Duration::from_millis(debounce_ms),
            last_index_time: None,
            code_extensions,
            code_files_only: false,
        }
    }

    pub fn code_files_only(mut self) -> Self {
        self.code_files_only = true;
        self
    }

    pub fn add_extension(&mut self, ext: &str) {
        self.code_extensions.insert(ext.to_lowercase());
    }

    pub fn add_extensions<I>(&mut self, extensions: I) 
        where 
            I: IntoIterator<Item = String>,
        {
            for ext in extensions {
                self.code_extensions.insert(ext.to_lowercase());
            }
        }

    pub fn watch_directory<P: AsRef<Path>>(&mut self, path: P) {
        self.watched_dirs.insert(path.as_ref().to_path_buf());
    }

    pub fn watch_directories<I, P>(&mut self, paths: I) 
    where 
        I: IntoIterator<Item = P>,
        P: AsRef<Path>,
    {
        for path in paths {
            self.watch_directory(path);
        }
    }

    pub fn add_file<P: AsRef<Path>>(&mut self, path: P) {
        self.watched_files.insert(path.as_ref().to_path_buf());
    }

    pub fn add_files<I, P>(&mut self, paths: I) 
    where 
        I: IntoIterator<Item = P>,
        P: AsRef<Path>,
    {
        for path in paths {
            self.add_file(path);
        }
    }

    fn is_code_file(&self, path: &Path) -> bool {
        if !self.code_files_only {
            return true; 
        }

        path.extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| self.code_extensions.contains(&ext.to_lowercase()))
            .unwrap_or(false)
    }

    fn should_index(&mut self, path: &Path) -> bool {
        let is_watched = if !self.watched_files.is_empty() {
            self.watched_files.contains(path)
        } else if !self.watched_dirs.is_empty() {
            self.watched_dirs.iter().any(|dir| path.starts_with(dir))
        } else {
            false
        };

        if !is_watched {
            return false;
        }

        if !self.is_code_file(path) {
            return false;
        }

        let now = Instant::now();
        if let Some(last_time) = self.last_index_time {
            if now.duration_since(last_time) < self.debounce_duration {
                return false;
            }
        }

        self.last_index_time = Some(now);
        true
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

    fn handle_file_creation(&self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        println!("File created: {}", path.display());
        
        if self.is_code_file(path) {
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
        let (tx, rx) = channel();
        
        let mut watcher = RecommendedWatcher::new(
            tx,
            Config::default().with_poll_interval(Duration::from_millis(100))
        )?;

        let mut dirs_to_watch = self.watched_dirs.clone();
        
        for file_path in &self.watched_files {
            if let Some(parent) = file_path.parent() {
                dirs_to_watch.insert(parent.to_path_buf());
            }
        }

        if dirs_to_watch.is_empty() {
            return Err("No files or directories to watch".into());
        }

        for dir in &dirs_to_watch {
            println!("Watching directory: {}", dir.display());
            watcher.watch(dir, RecursiveMode::Recursive)?;
        }

        let file_count = self.watched_files.len();
        let dir_count = self.watched_dirs.len();
        
        if file_count > 0 {
            println!("File watcher started. Monitoring {} specific files.", file_count);
        }
        if dir_count > 0 {
            println!("File watcher started. Monitoring {} directories{}.", 
                dir_count, 
                if self.code_files_only { " (code files only)" } else { "" }
            );
        }
        
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

        Ok(())
    }

    fn handle_event(&mut self, event: Event) -> Result<(), Box<dyn std::error::Error>> {
        match event.kind {
            EventKind::Modify(notify::event::ModifyKind::Data(_)) => {
                for path in event.paths {
                    if self.should_index(&path) {
                        if let Err(e) = self.index_file(&path) {
                            eprintln!("Failed to index {}: {}", path.display(), e);
                        }
                    }
                }
            }
            EventKind::Create(_) => {
                for path in event.paths {
                    if path.is_file() && self.should_index(&path) {
                        if let Err(e) = self.handle_file_creation(&path) {
                            eprintln!("Failed to handle creation of {}: {}", path.display(), e);
                        }
                    }
                }
            }
            EventKind::Remove(_) => {
                for path in event.paths {
                    let would_be_indexed = if !self.watched_files.is_empty() {
                        self.watched_files.contains(&path)
                    } else if !self.watched_dirs.is_empty() {
                        self.watched_dirs.iter().any(|dir| path.starts_with(dir))
                    } else {
                        false
                    };

                    if would_be_indexed && self.is_code_file(&path) {
                        if let Err(e) = self.handle_file_deletion(&path) {
                            eprintln!("Failed to handle deletion of {}: {}", path.display(), e);
                        }
                    }
                }
            }
            EventKind::Modify(notify::event::ModifyKind::Name(_)) => {
                for path in event.paths {
                    if path.exists() {
                        if self.should_index(&path) {
                            if let Err(e) = self.handle_file_creation(&path) {
                                eprintln!("Failed to handle rename/move to {}: {}", path.display(), e);
                            }
                        }
                    } else {
                        let would_be_indexed = if !self.watched_files.is_empty() {
                            self.watched_files.contains(&path)
                        } else if !self.watched_dirs.is_empty() {
                            self.watched_dirs.iter().any(|dir| path.starts_with(dir))
                        } else {
                            false
                        };

                        if would_be_indexed && self.is_code_file(&path) {
                            if let Err(e) = self.handle_file_deletion(&path) {
                                eprintln!("Failed to handle rename/move from {}: {}", path.display(), e);
                            }
                        }
                    }
                }
            }
            _ => {
                // Ignore other events
            }
        }
        Ok(())
    }
}
