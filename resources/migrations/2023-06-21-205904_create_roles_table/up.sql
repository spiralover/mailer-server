CREATE TABLE roles
(
    role_id    UUID         NOT NULL UNIQUE PRIMARY KEY,
    created_by UUID         NOT NULL,
    role_name  VARCHAR(250) NOT NULL,
    guard_name VARCHAR(250) NOT NULL,
    status     VARCHAR(50)  NOT NULL DEFAULT 'active',
    created_at TIMESTAMP    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP    NULL     DEFAULT NULL
);

ALTER TABLE roles
    ADD CONSTRAINT fk_roles_created_by FOREIGN KEY (created_by) REFERENCES users (user_id);
