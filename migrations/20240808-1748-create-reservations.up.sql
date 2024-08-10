--- Creates the "reservation" table

CREATE TABLE reservation (
    client_id VARCHAR(256) NOT NULL PRIMARY KEY,
    ip_address INTEGER NOT NULL,
    scope INTEGER NOT NULL,
    description VARCHAR(4096),

    FOREIGN KEY (scope) REFERENCES scope(id) ON UPDATE CASCADE ON DELETE CASCADE
);