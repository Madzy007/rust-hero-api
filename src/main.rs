#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate diesel;

mod db;
mod schema;
mod hero;

use hero::{Hero};
use rocket_contrib::json::{Json, JsonValue};

#[post("/", data = "<hero>")]
fn createHero(hero: Json<Hero>, connection: db::Connection) -> Json<Hero> {
    let insert = Hero { id: None, ..hero.into_inner() };
    Json(Hero::create(insert, &connection))
}

#[get("/")]
fn read(connection: db::Connection) -> Json<JsonValue> {
    Json(json!(Hero::read(&connection)))
}

#[put("/<id>", data = "<hero>")]
fn update(id: i32, hero: Json<Hero>, connection: db::Connection) -> Json<JsonValue> {
    let update = Hero { id: Some(id), ..hero.into_inner() };
    Json(json!({
        "success": Hero::update(id, update, &connection)
    }))
}

#[delete("/<id>")]
fn delete(id: i32, connection: db::Connection) -> Json<JsonValue> {
    Json(json!({
        "success": Hero::delete(id, &connection)
    }))
}

#[get("/<name>/<age>")]
fn hello(name: String, age: u8) -> String {
    format!("Hello {}, you are {} years old", name, age)
}

fn main() {
    rocket::ignite()
        .mount("/hero", routes![createHero, update, delete])
        .mount("/heroes", routes![read])
        .manage(db::connect())
        .launch();
}