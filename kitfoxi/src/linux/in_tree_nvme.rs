use std::fs::{self, DirEntry};
use std::io;
use std::path::{Path, PathBuf};

// fn device_paths(dir: &Path, sig: &str) -> io::Result<Vec<PathBuf>> {
//     let mut res: Vec<PathBuf> = Vec::new();
//     if dir.is_dir() {
//         for entry in fs::read_dir(dir)? {
//             let entry = entry?;

//             let path = entry.path();
//             if path.is_dir() {
//                 println!("Checking subpath: {}", path.to_str().unwrap());

//                 let subpaths = device_paths(&path, sig)?;
//                 res.extend(subpaths);
//             } else {
//                 if let Some(sp) = path.to_str() {
//                     if sp.contains(sig) {
//                         println!("Pushing match: {}", path.to_str().unwrap());
//                         res.push(path);
//                     }
//                 }
//             }
//         }
//     }

//     Ok(res)
// }

pub fn find_in_tree_nvme_devices() -> Vec<InTreeNvmeDevice> {
    let mut res: Vec<InTreeNvmeDevice> = Vec::new();

    if let Ok(entries) = fs::read_dir(Path::new("/dev")) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if let Some(sp) = path.to_str() {
                    if sp.contains("nvme") {
                        res.push(InTreeNvmeDevice { path: path });
                    }
                }
            }
        }
    }

    res
}

#[derive(Debug)]
pub struct InTreeNvmeDevice {
    pub path: PathBuf,
}
