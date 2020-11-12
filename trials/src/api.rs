use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

enum Message {
  NewJob(Job),
  Terminate,
}
struct Worker {
  id: usize,
  thread: Option<thread::JoinHandle<()>>,
}
impl Worker {
  fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
    let thread = thread::spawn(move || loop {
      let message = receiver.lock().unwrap().recv().unwrap();

      if let Message::NewJob(job) = message {
        println!("Job ID: {:?}", id);
        job();
      } else {
        println!("Job #ID {:?} is dying slowly...! 😢", id);
        break;
      }
    });

    Worker {
      id,
      thread: Some(thread),
    }
  }
}

type Job = Box<dyn FnOnce() + Send + 'static>;

struct ThreadPool {
  workers: Vec<Worker>,
  sender: mpsc::Sender<Message>,
}
impl ThreadPool {
  fn new(size: usize) -> ThreadPool {
    assert!(size > 0);

    let (sender, receiver) = mpsc::channel();
    let receiver = Arc::new(Mutex::new(receiver));
    let mut workers = Vec::with_capacity(size);

    for id in 0..size {
      workers.push(Worker::new(id, Arc::clone(&receiver)));
    }

    ThreadPool { workers, sender }
  }
  fn execute<F>(&self, f: F)
  where
    F: FnOnce() + Send + 'static,
  {
    let job = Box::new(f);
    self.sender.send(Message::NewJob(job)).unwrap();
  }
}
impl Drop for ThreadPool {
  fn drop(&mut self) {
    for _ in &self.workers {
      self.sender.send(Message::Terminate).unwrap();
    }

    for worker in &mut self.workers {
      println!("Killin' {} softly...!", worker.id);

      if let Some(thread) = worker.thread.take() {
        thread.join().unwrap();
      }
    }
  }
}

fn handle_request(mut stream: TcpStream) -> Result<(), std::io::Error> {
  let mut buffer = [0; 1024];

  stream.read(&mut buffer)?;

  let (status_line, contents) = if buffer.starts_with(b"GET / HTTP/1.1\r\n") {
    ("HTTP/1.1 200 OK\r\n\r\n", fs::read_to_string("index.html")?)
  } else {
    (
      "HTTP/1.1 404 NOT FOUND\r\n\r\n",
      fs::read_to_string("404.html")?,
    )
  };

  let response = format!("{}{}", status_line, contents);

  stream.write(response.as_bytes())?;
  stream.flush()?;

  // std::process::exit(0);
  return Ok(());
}

pub fn start_server() -> Result<(), std::io::Error> {
  const SERVER_ROOT: &str = "127.0.0.1:2039";
  let pool = ThreadPool::new(4);
  let listener = TcpListener::bind(SERVER_ROOT)?;

  println!("Server started: http://{}", SERVER_ROOT);

  for stream in listener.incoming().take(8) {
    pool.execute(|| {
      handle_request(stream.unwrap()).unwrap();
    });
  }

  return Ok(());
}
