CREATE TABLE announcements
(
    announcement_id UUID         NOT NULL UNIQUE PRIMARY KEY,
    sender_id       UUID         NOT NULL,
    title           VARCHAR(150) NOT NULL,
    message         TEXT         NOT NULL,
    created_at      TIMESTAMP    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at      TIMESTAMP    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at      TIMESTAMP    NULL     DEFAULT NULL
);

SELECT auto_handle_updated_at('announcements');

ALTER TABLE announcements
    ADD CONSTRAINT fk_announcements_user_id FOREIGN KEY (sender_id) REFERENCES users (user_id);
