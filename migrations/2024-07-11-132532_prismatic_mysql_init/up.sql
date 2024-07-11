CREATE TABLE diaries (
    user_id CHAR(36) NOT NULL,
    human_diary TEXT,
    AI_diary_1 TEXT,
    AI_diary_2 TEXT,
    AI_diary_3 TEXT,
    AI_diary_4 TEXT,
    isPublic BOOLEAN,
    favorite_id INT,
    PRIMARY KEY (user_id)       
);
