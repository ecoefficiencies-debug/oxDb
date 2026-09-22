use redis::Commands;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Blog {
    pub title: String,
    pub content: String,
    pub author: String,
}

fn clean_value(value: &str) -> String {
    value
        .trim()
        .trim_matches('"')
        .trim_matches('\'')
        .to_string()
}

fn parse_front_matter(raw: &str) -> (String, String, String, String) {
    let lines: Vec<&str> = raw.lines().collect();

    if lines.first() == Some(&"---") {
        let end_index = lines.iter().position(|line| *line == "---").unwrap_or(lines.len());

        if end_index > 0 {
            let front = lines[1..end_index].join("\n");
            let body = lines[end_index + 1..].join("\n");

            let title = front
                .lines()
                .find_map(|line| line.strip_prefix("title:"))
                .map(clean_value)
                .unwrap_or_else(|| "Untitled".to_string());

            let author = front
                .lines()
                .find_map(|line| line.strip_prefix("author:"))
                .map(clean_value)
                .unwrap_or_else(|| "reader.doc".to_string());

            return (title, body.trim().to_string(), author, "frontmatter".to_string());
        }
    }

    let title = raw
        .lines()
        .find_map(|line| {
            line.strip_prefix("# ")
                .or_else(|| line.strip_prefix("## "))
                .or_else(|| line.strip_prefix("### "))
        })
        .map(str::trim)
        .unwrap_or_else(|| "Untitled".to_string());

    (title, raw.trim().to_string(), "reader.doc".to_string(), "plain".to_string())
}

fn read_markdown_file(path: &Path) -> Result<Blog, Box<dyn std::error::Error>> {
    let raw = fs::read_to_string(path)?;
    let (title, content, author, _) = parse_front_matter(&raw);

    Ok(Blog {
        title,
        content,
        author,
    })
}

fn upload_blog(con: &mut redis::Connection, blog: &Blog) -> redis::RedisResult<usize> {
    let id: usize = con.incr("blog:id", 1)?;
    let key = format!("blog:{}", id);
    let data = serde_json::to_string(blog)
        .map_err(|e| redis::RedisError::from((redis::ErrorKind::TypeError, format!("serde error: {}", e))))?;
    let _: () = con.set(&key, data)?;
    Ok(id)
}

fn walk_markdown_files(path: &Path, files: &mut Vec<PathBuf>) -> io::Result<()> {
    if path.is_file() {
        if path.extension().and_then(|ext| ext.to_str()) == Some("md") {
            files.push(path.to_path_buf());
        }
        return Ok(());
    }

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let child = entry.path();
        if child.is_dir() {
            walk_markdown_files(&child, files)?;
        } else if child.extension().and_then(|ext| ext.to_str()) == Some("md") {
            files.push(child);
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let target = env::args()
        .nth(1)
        .unwrap_or_else(|| panic!("Usage: reader_doc <markdown-file-or-directory>"));

    let url = env::var("REDIS_URL").expect("REDIS_URL must be set");
    let client = redis::Client::open(url)?;
    let mut con = client.get_connection()?;

    let root = PathBuf::from(target);
    let mut files = Vec::new();
    walk_markdown_files(&root, &mut files)?;

    if files.is_empty() {
        eprintln!("No .md files found under {}", root.display());
        std::process::exit(1);
    }

    for file in files {
        let blog = read_markdown_file(&file)?;
        let id = upload_blog(&mut con, &blog)?;
        println!("stored {} from {} as blog:{}", blog.title, file.display(), id);
    }

    Ok(())
}
