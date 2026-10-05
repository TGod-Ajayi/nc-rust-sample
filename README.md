# nc-rust-sample

A minimal [axum](https://github.com/tokio-rs/axum) web service, ready to deploy on [Naijacloud](https://naijacloud.com).

- `GET /` returns a greeting
- `GET /health` returns `{"status":"ok"}`

It listens on `0.0.0.0:$PORT` (defaults to 3000). Naijacloud sets `PORT` for you.

## Run locally

```sh
cargo run
curl localhost:3000
```

## Deploy on Naijacloud

1. In the dashboard: **New service → Web service → Public repo**.
2. Repository URL: `https://github.com/TGod-Ajayi/nc-rust-sample`, branch `main`.
3. Pick an instance type and region, then **Create web service**.

Naijacloud detects the Cargo project, builds a release binary and runs it.
