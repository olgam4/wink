run:
  cargo watch -x run

build:
  docker build -t wink .

docker:
  docker run -p 8000:8000 --rm --name wink0 wink
