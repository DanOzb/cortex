use crate::file_watcher::FileIndexer;

/* 
    Coming soon
*/
mod file_watcher;
mod extension_filter;
mod ignore_matcher;
mod index_decider;
mod debouncer;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    //simple check if it works
    let mut indexer = FileIndexer::from_root_project(r".");
    indexer.start_watching() 
}   