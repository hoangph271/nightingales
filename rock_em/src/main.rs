#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate rocket_multipart_form_data;

use rocket_multipart_form_data as rmfd;

use rocket::http::ContentType;
use rocket::{Data, Request};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/users", data = "<data>")]
fn post_users(content_type: &ContentType, data: Data) -> &'static str {
    // TODO: NOT unwrap...! :"<
    let mut options = rmfd::MultipartFormDataOptions::with_multipart_form_data_fields(
        vec! [
            rmfd::MultipartFormDataField::file("avatar").content_type_by_string(Some(rmfd::mime::IMAGE_STAR)).unwrap(),
            // rmfd::MultipartFormDataField::raw("fingerprint").size_limit(4096),
            rmfd::MultipartFormDataField::text("email"),
            rmfd::MultipartFormDataField::text("password"),
            rmfd::MultipartFormDataField::text("fullname"),
        ]
    );

    let mut form_data = rmfd::MultipartFormData::parse(content_type, data, options).unwrap();

    let avatar = form_data.files.get("avatar").or_else(|| { "avatar is required" });
    let email = form_data.texts.remove("email");
    let password = form_data.texts.remove("password");
    let fullname = form_data.texts.remove("fullname");

    if let Some(mut text_fields) = email {
        let text_field = text_fields.remove(0);

        // let _content_type = text_field.content_type;
        // let _file_name = text_field.file_name;
        // let _text = text_field.text;

        println!("{:?}", text_field);
        // You can now deal with the text data.
    }

    println!("{:?}", avatar);

    // TODO: Validate more

    "Ok"
}

#[catch(404)]
fn not_found(_req: &Request) -> &'static str {
    "Not Found"
}

fn main() {
    rocket::ignite()
        .register(catchers![not_found])
        .mount("/", routes![post_users])
        .mount("/", routes![index])
        .launch();
}
