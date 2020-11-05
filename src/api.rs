use std::net::TcpListener;

pub fn start_server() -> Result<(), std::io::Error> {
  const SERVER_ROOT: &str = "127.0.0.1:2039";
  let listener = TcpListener::bind(SERVER_ROOT)?;

  println!("Server started: http://{}", SERVER_ROOT);

  for stream in listener.incoming() {
    let stream = stream.unwrap();
    println!("Connection established...!");
  }

  return Ok(());
}
