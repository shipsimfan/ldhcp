use crate::LDHCPD;

mod post;

router::router!(pub route, &'b LDHCPD, (), [POST => post::post], [], []);
