```
cargo install cargo-watch systemfd
```

```
systemfd --no-pid -s http::4000 -- cargo watch -x run
```