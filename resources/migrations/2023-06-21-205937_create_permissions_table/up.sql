CREATE TABLE permissions
(
    permission_id   UUID         NOT NULL UNIQUE PRIMARY KEY,
    created_by      UUID         NOT NULL,
    permission_name VARCHAR(250) NOT NULL,
    guard_name      VARCHAR(250) NOT NULL,
    created_at      TIMESTAMP    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at      TIMESTAMP    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at      TIMESTAMP    NULL     DEFAULT NULL
);

SELECT auto_handle_updated_at('permissions');

ALTER TABLE permissions
    ADD CONSTRAINT fk_permissions_created_by FOREIGN KEY (created_by) REFERENCES users (user_id);
