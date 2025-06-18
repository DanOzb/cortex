use ignore::WalkBuilder;
use std::path::Path;

fn is_code_file(entry: &ignore::DirEntry) -> bool {
    if !entry.file_type().map(|t| t.is_file()).unwrap_or(false) {
        return false;
    }

    let allowed_extensions = [
        "rs", "js", "ts", "py", "java", "cpp", "c", "cs", "go", "rb",
        "html", "css", "json", "toml", "yaml", "yml", "sh", "php", "swift", "kt"
    ];

    entry.path()
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| allowed_extensions.contains(&ext))
        .unwrap_or(false)
}

fn walk_filtered_files(root: &Path) {
    let walker = WalkBuilder::new(root)
        .standard_filters(true) 
        .hidden(false)          
        .build();

    for result in walker {
        match result {
            Ok(entry) => {
                if is_code_file(&entry) {
                    println!("Code file: {}", entry.path().display());
                }
            }
            Err(err) => eprintln!("Error reading entry: {:?}", err),
        }
    }
}

fn main() {
    let path = std::env::args().nth(1).expect("Please provide a path");
    let root = Path::new(&path);
    if !root.exists() {
        eprintln!("Path does not exist: {}", path);
        return;
    }

    walk_filtered_files(root);
}