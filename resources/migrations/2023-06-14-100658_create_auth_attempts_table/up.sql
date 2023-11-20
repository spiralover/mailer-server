CREATE TABLE auth_attempts
(
    auth_attempt_id          UUID          NOT NULL UNIQUE PRIMARY KEY,
    user_id                  UUID          NULL,
    email                    VARCHAR(150)  NOT NULL,
    ip_address               VARCHAR(100)  NULL,
    user_agent               VARCHAR(250)  NULL,
    auth_error               VARCHAR(1000) NULL     DEFAULT NULL,
    verification_code        VARCHAR(10)   NULL     DEFAULT NULL,
    verification_code_trials SMALLINT      NOT NULL DEFAULT 0,
    status                   VARCHAR(20)   NOT NULL DEFAULT 'success',
    created_at               TIMESTAMP     NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at               TIMESTAMP     NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at               TIMESTAMP     NULL     DEFAULT NULL
);

ALTER TABLE auth_attempts
    ADD CONSTRAINT fk_auth_attempts_user_id FOREIGN KEY (user_id) REFERENCES users (user_id);
