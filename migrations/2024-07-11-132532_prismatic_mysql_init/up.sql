CREATE TABLE user (
    user_id VARCHAR(255) NOT NULL,
    human_diary TEXT,
    ai_diary_1 TEXT,
    ai_diary_2 TEXT,
    ai_diary_3 TEXT,
    ai_diary_4 TEXT,
    is_public BOOLEAN,
    favorite_id INT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    PRIMARY KEY (user_id)
);
