CREATE TABLE user_apps
(
    user_app_id    UUID        NOT NULL UNIQUE PRIMARY KEY,
    created_by     UUID        NOT NULL,
    user_id        UUID        NOT NULL,
    application_id UUID        NOT NULL,
    comment        VARCHAR(1000),
    status         VARCHAR(50) NOT NULL DEFAULT 'active',
    created_at     TIMESTAMP   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at     TIMESTAMP   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at     TIMESTAMP   NULL     DEFAULT NULL
);

ALTER TABLE user_apps
    ADD CONSTRAINT fk_user_apps_created_by FOREIGN KEY (created_by) REFERENCES users (user_id);

ALTER TABLE user_apps
    ADD CONSTRAINT fk_user_apps_user_id FOREIGN KEY (user_id) REFERENCES users (user_id);

ALTER TABLE user_apps
    ADD CONSTRAINT fk_user_apps_application_id FOREIGN KEY (application_id) REFERENCES applications (application_id);
