#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate rocket;

extern crate dotenv;
extern crate rocket_contrib;

use diesel::pg::PgConnection;
use diesel::Connection;
use dotenv::dotenv;
// use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;
use std::env;

// images
use rocket::response::NamedFile;
use std::path::{Path, PathBuf};

// diagrams, model and schema modules
// Todo uncomment when ready

pub mod diagrams;
pub mod models;
pub mod schema;

// connecting to postgres
pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be added to .env");

    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

// get images
#[get("/img/<file..>")]
fn images(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("img/").join(file)).ok()
}

fn main() {
    rocket::ignite()
        // .mount("templates/", StaticFiles::from("static"))
        .mount(
            "/",
            routes![
                images,
                diagrams::list,
                diagrams::new,
                diagrams::insert,
                diagrams::update,
                diagrams::process_update,
                diagrams::delete
            ],
        )
        .attach(Template::fairing())
        .launch();
}
