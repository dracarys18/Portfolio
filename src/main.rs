#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
mod temp;

use rocket_contrib::{serve::StaticFiles, templates::Template};
use temp::Index;

#[get("/")]
fn index() -> Template {
    let context = Index::default();
    Template::render("index", &context)
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .attach(Template::fairing())
        .mount("/static", StaticFiles::from("static"))
        .launch();
}
