# I Seek You Server

![GitHub](https://img.shields.io/github/license/Lifeni/i-seek-you-server)
![Docker Image Version (latest semver)](https://img.shields.io/docker/v/lifeni/i-seek-you)

A WebRTC server, see also [I Seek You](https://github.com/Lifeni/i-seek-you).

## Development

```sh
cargo run # default listening 0.0.0.0:8081
cargo run 127.0.0.1:8081
```

## Build

Tested on Rust `1.59.0`.

```sh
cargo build --release
# target/release/i-seek-you[.exe]
```

### Dockerfile

```sh
docker build -t i-seek-you:local .
```

Or download from [Docker Hub](https://hub.docker.com/r/lifeni/i-seek-you).

```docker
docker run --rm --name i-seek-you -p:8081:8081 i-seek-you:latest
```

## License

MIT License
