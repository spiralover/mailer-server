CREATE TABLE user_ui_menu_items
(
    user_ui_menu_item_id UUID      NOT NULL UNIQUE PRIMARY KEY,
    created_by           UUID      NOT NULL,
    ui_menu_id           UUID      NOT NULL,
    ui_menu_item_id      UUID      NOT NULL,
    user_id              UUID      NOT NULL,
    created_at           TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at           TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at           TIMESTAMP NULL     DEFAULT NULL
);

SELECT auto_handle_updated_at('user_ui_menu_items');

ALTER TABLE user_ui_menu_items
    ADD CONSTRAINT fk_user_ui_menu_items_created_by FOREIGN KEY (created_by) REFERENCES users (user_id);

ALTER TABLE user_ui_menu_items
    ADD CONSTRAINT fk_user_ui_menu_items_ui_menu_id FOREIGN KEY (ui_menu_id) REFERENCES ui_menus (ui_menu_id);

ALTER TABLE user_ui_menu_items
    ADD CONSTRAINT fk_user_ui_menu_items_ui_menu_item_id FOREIGN KEY (ui_menu_item_id) REFERENCES ui_menu_items (ui_menu_item_id);

ALTER TABLE user_ui_menu_items
    ADD CONSTRAINT fk_user_ui_menu_items_user_id FOREIGN KEY (user_id) REFERENCES users (user_id);
