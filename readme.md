```
# 자동 리로드 실행전 설치 크레이트
cargo install cargo-watch systemfd
# 자동 리로드 실행
systemfd --no-pid -s http::4000 -- cargo watch -x run
```

```
# 디비 생성 및 드랍 개발할때 사용
sqlx database create
sqlx database drop
```

```
# 디비 마이그레이션 생성 및 적용
sqlx migrate add enum
sqlx migrate run --dry-run
sqlx migrate run
```

```
# 디비 현재 커넥션 확인 state = "idle"
SELECT pid, usename, application_name, client_addr, state, query
FROM pg_stat_activity
WHERE datname = 'mydatabase';

# 디비 연결 커넥션 제거
SELECT pg_terminate_backend(388);
```



### /.vscode/settings.json
### vscode 서브 프로젝트 인텔리센스 워크스페이스에는 포함시키지않음 이유는 빌드결과물이 합쳐지길 바라진 않으니
```
{
    "rust-analyzer.linkedProjects": [
        "./Cargo.toml",
        "./oh_my_bevy/bevy_wasm_test/Cargo.toml"
    ],
    "rust-analyzer.cargo.loadOutDirsFromCheck": true,
    "rust-analyzer.procMacro.enable": true
}
```