CREATE TABLE mail_addresses
(
    mail_address_id UUID         NOT NULL UNIQUE PRIMARY KEY,
    mail_id         UUID         NOT NULL,
    name            VARCHAR(200) NOT NULL,
    email           VARCHAR(200) NOT NULL,
    addr_type       VARCHAR(50)  NOT NULL,
    created_at      TIMESTAMP    NOT NULL DEFAULT CURRENT_TIMESTAMP
);

SELECT auto_handle_updated_at('mail_addresses');

ALTER TABLE mail_addresses
    ADD CONSTRAINT fk_mail_addresses_mail_id FOREIGN KEY (mail_id) REFERENCES mails (mail_id);
