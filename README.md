## Run servers locally (Docker)
```bash
./docker.sh
```

## Setup & Building
```bash
cargo install cargo-watch
cd app_service
cargo build
cd ..
cd auth_service
cargo build
cd ..
```

## Run servers locally (Manually)
#### App service
```bash
cd app_service
cargo watch -q -c -w src/ -w assets/ -w templates/ -x run
```

visit http://localhost:8000

#### Auth service
```bash
cd auth_service
cargo watch -q -c -w src/ -w assets/ -x run
```

visit http://localhost:3000

## Run servers locally (Docker)
```bash
docker compose build
docker compose up
```

visit http://localhost:8000 and http://localhost:3000