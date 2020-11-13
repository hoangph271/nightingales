use ring::digest;
use std::fs;
use std::io::*;
use std::path::{Path, MAIN_SEPARATOR};
use std::time::SystemTime;

const SEPERATOR: &str = "/";

#[derive(Debug)]
pub struct LocalSyncRecord {
    byte_length: u64,
    modified: u64,
    is_file: bool,
    hash: String,
}

#[derive(Debug)]
pub struct OnlineFSItem {
    fs_item: FSItem,
}
impl OnlineFSItem {
    fn key(&self, fs_root: &str) -> Result<String> {
        let fs_root = Path::new(fs_root).canonicalize()?;
        let fs_root = fs_root.to_str().unwrap();

        let suffix = self.fs_item.path.strip_prefix(&fs_root).ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Path NOT exists: {}", self.fs_item.path),
            )
        })?;

        Ok(String::from(
            Path::new(suffix)
                .iter()
                .map(|os_str| os_str.to_str().unwrap())
                .filter(|part| part.to_string() != MAIN_SEPARATOR.to_string())
                .collect::<Vec<_>>()
                .join(SEPERATOR),
        ))
    }
}

#[derive(Debug)]
pub struct FSItem {
    is_file: bool,
    byte_length: u64,
    modified: u64,
    path: String,
}

impl FSItem {
    pub fn hash(&self) -> Result<String> {
        let content = &fs::read(&self.path)?;
        let actual = digest::digest(&digest::SHA256, content);

        Ok(hex::encode(actual.as_ref()))
    }

    pub fn from_path(path: &str) -> Result<FSItem> {
        let metadata = fs::metadata(Path::new(path))?;

        let is_file = metadata.is_file();
        let byte_length = metadata.len();

        match fs::canonicalize(path)?.to_str() {
            None => Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Path NOT exists: {}", path),
            )),
            Some(path) => {
                let modified = metadata
                    .modified()?
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .expect("Error getting modified")
                    .as_millis() as u64;

                Ok(FSItem {
                    is_file,
                    byte_length,
                    path: String::from(path),
                    modified,
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn return_correct_online_fs_item_key() {
        let path = Path::new("src").join("test").join("sub").join("item");
        let file_path = path.to_str().unwrap();

        let fs_item = FSItem::from_path(file_path).unwrap();
        let online_fs_item = OnlineFSItem { fs_item };

        let fs_root = Path::new("src").join("test");
        let fs_root = fs_root.to_str().unwrap();

        match online_fs_item.key(fs_root) {
            Ok(key) => assert_eq!(key, "sub/item"),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn reject_not_exists_file() {
        let path = Path::new("src").join("test").join("null");
        let file_path = path.to_str().unwrap();

        match FSItem::from_path(file_path) {
            Err(e) => match e.kind() {
                ErrorKind::NotFound => assert!(true),
                _ => assert!(false),
            },
            _ => assert!(false),
        }
    }

    #[test]
    fn hash_not_empty_file_works() {
        let path = Path::new("src").join("test").join("rust.txt");
        let file_path = path.to_str().unwrap();

        let fs_item = FSItem::from_path(file_path).unwrap();
        let hash = fs_item.hash().unwrap();
        assert_eq!(
            hash,
            "87ee07307593493b96730dcbb36fd51e9fa4ba189696dad60758e89e6e7750bf"
        );
    }

    #[test]
    fn hash_empty_file_works() {
        let path = Path::new("src").join("test").join("empty.txt");
        let file_path = path.to_str().unwrap();

        let fs_item = FSItem::from_path(file_path).unwrap();
        let hash = fs_item.hash().unwrap();
        assert_eq!(
            hash,
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }

    #[test]
    fn from_path_works() {
        let path = Path::new("src").join("fs_lib.rs");
        let file_path = path.to_str().unwrap();

        match FSItem::from_path(file_path) {
            Ok(fs_item) => {
                println!("{:?}", fs_item);
                assert!(true)
            }
            e => {
                println!("{:?}", e);
                assert!(false)
            }
        }
    }

    #[test]
    fn from_path_reject_invalid_file() {
        match FSItem::from_path("not_exists") {
            Ok(_) => assert!(false),
            Err(e) => match e.kind() {
                std::io::ErrorKind::NotFound => assert!(true),
                _ => assert!(false),
            },
        }
    }

    #[test]
    fn can_create_empty_fs_item() {
        let empty_item = FSItem {
            is_file: true,
            byte_length: 0,
            path: String::from(
                Path::new("src")
                    .join("test")
                    .join("empty.txt")
                    .canonicalize()
                    .unwrap()
                    .to_str()
                    .unwrap(),
            ),
            modified: 0,
        };

        assert_eq!(empty_item.is_file, true);
        assert_eq!(empty_item.byte_length, 0);
        assert!(empty_item.path.ends_with("empty.txt"));
        assert_eq!(empty_item.modified, 0);
    }
}
