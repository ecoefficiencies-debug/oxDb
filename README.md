# oxDb

## Import Markdown into Redis

The `reader_doc` binary reads one Markdown file or recursively scans a directory
for files with the `.md` extension. It requires a reachable Redis instance and a
`REDIS_URL` environment variable.

### Local Redis with Docker

For local development, run Redis bound only to loopback with persistence
disabled:

If a local Redis is already responding on port 6379, reuse it instead of
starting another container on the same port.

```sh
docker run -d --name oxdb-local-redis -p 127.0.0.1:6379:6379 redis:7-alpine redis-server --save "" --appendonly no
export REDIS_URL=redis://127.0.0.1:6379/
docker exec oxdb-local-redis redis-cli ping
```

The ping should return `PONG`. If the named container already exists but is
stopped, start it with `docker start oxdb-local-redis`; if it is already running,
reuse it. If port 6379 is occupied, choose another host port in the mapping and
use that port in `REDIS_URL`. This development database is in memory only, so
its records are lost when the container stops. Stop it when finished with
`docker stop oxdb-local-redis`.

### Import Markdown

From the repository root, with `REDIS_URL` set:

```sh
cargo run --manifest-path mobile/Cargo.toml --bin reader_doc -- path/to/file-or-directory
```

The Redis URL may include credentials; keep it in the environment rather than
committing it to source control. The binary reports each stored file and its
assigned key. If no Markdown files are found, it exits with an error.

### Markdown fields

Files may start with YAML-style front matter delimited by `---` lines:

```markdown
---
title: Example title
author: Example author
---
Markdown body
```

When a closing delimiter is present, `title` and `author` are read from the
front matter; omitted values default to `Untitled` and `reader.doc`. The text
after the closing delimiter becomes the stored content. Without valid closed
front matter, the full file is stored as content, the first Markdown heading
(levels 1-3) is used as the title when present, and the author defaults to
`reader.doc`.

Each record is serialized as JSON and stored at `blog:<id>`. The ID is allocated
by incrementing the Redis key `blog:id`, so subsequent imports continue the
sequence.
