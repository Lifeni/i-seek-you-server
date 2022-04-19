# I Seek You Server

![GitHub](https://img.shields.io/github/license/Lifeni/i-seek-you-server)
![Docker Image Version (latest semver)](https://img.shields.io/docker/v/lifeni/i-seek-you)

A WebRTC server, see also [I Seek You](https://github.com/Lifeni/i-seek-you).

## Development

```sh
# Run signaling server
cargo run --bin signaling
# Run STUN server
cargo run --bin stun
# Run TURN server
cargo run --bin turn
```

## Build

Tested on Rust `1.59.0`.

```sh
cargo build --release
# target/release/signaling[.exe]
# target/release/stun[.exe]
# target/release/turn[.exe]
```

### Dockerfile

```sh
docker build -t i-seek-you:local .
```

You can also download it from [Docker Hub](https://hub.docker.com/r/lifeni/i-seek-you).

```docker
# Recommended
docker-compose up -d
# Or
docker run -d --rm --name i-seek-you -p 8081:8081 -p 8082:8082 -p 8083:8083 lifeni/i-seek-you:latest
```

## License

MIT License
