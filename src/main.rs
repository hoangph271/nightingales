use std::path::Path;
// mod gui;
mod fs_walker;

fn main() {
  let dir = Path::new(".");
  fs_walker::walk_dir(dir, &|fs_item| {
    match fs_item {
      fs_walker::FSItem::DirPath(dir_path) => println!("Dir: {:#?}", dir_path),
      fs_walker::FSItem::File(file) => println!("{:#?}", file.metadata().unwrap().modified().unwrap()),
    }

    Ok(())
  });
  // gui::start_app();
}
