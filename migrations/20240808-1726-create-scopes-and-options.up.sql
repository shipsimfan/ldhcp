--- Creates the "scope" and "option" tables and the connection between them

CREATE TABLE scope_type (
    id INTEGER NOT NULL PRIMARY KEY,
    name VARCHAR(16) NOT NULL UNIQUE
);

INSERT INTO scope_type (name) VALUES ("Global");
INSERT INTO scope_type (name) VALUES ("Reservation");


CREATE TABLE scope (
    id INTEGER NOT NULL PRIMARY KEY,
    type INTEGER NOT NULL,

    FOREIGN KEY (type) REFERENCES scope_type(id) ON UPDATE CASCADE ON DELETE RESTRICT
);

INSERT INTO scope (type) VALUES (1);


CREATE TABLE option (
    scope INTEGER NOT NULL,
    number INTEGER NOT NULL,
    content VARCHAR(256),

    FOREIGN KEY (scope) REFERENCES scope(id)
);