use crate::LDHCPD;

mod reservations;

router::router!(pub route, LDHCPD, (), [], [b"reservations" => reservations::route], []);
