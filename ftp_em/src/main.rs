use base64;
use ftp::FtpStream;
use sha2::{Digest, Sha256};
use std::path::Path;
use std::str;

mod constants;

fn main() {
    let mut ftp_stream = FtpStream::connect("127.0.0.1:21").unwrap();
    ftp_stream
        .login(constants::USERNAME, constants::PASSWORD)
        .unwrap();

    walk_ftp_dir(None, &mut ftp_stream);

    let _ = ftp_stream.quit();
}

fn walk_ftp_dir(dir_path: Option<&str>, ftp_stream: &mut FtpStream) {
    let files = ftp_stream.nlst(dir_path).unwrap();

    let dir_path = match dir_path {
        Some(path) => Path::new(path),
        None => Path::new(""),
    };

    for file in files {
        let full_path = dir_path.join(file);
        let full_path = full_path.to_str().unwrap();

        if is_dir(full_path, ftp_stream) {
            println!("{}", full_path);
            walk_ftp_dir(Some(full_path), ftp_stream);
        } else {
            let mdtm = ftp_stream
                .mdtm(full_path)
                .expect(format!("{}: mdtm failed...!", full_path).as_str())
                .unwrap();
            let size = ftp_stream
                .size(full_path)
                .expect(format!("WTF, {}...?", full_path).as_str())
                .unwrap();

            let hash = create_hash(full_path, ftp_stream);
            println!("[{}]: {}", mdtm, full_path);
            println!("{} - {}", hash, size);
        }
    }
}

fn is_dir(path: &str, ftp_stream: &mut FtpStream) -> bool {
    match ftp_stream.size(path) {
        Ok(_) => false,
        Err(_) => true,
    }
}

fn create_hash(path: &str, ftp_stream: &mut FtpStream) -> String {
    ftp_stream
        .retr(path, move |stream| {
            let mut buffer = [0; 1024];
            let mut hasher = Sha256::new();

            loop {
                let read = stream.read(&mut buffer).unwrap();

                if read == 0 {
                    break;
                }

                hasher.update(buffer);
            }

            Ok(base64::encode(hasher.finalize()))
        })
        .unwrap()
}
