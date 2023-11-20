CREATE TABLE user_roles
(
    user_role_id UUID      NOT NULL UNIQUE PRIMARY KEY,
    created_by   UUID      NOT NULL,
    role_id      UUID      NOT NULL,
    user_id      UUID      NOT NULL,
    created_at   TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at   TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at   TIMESTAMP NULL     DEFAULT NULL
);

CREATE INDEX user_roles_role_id_index ON user_roles (role_id);
CREATE INDEX user_roles_user_id_index ON user_roles (user_id);

ALTER TABLE user_roles
    ADD CONSTRAINT fk_user_roles_created_by FOREIGN KEY (created_by) REFERENCES users (user_id);

ALTER TABLE user_roles
    ADD CONSTRAINT fk_user_roles_role_id FOREIGN KEY (role_id) REFERENCES roles (role_id);

ALTER TABLE user_roles
    ADD CONSTRAINT fk_user_roles_user_id FOREIGN KEY (user_id) REFERENCES users (user_id);
