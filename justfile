run:
  cargo watch -x run

build:
  docker build -t wink .

docker:
  docker run -p 8000:8000 --rm --name wink0 wink

postgres:
  docker run -p 5432:5432 --rm --name pgsql-wink-dev -e POSTGRES_PASSWORD=Welcome4$ postgres

migrate:
  cargo sqlx migrate run
