use crate::LDHCPD;

mod v1;

router::router!(pub route, &'b LDHCPD, (), [], [b"v1" => v1::route], []);
