CREATE TABLE file_uploads
(
    file_upload_id  UUID          NOT NULL UNIQUE PRIMARY KEY,
    uploader_id     UUID          NOT NULL,
    owner_id        UUID          NOT NULL,
    owner_type      VARCHAR(50)   NOT NULL,
    orig_name       VARCHAR(255)  NOT NULL,
    file_name       VARCHAR(255)  NOT NULL,
    file_path       VARCHAR(1000) NOT NULL,
    file_ext        VARCHAR(255)  NOT NULL,
    description     VARCHAR(250)  NULL,
    additional_info VARCHAR(250)  NULL,
    is_temp         BOOLEAN       NOT NULL DEFAULT FALSE,
    created_at      TIMESTAMP     NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at      TIMESTAMP     NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at      TIMESTAMP     NULL
);

SELECT auto_handle_updated_at('file_uploads');

ALTER TABLE file_uploads
    ADD CONSTRAINT fk_file_uploads_user_id FOREIGN KEY (uploader_id) REFERENCES users (user_id);


INSERT INTO public.file_uploads (file_upload_id, uploader_id, owner_id, owner_type, orig_name, file_name, file_path,
                                 file_ext, description, additional_info, is_temp, created_at, updated_at, deleted_at)
VALUES ('d2dcfb99-d38c-46a0-a1a3-8ec268b6a818', 'be6ee736-ed4d-43c9-9c91-bfd0318b875e',
        'be6ee736-ed4d-43c9-9c91-bfd0318b875e', 'temp', 'vlcsnap-2023-07-03-18h14m17s955.png',
        'leM59XLElivn8r4AGZ97K.png', 'static/uploads/leM59XLElivn8r4AGZ97K.png', 'png', 'temporary file', NULL, FALSE,
        '2023-08-20 12:36:47.827246', '2023-08-20 12:37:01.788392', NULL);
INSERT INTO public.file_uploads (file_upload_id, uploader_id, owner_id, owner_type, orig_name, file_name, file_path,
                                 file_ext, description, additional_info, is_temp, created_at, updated_at, deleted_at)
VALUES ('45114eb9-02cf-4054-b6b2-d691e39257d4', 'be6ee736-ed4d-43c9-9c91-bfd0318b875e',
        'be6ee736-ed4d-43c9-9c91-bfd0318b875e', 'temp', 'vlcsnap-2022-09-26-13h06m04s198.png',
        'YJCS-OcHyQLMI3ltkyyQM.png', 'static/uploads/YJCS-OcHyQLMI3ltkyyQM.png', 'png', 'temporary file', NULL, TRUE,
        '2023-08-20 12:42:31.967458', '2023-08-20 12:42:31.967581', NULL);
