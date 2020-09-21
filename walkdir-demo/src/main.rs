use std::os::unix::fs::{MetadataExt, PermissionsExt};
use walkdir::WalkDir;

fn main() {
    let mut walker = WalkDir::new("/tmp").into_iter();
    while let Some(entry) = walker.next() {
        match entry {
            Ok(entry) => {
                let metadata = entry.metadata().unwrap();
                let owner = metadata.uid();
                let group = metadata.gid();
                let size = metadata.len();
                println!(
                    "path: {:?}\nowner: {}\ngroup: {}\nsize: {} bytes\npermission: {:o}\n======================================",
                    entry.path(),
                    owner,
                    group,
                    size,
                    metadata.permissions().mode()
                );
            }
            Err(e) => println!("Error: {:?}\n======================================", e),
        }
    }
}
