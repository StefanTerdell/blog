CREATE TABLE IF NOT EXISTS blog_post_assets (
    id SERIAL PRIMARY KEY,
    file_name TEXT NOT NULL UNIQUE,
    data BYTEA NOT NULL,
    blog_post_id INTEGER NOT NULL,

    FOREIGN KEY (blog_post_id) REFERENCES blog_posts(id) ON DELETE CASCADE
)
