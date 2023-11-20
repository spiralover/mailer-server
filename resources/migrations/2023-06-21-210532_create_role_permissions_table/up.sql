CREATE TABLE role_permissions
(
    role_permission_id UUID      NOT NULL UNIQUE PRIMARY KEY,
    created_by         UUID      NOT NULL,
    role_id            UUID      NOT NULL,
    permission_id      UUID      NOT NULL,
    created_at         TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at         TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at         TIMESTAMP NULL     DEFAULT NULL
);

CREATE INDEX role_permissions_role_index ON role_permissions (role_id);
CREATE INDEX role_permissions_permission_id_index ON role_permissions (permission_id);

ALTER TABLE role_permissions
    ADD CONSTRAINT fk_role_permissions_created_by FOREIGN KEY (created_by) REFERENCES users (user_id);

ALTER TABLE role_permissions
    ADD CONSTRAINT fk_role_permissions_role_id FOREIGN KEY (role_id) REFERENCES roles (role_id);

ALTER TABLE role_permissions
    ADD CONSTRAINT fk_role_permissions_permission_id FOREIGN KEY (permission_id) REFERENCES permissions (permission_id);
