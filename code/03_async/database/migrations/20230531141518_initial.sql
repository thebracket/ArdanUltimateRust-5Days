-- Create a messages table
CREATE TABLE IF NOT EXISTS messages
(
    id          INTEGER PRIMARY KEY NOT NULL,
    message     TEXT                NOT NULL
);

--- Insert some test messages
INSERT INTO messages (id, message) VALUES (1, 'Hello World!');
INSERT INTO messages (id, message) VALUES (2, 'Hello Galaxy!');
INSERT INTO messages (id, message) VALUES (3, 'Hello Universe!');