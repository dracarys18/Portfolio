#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
mod temp;

use std::{ffi::OsStr, io::Cursor, path::PathBuf};

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
        "{}/articles/{}.md",
        std::env::current_dir().unwrap().display(),
        file
    );
    let context = PostIndex::new(file_path);
    Template::render("blog/post", context)
}
#[get("/favicon.ico")]
fn favicon<'f>() -> response::Result<'f> {
    let fav = std::fs::read(format!(
        "{}/static/favicon.ico",
        std::env::current_dir().unwrap().display()
    ))
    .unwrap();
    response::Response::build()
        .header(ContentType::Icon)
        .sized_body(Cursor::new(fav))
        .ok()
}
#[get("/static/<file..>")]
fn get_css<'c>(file: PathBuf) -> response::Result<'c> {
    let ext = file.as_path().extension().and_then(OsStr::to_str).unwrap();
    let content = std::fs::read_to_string(format!(
        "{}/static/{}",
        std::env::current_dir().unwrap().display(),
        file.display().to_string()
    ))
    .unwrap();
    response::Response::build()
        .header(ContentType::from_extension(ext).unwrap())
        .sized_body(Cursor::new(content))
        .ok()
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, favicon, blog, get_article, get_css])
        .attach(Template::fairing())
        .mount(
            "/static",
            StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/static")),
        )
        .launch();
}
