extern crate redis;
use redis::Commands;

fn main() {
    let i = fetch_an_integer().unwrap();
    println!("{}", i);
}

fn fetch_an_integer() -> redis::RedisResult<isize> {
    let redis_url = std::env::var("REDIS_URL")
        .expect("REDIS_URL must be set in mobile/.env or the environment");

    let client = redis::Client::open(redis_url)?;
    let mut con = client.get_connection()?;

    let _: () = con.set("my_key", 42)?;

    con.get("my_key")
}
