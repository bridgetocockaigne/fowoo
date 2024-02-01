CREATE TABLE IF NOT EXISTS users
(
    id    xBIGSERIAL PRIMARY KEY,
    i
    email TEXT NOT NULL,
    CONSTRAINT unique_email UNIQUE(email)
);
