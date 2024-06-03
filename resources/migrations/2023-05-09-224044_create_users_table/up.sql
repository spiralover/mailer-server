CREATE TABLE users
(
    user_id                    UUID         NOT NULL UNIQUE PRIMARY KEY,
    created_by                 UUID         NULL     DEFAULT NULL,
    username                   VARCHAR(150) NOT NULL,
    first_name                 VARCHAR(150) NULL     DEFAULT NULL,
    last_name                  VARCHAR(150) NULL     DEFAULT NULL,
    email                      VARCHAR(100) NOT NULL,
    password                   VARCHAR(250) NOT NULL,
    profile_picture            VARCHAR(700) NULL     DEFAULT NULL,
    verification_code          VARCHAR(10)  NULL     DEFAULT NULL,
    verification_token         VARCHAR(50)  NULL     DEFAULT NULL,
    verified_at                TIMESTAMP    NULL     DEFAULT NULL,
    is_verified                BOOLEAN      NOT NULL DEFAULT FALSE,
    is_password_locked         BOOLEAN      NOT NULL DEFAULT FALSE,
    has_started_password_reset BOOLEAN      NOT NULL DEFAULT FALSE,
    temp_password_status       VARCHAR(50)  NOT NULL DEFAULT 'used',
    status                     VARCHAR(50)  NOT NULL DEFAULT 'active',
    created_at                 TIMESTAMP    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at                 TIMESTAMP    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at                 TIMESTAMP    NULL     DEFAULT NULL
);

SELECT auto_handle_updated_at('users');

ALTER TABLE users
    ADD CONSTRAINT fk_users_created_by FOREIGN KEY (created_by) REFERENCES users (user_id);

INSERT INTO users(user_id, is_verified, username, first_name, last_name, email, password)
VALUES ('8caadfd3-ead5-422e-991a-9ad2c90935f3', TRUE, 'system', 'System', 'Mailer', 'system@spiralover.com',
        '$argon2i$v=19$m=4096,t=3,p=1$NTFjYWYyYzc3YWNjZjU3OWJlMjExNTUxZjdiNGI1YmU$uowVYN+UsXOCZNx3JicBppmteh4zDIWvW8gc5XwmSsQ'),
       ('be6ee736-ed4d-43c9-9c91-bfd0318b875e', TRUE, 'super.admin', 'Super', 'Admin', 'super.admin@spiralover.com',
        '$argon2i$v=19$m=4096,t=3,p=1$NTFjYWYyYzc3YWNjZjU3OWJlMjExNTUxZjdiNGI1YmU$uowVYN+UsXOCZNx3JicBppmteh4zDIWvW8gc5XwmSsQ'),
       ('3b9fcf79-188c-489c-97e9-d9b57b29109b', TRUE, 'admin', 'Admin', 'SpiralOver', 'admin@spiralover.com',
        '$argon2i$v=19$m=4096,t=3,p=1$NTFjYWYyYzc3YWNjZjU3OWJlMjExNTUxZjdiNGI1YmU$uowVYN+UsXOCZNx3JicBppmteh4zDIWvW8gc5XwmSsQ'),
       ('430167fd-0b57-46e0-a184-6fe92b9658ea', TRUE, 'ahmard', 'Ahmad', 'Mustapha', 'ahmad.mustapha@spiralover.com',
        '$argon2i$v=19$m=4096,t=3,p=1$NTFjYWYyYzc3YWNjZjU3OWJlMjExNTUxZjdiNGI1YmU$uowVYN+UsXOCZNx3JicBppmteh4zDIWvW8gc5XwmSsQ'),
       ('23d10910-5bd2-4cec-b979-9bd7f21cc6d1', TRUE, 'ahmardiy', 'Ahmad', 'Mustapha', 'me@ahmard.com',
        '$argon2i$v=19$m=4096,t=3,p=1$NTFjYWYyYzc3YWNjZjU3OWJlMjExNTUxZjdiNGI1YmU$uowVYN+UsXOCZNx3JicBppmteh4zDIWvW8gc5XwmSsQ');
