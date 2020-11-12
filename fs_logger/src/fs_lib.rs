use ring::digest;
use std::fs;
use std::io::*;
use std::path::Path;
use std::time::SystemTime;

#[derive(Debug)]
pub struct FSItem<'a> {
    is_file: bool,
    byte_length: u64,
    key: &'a str,
    modified: u64,
}

// TODO: Global root...?
// TODO: Key, hash

impl FSItem<'_> {
    pub fn hash(&self) -> Result<String> {
        let content = &fs::read(self.key)?;
        let actual = digest::digest(&digest::SHA256, content);

        Ok(hex::encode(actual.as_ref()))
    }

    pub fn from_path(path: &str) -> Result<FSItem> {
        let metadata = fs::metadata(Path::new(path))?;

        let is_file = metadata.is_file();
        let byte_length = metadata.len();
        let key = path;
        let modified = metadata
            .modified()?
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("Error getting modified")
            .as_millis() as u64;

        Ok(FSItem {
            is_file,
            byte_length,
            key,
            modified,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(hash, "87ee07307593493b96730dcbb36fd51e9fa4ba189696dad60758e89e6e7750bf");
    }

    #[test]
    fn hash_empty_file_works() {
        let path = Path::new("src").join("test").join("empty.txt");
        let file_path = path.to_str().unwrap();

        let fs_item = FSItem::from_path(file_path).unwrap();
        let hash = fs_item.hash().unwrap();
        assert_eq!(hash, "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855");
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
            key: "empty.txt",
            modified: 0,
        };

        assert_eq!(empty_item.is_file, true);
        assert_eq!(empty_item.byte_length, 0);
        assert_eq!(empty_item.key, "empty.txt");
        assert_eq!(empty_item.modified, 0);
    }
}
