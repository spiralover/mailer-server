CREATE TABLE personal_access_tokens
(
    pat_id     UUID          NOT NULL UNIQUE PRIMARY KEY,
    user_id    UUID          NOT NULL,
    title      VARCHAR(1000) NOT NULL,
    comment    VARCHAR(1500) NOT NULL,
    token      VARCHAR(1500) NOT NULL,
    status     VARCHAR(50)   NOT NULL DEFAULT 'active',
    expired_at TIMESTAMP     NOT NULL,
    created_at TIMESTAMP     NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP     NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP     NULL     DEFAULT NULL
);

SELECT auto_handle_updated_at('personal_access_tokens');

ALTER TABLE personal_access_tokens
    ADD CONSTRAINT fk_personal_access_tokens_user_id FOREIGN KEY (user_id) REFERENCES users (user_id);
