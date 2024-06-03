CREATE TABLE applications
(
    application_id UUID          NOT NULL UNIQUE PRIMARY KEY,
    created_by     UUID          NOT NULL,
    name           VARCHAR(150)  NOT NULL,
    code           VARCHAR(100)  NOT NULL,
    url            VARCHAR(100)  NOT NULL,
    logo           VARCHAR(250)  NOT NULL,
    webhook        VARCHAR(1000) NOT NULL DEFAULT NULL,
    description    VARCHAR(1000) NOT NULL,
    status         VARCHAR(50)   NOT NULL DEFAULT 'active',
    created_at     TIMESTAMP     NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at     TIMESTAMP     NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at     TIMESTAMP     NULL     DEFAULT NULL
);

SELECT auto_handle_updated_at('applications');

ALTER TABLE applications
    ADD CONSTRAINT fk_applications_created_by FOREIGN KEY (created_by) REFERENCES users (user_id);

INSERT INTO applications(application_id, created_by, name, code, url, webhook, logo, description)
VALUES ('eb57d419-4f13-4c0c-ac78-7ce6124957b3', 'be6ee736-ed4d-43c9-9c91-bfd0318b875e', 'Mailer',
        'spiralover.apps.mailer', 'https://mailer.spiralover.com',
        'https://mailer.spiralover.com/webhooks/v1/callback', '', 'This Mailer Instance Application'),
       ('2eb91dc3-b8ad-4d41-a207-963cec055fab', 'be6ee736-ed4d-43c9-9c91-bfd0318b875e', 'Expenses',
        'spiralover.apps.expenses', 'https://expenses.spiralover.com',
        'https://expenses.spiralover.com/webhooks/v1/accounts', '', 'Expenses Management Software'),
       ('2eb91dc3-b8ad-4d41-a207-963cec055fac', 'be6ee736-ed4d-43c9-9c91-bfd0318b875e', 'FMS',
        'spiralover.apps.fms', 'https://fms.spiralover.com',
        'https://fms.spiralover.com/webhooks/v1/accounts', '', 'File Management Software');