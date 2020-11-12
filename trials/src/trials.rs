// ! FIXME: Not working
// #region // ! fs
// use std::env;
// use std::error;
// use std::fs;
// use std::path::Path;
// use std::time::SystemTime;

// type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

// #[derive(Debug)]
// struct FSItem {
//   fs_path: String,
//   modified: u128,
//   is_file: bool,
// }

// static base_path: &str = env::current_dir().unwrap().as_path().to_str().unwrap();

// impl FSItem {
//   fn stats_local(full_path: &str) -> Result<FSItem> {
//     let metadata = fs::metadata(full_path);

//     match metadata {
//       Ok(v) => {
//         let fs_path = full_path.replace(base_path, "");
//         let is_file = v.is_file();

//         match v.modified() {
//           Ok(modified) => match modified.duration_since(SystemTime::UNIX_EPOCH) {
//             Ok(duration) => Ok(FSItem {
//               fs_path,
//               is_file,
//               modified: duration.as_millis(),
//             }),
//             Err(e) => Box<Err>(e),
//           },
//           Err(e) => return Err(e),
//         }
//       }
//       Err(e) => Err(e),
//     }
//   }
// }

// fn main() {
//   let full_path = Path::new("myfile.db").to_str().unwrap();
//   let metadata = fs::metadata(full_path);

//   match metadata {
//     Ok(v) => {
//       let fs_item = FSItem {
//         fs_path: full_path.replace(env::current_dir().unwrap().as_path().to_str().unwrap(), ""),
//         is_file: v.is_file(),
//         modified: v
//           .modified()
//           .unwrap()
//           .duration_since(SystemTime::UNIX_EPOCH)
//           .unwrap()
//           .as_millis(),
//       };

//       println!("{:#?}", fs_item)
//     }
//     Err(e) => println!("{:?}", e),
//   }
// }
// #endregion

// // #region // ! sqlite
// use rusqlite::{Connection, Error as SqlError, Row, NO_PARAMS};

// #[derive(Debug)]
// struct Person {
//   name: String,
//   email: String,
// }
// #[derive(Debug)]
// struct DbPerson {
//   _id: isize,
//   person: Person,
// }
// struct PersonModel<'a> {
//   conn: &'a Connection,
// }

// #[allow(dead_code)]
// impl<'a> PersonModel<'a> {
//   const ROW_TO_PERSON: fn(&Row) -> Result<DbPerson, SqlError> = |row| {
//     Ok(DbPerson {
//       _id: row.get(0)?,
//       person: Person {
//         name: row.get(1)?,
//         email: row.get(2)?,
//       },
//     })
//   };
//   fn new(conn: &'a Connection) -> Self {
//     Self { conn: conn }
//   }
//   fn init_tbl(&self) -> usize {
//     self
//       .conn
//       .execute("DROP TABLE IF EXISTS person_tbl", NO_PARAMS)
//       .unwrap();

//     self
//       .conn
//       .execute(
//         "CREATE TABLE person_tbl (
//           _id INTEGER PRIMARY KEY AUTOINCREMENT,
//           name TEXT NOT NULL,
//           email TEXT NOT NULL
//         )",
//         NO_PARAMS,
//       )
//       .unwrap()
//   }
//   fn read(&self, id: isize) -> Result<Option<DbPerson>, SqlError> {
//     let query_result = self.conn.query_row_named(
//       "
//         SELECT _id, name, email
//         FROM person_tbl
//         WHERE _id = :_id",
//       &[(":_id", &id)],
//       PersonModel::ROW_TO_PERSON,
//     );

//     match query_result {
//       Ok(result) => Ok(Some(result)),
//       Err(e) => match e {
//         SqlError::QueryReturnedNoRows => Ok(None),
//         _ => Err(e),
//       },
//     }
//   }
//   fn read_all(&self) -> Result<Vec<DbPerson>, SqlError> {
//     let mut sql = self.conn.prepare(
//       "SELECT _id, name, email
//         FROM person_tbl",
//     )?;

//     let person_rows = sql.query_map(NO_PARAMS, PersonModel::ROW_TO_PERSON)?;

//     let mut persons = vec![];

//     for person_row in person_rows {
//       persons.push(person_row?);
//     }

//     Ok(persons)
//   }
//   fn create(&self, person: Person) -> Result<i64, SqlError> {
//     let insert_count = self.conn.execute_named(
//       "INSERT INTO person_tbl (name, email) VALUES (:name, :email);",
//       &[(":name", &person.name), (":email", &person.email)],
//     )?;

//     if let 1 = insert_count {
//       Ok(self.conn.last_insert_rowid())
//     } else {
//       Err(SqlError::StatementChangedRows(0))
//     }
//   }
//   fn update(&self, db_person: DbPerson) -> Result<i64, SqlError> {
//     let update_count = self.conn.execute_named(
//       "UPDATE person_tbl
//         SET name = :name, email = :email
//         WHERE _id = :_id",
//       &[
//         (":name", &db_person.person.name),
//         (":email", &db_person.person.email),
//         (":_id", &db_person._id),
//       ],
//     )?;

//     if let 1 = update_count {
//       Ok(1)
//     } else {
//       Err(SqlError::StatementChangedRows(0))
//     }
//   }
// }

// fn init_person_tbl_data(person_model: &PersonModel) {
//   person_model.init_tbl();

//   person_model
//     .create(Person {
//       name: "@HHP".to_string(),
//       email: "phh@hhp".to_string(),
//     })
//     .unwrap();
//   person_model
//     .create(Person {
//       name: "Steve Example".to_string(),
//       email: "steve@example.org".to_string(),
//     })
//     .unwrap();
//   person_model
//     .create(Person {
//       name: "Orv Jund".to_string(),
//       email: "orvjund@example.com".to_string(),
//     })
//     .unwrap();
// }

// fn main() -> Result<(), SqlError> {
//   let conn = Connection::open("myfile.db").unwrap();
//   let person_model = PersonModel::new(&conn);

//   init_person_tbl_data(&person_model);

//   let db_persons = person_model.read_all().or_else(|e| {
//     println!("Error on reading all: {:?}", e);
//     return Err(e);
//   })?;

//   println!("Person count: {:?}", db_persons.len());

//   let persons = person_model.read_all().or_else(|e| {
//     println!("Error on reading all: {:?}", e);
//     Err(e)
//   })?;

//   if let Some(db_person) = persons.get(0) {
//     let person = DbPerson {
//       _id: db_person._id,
//       person: Person {
//         name: if db_person.person.name.ends_with("...!") {
//           "@HHP".to_string()
//         } else {
//           "@HHP...!".to_string()
//         },
//         email: db_person.person.email.clone(),
//       },
//     };

//     person_model.update(person).or_else(|e| {
//       println!("Error on updating: {:#?}", e);
//       Err(e)
//     })?;

//     let query_result = person_model.read(db_person._id).or_else(|e| {
//       println!("Error on reading: {:#?}", e);
//       Err(e)
//     })?;

//     if let Some(person) = query_result {
//       println!("{:#?}", person);
//     } else {
//       println!("404 - Person not found...!");
//     }
//   } else {
//     println!("Insert some person");
//   }

//   return Ok(());
// }
// #endregion

// #region // ! panic!
// use std::fs::File;
// use std::io::ErrorKind;

// fn main() {
//   let file = File::open("text.txt");

//   let file = match file {
//     Ok(file) => file,
//     Err(error) => match error.kind() {
//       ErrorKind::NotFound => match File::create("text.txt") {
//         Ok(file) => file,
//         Err(error) => panic!("Error creating file: {:?}", error),
//       },
//       other_error => panic!("Error opening file: {:?}", other_error),
//     },
//   };

//   // ? Simpler...!
//   let file = File::open("text.txt").unwrap();
//   let file = File::open("text.txt").expect("Error opening file hello.txt");

//   println!("{:?}", file)
// }
// #endregion

// #region // ! hash maps
// use std::collections::HashMap;

// #[derive(Debug)]
// enum Champion {
//   ADC,
//   Jungler,
//   Tanker(String),
// }

// fn main() {
//   let jinx = Champion::ADC;
//   let yorick = Champion::Tanker(String::from("MP"));
//   let rammus = Champion::Tanker(String::from("AD"));
//   let lee = Champion::Jungler;

//   let mut champions_by_name: HashMap<String, Champion> = HashMap::new();

//   champions_by_name.insert(String::from("Jinx"), jinx);
//   champions_by_name.insert(String::from("Yorick"), yorick);
//   champions_by_name.insert(String::from("Rammus"), rammus);
//   champions_by_name.insert(String::from("Lee Sin"), lee);

//   println!("{:?}", champions_by_name.get(&String::from("Jinx")));

//   match champions_by_name.get(&String::from("Rammus")) {
//     Some(v) => println!("{:?}", v),
//     None => println!("404 - Rammus not found...!"),
//   }

//   match champions_by_name.get(&String::from("Jhin")) {
//     Some(v) => println!("{:?}", v),
//     None => println!("404 - Jhin not found...!"),
//   }

//   let skill_names = vec![
//     "Dancing Grenade",
//     "Deadly Flourish",
//     "Captive Audience",
//     "Curtain Call",
//   ];

//   let hotkeys = vec!["Q", "W", "E", "R"];

//   let mut keymap: HashMap<_, _> = hotkeys.into_iter().zip(skill_names.into_iter()).collect();
//   keymap.entry("").or_insert("Whisper");

//   match keymap.get("") {
//     Some(v) => println!("{:?}", v),
//     None => println!("404 - Jhin not found...!"),
//   }
// }
// #endregion

// #region // ! strings
// fn main() {
//   for char in "नमस्ते: Huy Hoàng Phan...! 🙊".chars() {
//     println!("{}", char);
//   }

//   println!("Length: {}", "नमस्ते: Huy Hoàng Phan...! 🙊".chars().count());

//   for byte in "🐁 em 🥺".bytes() {
//     println!("{}", byte);
//   }
// }
// #endregion

// #region // ! enum
// * enum matching
// * enum with value
// #[derive(Debug)]
// enum UsState {
//     Alabama,
//     Alaska,
//     Others,
// }

// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter(UsState),
// }

// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => {
//             println!("BigBang...!");
//             1
//         }
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter(state) => {
//             println!("State is: {:?}", state);
//             25
//         }
//     }
// }

// fn main() {
//     let coin = Coin::Quarter(UsState::Alabama);
//     let cents = value_in_cents(coin);
//     println!("Cents: {}", cents);

//     value_in_cents(Coin::Penny);
// }
// #endregion

// #region // ! Guessin' game
// * let mut
// * std::io::stdin
// * rand::thread_rng()
// * match
// * .expect
// use rand::Rng;
// use std::cmp::Ordering;

// fn main() {
//     let secret = rand::thread_rng().gen_range(1, 101);

//     loop {
//         println!("Guess a number: ");
//         let mut guess = String::new();
//         std::io::stdin().read_line(&mut guess).expect("400");

//         match guess.trim().parse::<u32>() {
//             Ok(v) => {
//                 match v.cmp(&secret) {
//                     Ordering::Less => println!("Bigger...!"),
//                     Ordering::Greater => println!("Smoller...!"),
//                     Ordering::Equal => {
//                         println!("Won...!");
//                         break;
//                     }
//                 }

//                 println!("Nope...!");
//             }
//             Err(e) => println!("400: {} - !{}!", e, guess),
//         }
//     }
// }
// #endregion
