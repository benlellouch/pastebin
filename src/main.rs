#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;


mod paste_id;

use paste_id::PasteId;
use std::path::Path;
use std::fs::File;
use std::fs;
use std::io::prelude::*;


use rocket::request::Form;
use rocket_contrib::templates::Template;
use rocket::response::{Redirect, Flash};

#[derive(FromForm)]
struct Paste
{
    content: String
}

#[derive(FromForm)]
struct DeletePaste
{
    pastes: String
}

#[derive(Serialize)]
struct PasteTemplate
{
    pastes: Vec<String>
}

#[get("/")]
fn index() -> Template
{
    let path = Path::new("./upload/");
    let pastes = PasteTemplate{pastes: read_dir(&path)};
    Template::render("index", &pastes)
}

fn read_dir(path: &Path) -> Vec<String>
{
    let paths = fs::read_dir(path).unwrap();
    let mut pastes: Vec<String> = vec![];
    for path in paths
    {
        let unwrapped_path = path.unwrap().path();
        let path_str = unwrapped_path.to_str().unwrap();
        let path_str = path_str.replace("./upload/", "");
        pastes.push(String::from(path_str));
    }

    pastes
}

#[post("/", data = "<paste>")]
fn upload(paste: Form<Paste>) -> Result<Flash<Redirect>, std::io::Error>
{
    let id = PasteId::new(3);
    let filename = format!("upload/{id}", id = id);

    let mut file = File::create(&filename)?;
    file.write(paste.content.as_bytes())?;

    Ok(Flash::success(Redirect::to("/"), "Successfully created paste"))
}

#[post("/delete", data="<paste>")]
fn delete(paste: Form<DeletePaste>) -> Result<Flash<Redirect>, std::io::Error>
{
    fs::remove_file(format!("upload/{id}", id = paste.pastes))?;
    Ok(Flash::success(Redirect::to("/"), "Successfully deleted paste"))
}

#[get("/<id>")]
fn retrieve(id: PasteId) -> Option<File>
{
    let filename = format!("upload/{id}", id = id);
    File::open(&filename).ok()
}

fn main() {
    rocket::ignite()
    .mount("/", routes![index, upload, retrieve, delete])
    .attach(Template::fairing())
    .launch();
}
