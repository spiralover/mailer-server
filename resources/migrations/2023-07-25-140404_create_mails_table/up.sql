CREATE TABLE mails
(
    mail_id         UUID          NOT NULL UNIQUE PRIMARY KEY,
    created_by      UUID          NOT NULL,
    application_id  UUID          NOT NULL,
    subject         VARCHAR(1000) NOT NULL,
    message         TEXT          NOT NULL,
    from_name       VARCHAR(200)  NOT NULL,
    from_email      VARCHAR(200)  NOT NULL,
    reply_to_name   VARCHAR(200)  NULL,
    reply_to_email  VARCHAR(200)  NULL,
    trials          SMALLINT      NOT NULL DEFAULT 0,
    status          VARCHAR(50)   NOT NULL DEFAULT 'active',
    sent_at         TIMESTAMP     NULL     DEFAULT NULL,
    next_retrial_at TIMESTAMP     NULL     DEFAULT NULL,
    created_at      TIMESTAMP     NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at      TIMESTAMP     NOT NULL DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE mails
    ADD CONSTRAINT fk_mails_created_by FOREIGN KEY (created_by) REFERENCES users (user_id);

ALTER TABLE mails
    ADD CONSTRAINT fk_mails_application_id FOREIGN KEY (application_id) REFERENCES applications (application_id);
