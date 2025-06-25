use crate::file_watcher::FileIndexer;

/* 
    Coming soon
*/
mod file_watcher;
mod extension_filter;
mod ignore_matcher;
mod index_decider;
mod debouncer;
mod parser;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    //simple check if it works
    let mut indexer = FileIndexer::from_root_project(r"C:\Users\eneso\Desktop\ripgrep");
    indexer.start_watching() 
}   