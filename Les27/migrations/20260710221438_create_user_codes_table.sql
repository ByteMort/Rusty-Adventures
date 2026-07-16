-- Add migration script here
CREATE TABLE IF NOT EXISTS user_codes(
    id SERIAL PRIMARY KEY,
    code VARCHAR(25) NOT NULL,
    expires_at TIMESTAMP WITH TIME ZONE NOT NULL,
    user_id int NOT NULL,
    CONSTRAINT fk_user FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);
