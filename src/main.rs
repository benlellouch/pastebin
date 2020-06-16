#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

mod paste_id;

use paste_id::PasteId;
use std::io;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;

use rocket::Data;
use rocket::http::RawStr;
use rocket::request::Form;

#[derive(FromForm)]
struct Paste
{
    content: String
}

#[get("/")]
fn index() -> Option<File>
{
    File::open("static/index.html").ok()
}

#[post("/", data = "<paste>")]
fn upload(paste: Form<Paste>) -> Result<String, std::io::Error>
{
    let id = PasteId::new(3);
    let filename = format!("upload/{id}", id = id);
    let url = format!("{host}/{id}\n", host = "http://localhost:8000", id = id);

    let mut file = File::create(&filename)?;
    file.write(paste.content.as_bytes())?;

    Ok(url)
}

#[get("/<id>")]
fn retrieve(id: PasteId) -> Option<File>
{
    let filename = format!("upload/{id}", id = id);
    File::open(&filename).ok()
}

fn main() {
    rocket::ignite()
    .mount("/", routes![index, upload, retrieve])
    .launch();
}
