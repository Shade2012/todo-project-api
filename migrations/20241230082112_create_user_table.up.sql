-- Add up migration script here
CREATE TABLE IF NOT EXISTS users (
    id INT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(255) NOT NULL UNIQUE,         -- Use VARCHAR instead of TINYTEXT for better indexing
    password VARCHAR(255) NOT NULL,            -- Enforce NOT NULL if passwords are mandatory
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);
