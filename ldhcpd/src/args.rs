router::arguments! {
    pub PARSER -> Options.router
    "LDHCPD"
    "A dynamic host configuration protocol server"
    []
}

#[derive(Default)]
pub struct Options {
    router: router::Options,
}
