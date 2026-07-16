-- Add migration script here
CREATE TABLE IF NOT EXISTS tasks(
    id SERIAL PRIMARY KEY,
    name VARCHAR(250) NOT NULL,
    description VARCHAR(500),
    start_date DATE,
    end_date DATE,
    priority_score INT,
    status VARCHAR(20) NOT NULL DEFAULT 'pending',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    user_id int NOT NULL,
    CONSTRAINT fk_user FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);