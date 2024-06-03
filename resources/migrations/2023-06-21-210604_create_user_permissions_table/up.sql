CREATE TABLE user_permissions
(
    user_permission_id UUID      NOT NULL UNIQUE PRIMARY KEY,
    created_by         UUID      NOT NULL,
    user_id            UUID      NOT NULL,
    permission_id      UUID      NOT NULL,
    created_at         TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at         TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at         TIMESTAMP NULL     DEFAULT NULL
);

SELECT auto_handle_updated_at('user_permissions');

CREATE INDEX user_permissions_permission_id_index ON user_permissions (permission_id);
CREATE INDEX user_permissions_user_id_index ON user_permissions (user_id);

ALTER TABLE user_permissions
    ADD CONSTRAINT fk_user_permissions_created_by FOREIGN KEY (created_by) REFERENCES users (user_id);

ALTER TABLE user_permissions
    ADD CONSTRAINT fk_user_permissions_permission_id FOREIGN KEY (permission_id) REFERENCES permissions (permission_id);

ALTER TABLE user_permissions
    ADD CONSTRAINT fk_user_permissions_user_id FOREIGN KEY (user_id) REFERENCES users (user_id);
