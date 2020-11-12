use std::fs;
use std::fs::File;
use std::io;
use std::path::Path;

pub enum FSItem<'a> {
  DirPath(&'a str),
  File(&'a File),
}

pub fn walk_dir(dir: &Path, cb: &dyn Fn(&FSItem) -> io::Result<()>) {
  if dir.is_dir() {
    cb(&FSItem::DirPath(&dir.to_str().unwrap())).unwrap();

    for entry in fs::read_dir(dir).unwrap() {
      walk_dir(entry.unwrap().path().as_path(), cb);
    }
  } else {
    let file = File::open(dir).unwrap();
    cb(&FSItem::File(&file)).unwrap();
  }
}
