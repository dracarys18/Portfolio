#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
mod temp;

use std::io::Cursor;

use crate::temp::PostIndex;
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

#[get("/blog/<file>")]
fn get_article(file: String) -> Template {
    let file_path = format!(
        "{}/articles/{}",
        std::env::current_dir().unwrap().display(),
        file
    );
    let context = PostIndex::new(file_path);
    Template::render("blog/post", context)
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
        .mount("/", routes![index, favicon, blog, get_article])
        .attach(Template::fairing())
        .mount(
            "/static",
            StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/static")),
        )
        .launch();
}
