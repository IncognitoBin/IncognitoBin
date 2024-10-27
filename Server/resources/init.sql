
CREATE KEYSPACE IF NOT EXISTS incognitobin_keyspace
WITH replication = {
    'class': 'SimpleStrategy',
    'replication_factor': 1
};

USE incognitobin_keyspace;

CREATE TABLE IF NOT EXISTS user_by_id (
    user_id UUID PRIMARY KEY,
    user_token TEXT
);

CREATE TABLE IF NOT EXISTS user_by_token (
    user_token TEXT PRIMARY KEY,
    user_id UUID
) WITH default_time_to_live = 86400;


CREATE TABLE IF NOT EXISTS paste_by_id (
    paste_id UUID PRIMARY KEY,
    title TEXT,
    signature TEXT,
    content TEXT,
    syntax TEXT,
    expire TIMESTAMP,
    burn BOOLEAN,
    user_id UUID
);

CREATE TABLE IF NOT EXISTS pastes_by_user_id (
    user_id UUID,
    paste_id UUID,
    PRIMARY KEY (user_id, paste_id)
);

CREATE TABLE IF NOT EXISTS paste_view_counts (
    paste_id UUID PRIMARY KEY,
    view_count COUNTER
);
