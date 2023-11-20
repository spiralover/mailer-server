CREATE TABLE notifications
(
    notification_id UUID          NOT NULL UNIQUE PRIMARY KEY,
    receiver_id     UUID          NOT NULL,
    title           VARCHAR(250)  NOT NULL,
    url             VARCHAR(3000) NOT NULL,
    content         VARCHAR(6000) NOT NULL,
    status          VARCHAR(30)   NOT NULL,
    created_at      TIMESTAMP     NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at      TIMESTAMP     NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at      TIMESTAMP     NULL     DEFAULT NULL
);

ALTER TABLE notifications
    ADD CONSTRAINT fk_notifications_receiver_id FOREIGN KEY (receiver_id) REFERENCES users (user_id);
