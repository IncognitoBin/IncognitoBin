
CREATE KEYSPACE IF NOT EXISTS xsnippet_keyspace
WITH replication = {
    'class': 'SimpleStrategy',
    'replication_factor': 1
};

USE xsnippet_keyspace;

CREATE TABLE IF NOT EXISTS user_by_id (
    user_id UUID PRIMARY KEY,
    username TEXT,
    user_token TEXT
);

CREATE TABLE IF NOT EXISTS user_by_token (
    user_token TEXT PRIMARY KEY,
    user_id UUID
);

CREATE TABLE IF NOT EXISTS paste_by_id (
    paste_id UUID PRIMARY KEY,
    title TEXT,
    content TEXT,
    syntax TEXT,
    password TEXT,
    encrypted BOOLEAN,
    expire TIMESTAMP,
    burn BOOLEAN,
    user_id UUID
);

CREATE TABLE IF NOT EXISTS pastes_by_user_id (
    user_id UUID,
    paste_id UUID,
    PRIMARY KEY (user_id, paste_id)
);

CREATE TABLE IF NOT EXISTS expire_date (
    year INT,
    month INT,
    day INT,
    hour INT,
    minute INT,
    PRIMARY KEY (year, month, day, hour, minute)
);
