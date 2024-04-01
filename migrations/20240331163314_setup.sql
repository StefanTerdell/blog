CREATE TABLE IF NOT EXISTS work (
    id SERIAL PRIMARY KEY,
    company TEXT NOT NULL,
    job_title TEXT NOT NULL,
    description TEXT NOT NULL,
    technologies TEXT[] NOT NULL, 
    from_date BIGINT NOT NULL,
    to_date BIGINT 
);

CREATE TABLE IF NOT EXISTS blog_post (
    id SERIAL PRIMARY KEY,
    title TEXT NOT NULL,
    content TEXT NOT NULL,
    published BOOLEAN NOT NULL,
    published_timestamp BIGINT NOT NULL,
    edited_timestamp BIGINT 
);

CREATE TABLE IF NOT EXISTS guestbook_post (
    id SERIAL PRIMARY KEY,
    user_id TEXT NOT NULL,
    user_name TEXT NOT NULL,
    user_url TEXT NOT NULL,
    content TEXT NOT NULL,
    published BOOLEAN NOT NULL,
    published_timestamp BIGINT NOT NULL,
    edited_timestamp BIGINT 
);
