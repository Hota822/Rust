
#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
// #[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_migrations;
// extern crate log;

// use log::logs;

use rocket_contrib::databases::diesel;

#[database("sqlite_logs")]
struct LogsDbConn(diesel::SqliteConnection);

fn main() {
    rocket::ignite()
       .attach(LogsDbConn::fairing())
       .launch();
}

#[get("/logs/<id>")]
fn get_logs(conn: LogsDbConn, id: usize) -> () {
    // logs::filter(id.eq(log_id)).load(&*conn)
}