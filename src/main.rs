#[macro_use]
extern crate rocket;

use controllers::{create_shorten_url, error, index, redirect_ori_url};
use db::init_connection_db;

mod controllers;
mod db;
mod utils;

#[launch]
fn rocket() -> _ {
    let client_db = init_connection_db();
    rocket::build().manage(client_db).mount(
        "/",
        routes![index, redirect_ori_url, create_shorten_url, error],
    )
}
