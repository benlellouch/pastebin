#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate rocket_contrib;


use rocket::request::Form;
use rocket_contrib::templates::Template;
use rocket_contrib::databases::rusqlite;
use rocket::response::{Redirect, Flash};

#[derive(FromForm)]
struct PasteForm
{
    name: String,
    content: String
}

#[derive(FromForm)]
struct DeletePaste
{
    paste_id: u32
}

#[derive(Serialize)]
struct PasteTemplate
{
    pastes: Vec<Paste>
}

#[derive(Debug, Serialize)]
struct Paste
{
    id: i32,
    name: String,
    content: String,
}

#[database("pastes_db")]
struct DbConn(rusqlite::Connection);

#[get("/")]
fn index(conn: DbConn) -> Template
{
    let mut stmt = conn.prepare("SELECT id, name, content FROM pastes").unwrap();
    let pastes = stmt.query_map(&[], |row|
    {
        Paste
        {
            id: row.get(0),
            name: row.get(1),
            content: row.get(2)
        }
    }
    ).unwrap().map(|paste| paste.unwrap());

    let pastes_templ = PasteTemplate{pastes: pastes.collect()};
    Template::render("index", &pastes_templ)
}

#[post("/", data = "<paste>")]
fn upload(conn: DbConn, paste: Form<PasteForm>) -> Result<Flash<Redirect>, rocket_contrib::databases::rusqlite::Error>
{
    conn.execute("INSERT INTO pastes (name, content) VALUES (?1, ?2)",
                &[&paste.name, &paste.content])?;

    Ok(Flash::success(Redirect::to("/"), "Successfully created paste"))
}

#[post("/delete", data="<paste>")]
fn delete(conn: DbConn, paste: Form<DeletePaste>) -> Result<Flash<Redirect>, rocket_contrib::databases::rusqlite::Error>
{
    conn.execute("DELETE FROM pastes WHERE id = ?", &[&paste.paste_id])?;
    Ok(Flash::success(Redirect::to("/"), "Successfully deleted paste"))
}

#[get("/<id>")]
fn retrieve(conn: DbConn, id: u32) -> Option<Template>
{
    let mut stmt = conn.prepare("SELECT id, name, content FROM pastes WHERE id = ?").unwrap();
    let result = stmt.query_map(&[&id], |row|
        {
          Paste
          {
              id: row.get(0),
              name: row.get(1),
              content: row.get(2)
          }  
        }
    ).unwrap().map(|paste| paste.unwrap());

    let mut pastes: Vec<Paste> = result.collect();
    let paste = pastes.pop();
    match paste
    {
        Some(p) => Some(Template::render("paste", &p)),
        None => None
    }
}
    

fn main() {
    rocket::ignite()
    .mount("/", routes![index, upload, retrieve, delete])
    .attach(Template::fairing())
    .attach(DbConn::fairing())
    .launch();
}
