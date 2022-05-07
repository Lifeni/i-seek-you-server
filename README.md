# I Seek You Server

[![Docker Image Version (latest semver)](https://img.shields.io/docker/v/lifeni/i-seek-you?label=docker%20hub)](https://hub.docker.com/r/lifeni/i-seek-you)

A WebRTC server, including signaling server and TURN server. See also [I Seek You](https://github.com/Lifeni/i-seek-you).

## Usage

#### Signaling Server

```js
// The signaling server uses WebSocket to connect.
// new WebSocket('ws://localhost:8081')
new WebSocket('wss://signaling.i-seek-you.dist.run')
```

#### TURN Server

```js
// STUN and TURN use the same server.
new RTCPeerConnection({
  iceServers: [
    // { urls: 'stun:localhost:3478' },
    { urls: 'stun:stun.i-seek-you.dist.run:3478' },
    {
      // urls: ['turn:localhost:3478'],
      urls: ['turn:turn.i-seek-you.dist.run:3478'],
      username: 'webrtc',
      credential: 'webrtc',
    },
  ],
})
```

## Development

### Build

```sh
# Run signaling server
cargo run --bin signaling
# Run TURN server
cargo run --bin turn

# Build
cargo build --release
# target/release/signaling[.exe]
# target/release/turn[.exe]
```

#### Dockerfile

```sh
docker build -t i-seek-you:local .
```

#### Docker Compose

```sh
# Recommended
docker-compose up -d

# Or
docker run -d --rm --name i-seek-you --network host lifeni/i-seek-you:latest
# Note: Declaring many ports may cause performance issues.
# docker run -d --rm --name i-seek-you -p 8081:8081 -p 3478:3478/udp -p 49152-65535:49152-65535/udp lifeni/i-seek-you:latest
```

## License

This project is licensed under either of

- Apache License, Version 2.0, (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
