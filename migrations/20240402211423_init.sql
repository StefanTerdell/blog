CREATE TABLE IF NOT EXISTS blog_posts (
    id SERIAL PRIMARY KEY,

    slug TEXT NOT NULL UNIQUE,
    title TEXT NOT NULL,
    content TEXT NOT NULL,
    published BOOLEAN NOT NULL,
    published_time BIGINT NOT NULL,
    views INT NOT NULL,
    edited_time BIGINT 
);

CREATE TABLE IF NOT EXISTS github_users (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    url TEXT NOT NULL,
    admin BOOLEAN NOT NULL
);

CREATE TABLE IF NOT EXISTS github_tokens (
    access_token TEXT NOT NULL PRIMARY KEY,
    user_id INTEGER NOT NULL UNIQUE,
    created_at BIGINT NOT NULL,

    FOREIGN KEY (user_id) REFERENCES github_users(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS guestbook_posts (
    id SERIAL PRIMARY KEY,

    user_id INTEGER NOT NULL,
    content TEXT NOT NULL,
    published BOOLEAN NOT NULL,
    created_time BIGINT NOT NULL,
    edited_time BIGINT, 

    FOREIGN KEY (user_id) REFERENCES github_users(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS csrf_tokens (
    csrf_token TEXT NOT NULL PRIMARY KEY UNIQUE,
    created_at BIGINT NOT NULL
);

