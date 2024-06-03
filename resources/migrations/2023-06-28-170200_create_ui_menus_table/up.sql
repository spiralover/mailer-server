CREATE TABLE ui_menus
(
    ui_menu_id  UUID          NOT NULL UNIQUE PRIMARY KEY,
    created_by  UUID          NOT NULL,
    m_name      VARCHAR(50)   NOT NULL,
    m_priority  INTEGER       NOT NULL,
    m_desc      VARCHAR(250)  NULL,
    m_url       VARCHAR(3000) NULL,
    m_has_items BOOLEAN       NOT NULL DEFAULT FALSE,
    created_at  TIMESTAMP     NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at  TIMESTAMP     NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at  TIMESTAMP     NULL     DEFAULT NULL
);

SELECT auto_handle_updated_at('ui_menus');

ALTER TABLE ui_menus
    ADD CONSTRAINT fk_ui_menus_created_by FOREIGN KEY (created_by) REFERENCES users (user_id);


INSERT INTO public.ui_menus (ui_menu_id, created_by, m_name, m_priority, m_desc, m_url, m_has_items, created_at,
                             updated_at, deleted_at)
VALUES ('7bdbbc14-4846-4778-b3ca-6f83c93dd8e3', 'be6ee736-ed4d-43c9-9c91-bfd0318b875e', 'Neurons', 2,
        'Application Management', '/neurons', FALSE, '2023-06-29 01:05:33.606599', '2023-06-29 01:05:33.606732', NULL);
INSERT INTO public.ui_menus (ui_menu_id, created_by, m_name, m_priority, m_desc, m_url, m_has_items, created_at,
                             updated_at, deleted_at)
VALUES ('e65c96bb-9f5b-4223-b755-ea46702eddc2', 'be6ee736-ed4d-43c9-9c91-bfd0318b875e', 'Staff Management', 3,
        'User Management', '/staffs', FALSE, '2023-06-29 03:11:09.786460', '2023-06-29 03:11:09.786543', NULL);
INSERT INTO public.ui_menus (ui_menu_id, created_by, m_name, m_priority, m_desc, m_url, m_has_items, created_at,
                             updated_at, deleted_at)
VALUES ('bf5d6a18-4990-4fd7-8454-d6de6aee2521', 'be6ee736-ed4d-43c9-9c91-bfd0318b875e', 'Notifications', 100,
        'Notifications Module', '/notifications', FALSE, '2023-10-29 01:18:14.849549', '2023-10-29 01:18:14.849615',
        NULL);
INSERT INTO public.ui_menus (ui_menu_id, created_by, m_name, m_priority, m_desc, m_url, m_has_items, created_at,
                             updated_at, deleted_at)
VALUES ('7400b6aa-b248-4264-bff6-1723b44605c6', 'be6ee736-ed4d-43c9-9c91-bfd0318b875e', 'Announcements', 7,
        'Announcements Module', '', TRUE, '2023-07-25 11:22:16.458383', '2023-07-25 11:22:16.458477', NULL);
INSERT INTO public.ui_menus (ui_menu_id, created_by, m_name, m_priority, m_desc, m_url, m_has_items, created_at,
                             updated_at, deleted_at)
VALUES ('476d12b5-6d61-4d7c-8605-c24159b79689', 'be6ee736-ed4d-43c9-9c91-bfd0318b875e', 'System Settings', 9,
        'System Settings Management', '', TRUE, '2023-07-17 00:48:06.115064', '2023-07-17 00:48:06.115064', NULL);
