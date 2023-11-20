CREATE TABLE ui_menu_items
(
    ui_menu_item_id UUID          NOT NULL UNIQUE PRIMARY KEY,
    created_by      UUID          NOT NULL,
    ui_menu_id      UUID          NOT NULL,
    mi_name         VARCHAR(50)   NOT NULL,
    mi_priority     INTEGER       NOT NULL,
    mi_desc         VARCHAR(250)  NULL,
    mi_url          VARCHAR(3000) NOT NULL,
    created_at      TIMESTAMP     NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at      TIMESTAMP     NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at      TIMESTAMP     NULL     DEFAULT NULL
);

ALTER TABLE ui_menu_items
    ADD CONSTRAINT fk_ui_menu_items_created_by FOREIGN KEY (created_by) REFERENCES users (user_id);

ALTER TABLE ui_menu_items
    ADD CONSTRAINT fk_ui_menu_items_ui_menu_id FOREIGN KEY (ui_menu_id) REFERENCES ui_menus (ui_menu_id);


INSERT INTO public.ui_menu_items (ui_menu_item_id, created_by, ui_menu_id, mi_name, mi_priority, mi_desc, mi_url, created_at, updated_at, deleted_at) VALUES ('ded29a81-d73a-40c4-a514-907a4e2e8f66', 'be6ee736-ed4d-43c9-9c91-bfd0318b875e', '7bdbbc14-4846-4778-b3ca-6f83c93dd8e3', 'Applications', 1, 'List of neurons', '/neurons', '2023-06-29 11:46:19.451417', '2023-06-29 11:46:19.451533', null);
INSERT INTO public.ui_menu_items (ui_menu_item_id, created_by, ui_menu_id, mi_name, mi_priority, mi_desc, mi_url, created_at, updated_at, deleted_at) VALUES ('c4b9103e-0359-4772-a929-9069876b79ed', 'be6ee736-ed4d-43c9-9c91-bfd0318b875e', 'e65c96bb-9f5b-4223-b755-ea46702eddc2', 'User Management', 1, 'Users list', '/users', '2023-06-29 11:46:42.119534', '2023-06-29 11:46:42.119607', null);
INSERT INTO public.ui_menu_items (ui_menu_item_id, created_by, ui_menu_id, mi_name, mi_priority, mi_desc, mi_url, created_at, updated_at, deleted_at) VALUES ('bb6eed4f-9fb4-49ec-bb46-e706dbcf6fc9', 'be6ee736-ed4d-43c9-9c91-bfd0318b875e', 'bf5d6a18-4990-4fd7-8454-d6de6aee2521', 'Notifications', 1, 'Notifications Module', '/notifications', '2023-06-29 11:48:21.316804', '2023-06-29 11:48:21.316823', null);
INSERT INTO public.ui_menu_items (ui_menu_item_id, created_by, ui_menu_id, mi_name, mi_priority, mi_desc, mi_url, created_at, updated_at, deleted_at) VALUES ('553d0dfe-d652-4851-998c-be4d817e0dfa', 'be6ee736-ed4d-43c9-9c91-bfd0318b875e', '476d12b5-6d61-4d7c-8605-c24159b79689', 'Roles', 1, 'Roles Management', '/system/roles', '2023-06-29 11:47:57.438493', '2023-06-29 11:47:57.438515', null);
INSERT INTO public.ui_menu_items (ui_menu_item_id, created_by, ui_menu_id, mi_name, mi_priority, mi_desc, mi_url, created_at, updated_at, deleted_at) VALUES ('4088b50f-0135-4eec-8645-388d7c56de73', 'be6ee736-ed4d-43c9-9c91-bfd0318b875e', '476d12b5-6d61-4d7c-8605-c24159b79689', 'Menu Management', 4, 'List of menus', '/system/menus', '2023-07-17 11:47:00.634056', '2023-07-17 11:47:00.634124', null);
INSERT INTO public.ui_menu_items (ui_menu_item_id, created_by, ui_menu_id, mi_name, mi_priority, mi_desc, mi_url, created_at, updated_at, deleted_at) VALUES ('ffa42eee-2266-4059-9f58-1d2c8f6bd043', 'be6ee736-ed4d-43c9-9c91-bfd0318b875e', '7400b6aa-b248-4264-bff6-1723b44605c6', 'List Sent Announcements', 2, 'List of send announcements', '/announcements', '2023-07-25 11:23:44.309390', '2023-07-25 11:23:44.309507', null);
INSERT INTO public.ui_menu_items (ui_menu_item_id, created_by, ui_menu_id, mi_name, mi_priority, mi_desc, mi_url, created_at, updated_at, deleted_at) VALUES ('4e354ee1-f44b-4ec9-b59f-e42e54768299', 'be6ee736-ed4d-43c9-9c91-bfd0318b875e', '7400b6aa-b248-4264-bff6-1723b44605c6', 'Send Announcement', 1, 'Send new announcement', '/announcements/send', '2023-07-25 11:22:58.216855', '2023-07-25 11:23:57.145330', null);
