use crate::LDHCPD;

mod get;
mod post;

router::router!(pub route, LDHCPD, (), [POST => post::post], [], []);
