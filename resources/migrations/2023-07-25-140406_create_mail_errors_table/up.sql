CREATE TABLE mail_errors
(
    mail_error_id UUID      NOT NULL UNIQUE PRIMARY KEY,
    mail_id       UUID      NOT NULL,
    smtp_error    TEXT      NOT NULL,
    created_at    TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE mail_errors
    ADD CONSTRAINT fk_mail_errors_mail_id FOREIGN KEY (mail_id) REFERENCES mails (mail_id);
