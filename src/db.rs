use redis::{Client, Connection};
use rocket::State;

pub fn init_connection_db() -> Client {
    let client = redis::Client::open("redis://127.0.0.1:6379/");
    if client.is_err() {
        panic!("Can not open connect to redis");
    }

    client.unwrap()
}

pub fn get_conn(client: &State<Client>) -> Connection {
    client.get_connection().unwrap()
}
