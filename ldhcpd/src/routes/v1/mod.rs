use crate::LDHCPD;

mod reservations;

router::router!(pub route, &'b LDHCPD, (), [], [b"reservations" => reservations::route], []);
