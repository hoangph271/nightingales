#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate rocket_multipart_form_data;

use byte_unit;
use rocket_multipart_form_data as rmfd;
use rocket_multipart_form_data::MultipartFormDataError;

use rocket::http::ContentType;
use rocket::{Data, Request};

#[macro_use]
extern crate rocket_contrib;

use rocket_contrib::databases::diesel;

#[database("sqlite_logs")]
struct LogsDbConn(diesel::SqliteConnection);

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[derive(Debug, Responder)]
enum ApiResponse {
    #[response(status = 400)]
    BadRequest(String),
    #[response(status = 404)]
    NotFound(String),
    #[response(status = 413)]
    RequestEntityTooLarge(String),
    #[response(status = 201)]
    Created(String),
}

#[post("/users", data = "<data>")]
fn post_users(conn: LogsDbConn, content_type: &ContentType, data: Data) -> ApiResponse {
    println!("{:?}", conn);

    let mut options = rmfd::MultipartFormDataOptions::with_multipart_form_data_fields(vec![
        match rmfd::MultipartFormDataField::file("avatar")
            .size_limit(byte_unit::n_mb_bytes!(4) as u64)
            .content_type_by_string(Some(rmfd::mime::IMAGE_STAR))
        {
            Ok(avatar) => avatar,
            Err(e) => {
                eprintln!("{:?}", e);

                // TODO: Split handling 412, 400...?
                // match e {
                // }

                return ApiResponse::BadRequest(String::from("avatar field is not an image"));
            }
        },
        rmfd::MultipartFormDataField::raw("fingerprint"),
        rmfd::MultipartFormDataField::text("email"),
        rmfd::MultipartFormDataField::text("password"),
        rmfd::MultipartFormDataField::text("fullname"),
    ]);

    let mut form_data = match rmfd::MultipartFormData::parse(content_type, data, options) {
        Ok(form_data) => form_data,
        Err(e) => {
            eprintln!("{:?}", e);
            return ApiResponse::BadRequest(String::from("avatar field is invalid"));
        }
    };

    let avatar = match form_data.files.get("avatar") {
        Some(avatar) => println!("{:?}", avatar),
        None => return ApiResponse::BadRequest(String::from("avatar is required")),
    };

    println!("{:?}", avatar);
    let email = match form_data.texts.remove("email") {
        Some(email) => email,
        None => return ApiResponse::BadRequest(String::from("email is requried")),
    };
    let password = form_data.texts.remove("password");
    let fullname = form_data.texts.remove("fullname");

    // if let Some(mut text_fields) = email {
    //     let text_field = text_fields.remove(0);

    //     // let _content_type = text_field.content_type;
    //     // let _file_name = text_field.file_name;
    //     // let _text = text_field.text;

    //     println!("{:?}", text_field);
    //     // You can now deal with the text data.
    // }

    // TODO: Validate more

    ApiResponse::Created(String::from("Created"))
}

#[catch(404)]
fn not_found(_req: &Request) -> &'static str {
    "Not Found"
}

fn main() {
    rocket::ignite()
        .attach(LogsDbConn::fairing())
        .register(catchers![not_found])
        .mount("/", routes![post_users])
        .mount("/", routes![index])
        .launch();
}
