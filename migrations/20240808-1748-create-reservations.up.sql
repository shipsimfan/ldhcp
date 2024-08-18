--- Creates the "reservation" table

CREATE TABLE reservation (
    id INTEGER PRIMARY KEY,
    client_id VARCHAR(256) NOT NULL UNIQUE,
    ip_address INTEGER NOT NULL,
    scope INTEGER NOT NULL UNIQUE,
    renewal_time INTEGER NOT NULL,
    description VARCHAR(4096),

    FOREIGN KEY (scope) REFERENCES scope(id) ON UPDATE CASCADE ON DELETE CASCADE
);