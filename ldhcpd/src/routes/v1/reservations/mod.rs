use crate::LDHCPD;

mod post;

router::router!(pub route, LDHCPD, (), [POST => post::post], [], []);
