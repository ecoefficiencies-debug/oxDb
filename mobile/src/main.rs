extern crate redis;
use redis::Commands;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Blog {
    pub title: String,
    pub content: String,
    pub author: String,
}

fn main() {
    // Simple demo: try to upload and fetch a blog when run manually.
    if let Ok(url) = std::env::var("REDIS_URL") {
        if let Ok(client) = redis::Client::open(url) {
            if let Ok(mut con) = client.get_connection() {
                let blog = Blog { title: "Hello".into(), content: "World".into(), author: "me".into() };
                if let Ok(id) = upload_blog(&mut con, &blog) {
                    if let Ok(fetched) = fetch_blog(&mut con, id) {
                        println!("uploaded blog {} => {:?}", id, fetched);
                    }
                }
            }
        }
    }
}

pub fn upload_blog(con: &mut redis::Connection, blog: &Blog) -> redis::RedisResult<usize> {
    let id: usize = con.incr("blog:id", 1)?;
    let key = format!("blog:{}", id);
    let data = serde_json::to_string(blog).map_err(|e| redis::RedisError::from((redis::ErrorKind::TypeError, format!("serde error: {}", e))))?;
    let _: () = con.set(&key, data)?;
    Ok(id)
}

pub fn fetch_blog(con: &mut redis::Connection, id: usize) -> redis::RedisResult<Blog> {
    let key = format!("blog:{}", id);
    let data: String = con.get(&key)?;
    let blog: Blog = serde_json::from_str(&data).map_err(|e| redis::RedisError::from((redis::ErrorKind::TypeError, format!("serde error: {}", e))))?;
    Ok(blog)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn upload_and_fetch_blog() -> Result<(), Box<dyn std::error::Error>> {
        let url = match env::var("REDIS_URL") {
            Ok(u) => u,
            Err(_) => {
                eprintln!("REDIS_URL not set - skipping test");
                return Ok(());
            }
        };

        let client = redis::Client::open(url)?;
        let mut con = client.get_connection()?;

        let blog = Blog { title: "Test".into(), content: "Body".into(), author: "Tester".into() };
        let id = upload_blog(&mut con, &blog)?;
        let fetched = fetch_blog(&mut con, id)?;
        assert_eq!(blog, fetched);

        let key = format!("blog:{}", id);
        let _: () = con.del(key)?;

        Ok(())
    }
}
