# Run local development API build in `watch` mode with hot-reload
dev: 
  cargo watch -x "run"

dev_docker:
  docker compose build
  docker compose up