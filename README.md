Shorten URL Application using Rust (Rocket + Redis)

## 1. Set up

- Clone this project

```bash
git clone git_url
```

- Install & run redis. You can run redis with docker:

```bash
docker run -p 6379:6379 -d redis
```

## 2. Using

- Run project:

```bash
cargo run
```

- Shorten url:

POST: http://127.0.0.1:8000

Body:

```json
{
  "ori_url": "<your_url>"
}
```

Response will include url shorted. Example:

Shorten successful to http://127.0.0.1:8000/0tTGhiP
