CREATE TABLE app_keys
(
    app_key_id     UUID         NOT NULL UNIQUE PRIMARY KEY,
    created_by     UUID         NOT NULL,
    application_id UUID         NOT NULL,
    public_key     VARCHAR(500) NOT NULL,
    private_key    VARCHAR(500) NOT NULL,
    status         VARCHAR(20)  NOT NULL DEFAULT 'active',
    created_at     TIMESTAMP    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at     TIMESTAMP    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at     TIMESTAMP    NULL     DEFAULT NULL
);

SELECT auto_handle_updated_at('app_keys');

ALTER TABLE app_keys
    ADD CONSTRAINT fk_app_keys_created_by FOREIGN KEY (created_by) REFERENCES users (user_id);

ALTER TABLE app_keys
    ADD CONSTRAINT fk_app_keys_app_id FOREIGN KEY (application_id) REFERENCES applications (application_id);

INSERT INTO app_keys (app_key_id, created_by, application_id, public_key, private_key)
VALUES ('8ee2941e-9068-408b-b3ba-3bf3a5c323e0', 'be6ee736-ed4d-43c9-9c91-bfd0318b875e',
        '2eb91dc3-b8ad-4d41-a207-963cec055fab',
        'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiIyZWI5MWRjMy1iOGFkLTRkNDEtYTIwNy05NjNjZWMwNTVmYWEiLCJpYXQiOjE2ODY4Mjg4NDksImV4cCI6MTY4OTQyMDg0OX0.f2E41kSGi91QjJY4M_NynI8-IRG9P_RsFhjg5Tza9hs',
        'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzUxMiJ9.eyJzdWIiOiIyZWI5MWRjMy1iOGFkLTRkNDEtYTIwNy05NjNjZWMwNTVmYWEiLCJpYXQiOjE2ODY4Mjg4NDksImV4cCI6MTY4OTQyMDg0OX0.ozb_Wk8Ml0weza7uS1Rdig90C4vp8KTZ-AS5KXJu3814MkUOjfQHWsvhvTN4WR5uiMmQjZDXBJi0te-Ej99GYw'),
       ('8ee2941e-9068-408b-b3ba-3bf3a5c323f0', 'be6ee736-ed4d-43c9-9c91-bfd0318b875e',
        '2eb91dc3-b8ad-4d41-a207-963cec055fac',
        'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiIyZWI5MWRjMy1iOGFkLTRkNDEtYTIwNy05NjNjZWMwNTVmYWEiLCJpYXQiOjE2ODY4Mjg4NDksImV4cCI6MTY4OTQyMDg0OX0.f2E41kSGi91QjJY4M_NynI8-IRG9P_RsFhjg5Tza9hs',
        'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzUxMiJ9.eyJzdWIiOiIyZWI5MWRjMy1iOGFkLTRkNDEtYTIwNy05NjNjZWMwNTVmYWEiLCJpYXQiOjE2ODY4Mjg4NDksImV4cCI6MTY4OTQyMDg0OX0.ozb_Wk8Ml0weza7uS1Rdig90C4vp8KTZ-AS5KXJu3814MkUOjfQHWsvhvTN4WR5uiMmQjZDXBJi0te-Ej99GYw');
