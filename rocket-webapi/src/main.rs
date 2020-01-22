#![feature(proc_macro_hygiene)]
#![feature(decl_macro)]

#[macro_use]
extern crate rocket;

mod models;
mod routes;

use routes::*;


fn main() {
    rocket::ignite()
        .mount("/", routes![index, todos, new_todo, todo_by_id])
        .launch();
}

pub fn valify_signature(&self, channel_secret_key: &str, channel_token: &str) -> bool {
    // シークレットキーを暗号化


}