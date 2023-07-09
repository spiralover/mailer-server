CREATE TABLE mails
(
    id             UUID         NOT NULL UNIQUE PRIMARY KEY,
    app            VARCHAR(200) NOT NULL,
    subject        VARCHAR(200) NOT NULL,
    receiver_name  VARCHAR(200) NOT NULL,
    receiver_email VARCHAR(200) NOT NULL,
    message        TEXT         NOT NULL,
    reply_to_name  VARCHAR(200) NULL,
    reply_to_email VARCHAR(200) NULL,
    cc             JSON         NULL,
    bcc            JSON         NULL,
    sent_at        TIMESTAMP    NOT NULL DEFAULT CURRENT_TIMESTAMP
)