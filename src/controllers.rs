// #[macro_use]
use redis::{Client, Commands};
use rocket::response::Redirect;
use rocket::serde::{json::Json, Deserialize};
use rocket::State;

use crate::db::get_conn;
use crate::utils::generate_string;

#[get("/")]
pub fn index() -> &'static str {
    "Welcome to URL shorten App."
}

#[get("/error")]
pub fn error() -> &'static str {
    "Can not parse your url"
}

#[get("/<url>")]
pub fn redirect_ori_url(client: &State<Client>, url: &str) -> Redirect {
    let mut conn = get_conn(client);

    let ori: Result<String, redis::RedisError> = conn.get(url);
    println!("{:?}", ori);
    if ori.is_err() {
        println!("Error when getting ori url: {:?}", ori.err());
        return Redirect::to("/error");
    }

    Redirect::to(ori.unwrap())
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CreateData<'r> {
    ori_url: &'r str,
}

#[post("/", data = "<input>")]
pub fn create_shorten_url(client: &State<Client>, input: Json<CreateData<'_>>) -> String {
    let mut conn = get_conn(client);
    let key = generate_string(None);
    println!("{key}");
    let shorten: String = redis::cmd("SET")
        .arg(&key)
        .arg(input.ori_url)
        .query(&mut conn)
        .unwrap();
    println!("Shorten: {shorten}");
    format!("Shorten successful to http://127.0.0.1:8000/{}", key)
}
