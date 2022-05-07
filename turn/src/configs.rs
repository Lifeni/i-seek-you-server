pub struct Configs<'a> {
    pub public_ip: &'a str,
    pub port: usize,
    pub realm: &'a str,
    pub username: &'a str,
    pub password: &'a str,
}

pub fn get_configs() -> Configs<'static> {
    Configs {
        public_ip: "81.68.166.119",
        port: 3478,
        realm: "i-seek-you.dist.run",
        username: "webrtc",
        password: "webrtc",
    }
}
