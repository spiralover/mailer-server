CREATE TABLE password_resets
(
    password_reset_id UUID         NOT NULL UNIQUE PRIMARY KEY,
    user_id           UUID         NOT NULL,
    email             VARCHAR(100) NOT NULL,
    token             VARCHAR(100) NOT NULL,
    ip_address        VARCHAR(50)  NULL,
    user_agent        VARCHAR(500) NULL,
    status            VARCHAR(30)  NOT NULL,
    created_at        TIMESTAMP    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at        TIMESTAMP    NOT NULL DEFAULT CURRENT_TIMESTAMP
);

SELECT auto_handle_updated_at('password_resets');

ALTER TABLE password_resets
    ADD CONSTRAINT fk_password_resets_user_id FOREIGN KEY (user_id) REFERENCES users (user_id);
