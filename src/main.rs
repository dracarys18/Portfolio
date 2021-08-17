#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
mod temp;

use std::io::Cursor;

use rocket::{http::ContentType, response};
use rocket_contrib::{serve::StaticFiles, templates::Template};
use temp::{BlogIndex, Index};

#[get("/")]
fn index() -> Template {
    let context = Index::default();
    Template::render("index", &context)
}

#[get("/blog")]
fn blog() -> Template {
    let context = BlogIndex::default();
    Template::render("blog/index", &context)
}
#[get("/favicon.ico")]
fn favicon<'f>() -> response::Result<'f> {
    let fav = std::fs::read("static/favicon.ico").unwrap();
    response::Response::build()
        .header(ContentType::Icon)
        .sized_body(Cursor::new(fav))
        .ok()
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, favicon, blog])
        .attach(Template::fairing())
        .mount("/static", StaticFiles::from("static"))
        .launch();
}
