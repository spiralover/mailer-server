// @generated automatically by Diesel CLI.

diesel::table! {
    announcements (announcement_id) {
        announcement_id -> Uuid,
        sender_id -> Uuid,
        #[max_length = 150]
        title -> Varchar,
        message -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    app_keys (app_key_id) {
        app_key_id -> Uuid,
        created_by -> Uuid,
        application_id -> Uuid,
        #[max_length = 500]
        public_key -> Varchar,
        #[max_length = 500]
        private_key -> Varchar,
        #[max_length = 20]
        status -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    applications (application_id) {
        application_id -> Uuid,
        created_by -> Uuid,
        #[max_length = 150]
        name -> Varchar,
        #[max_length = 100]
        code -> Varchar,
        #[max_length = 100]
        url -> Varchar,
        #[max_length = 250]
        logo -> Varchar,
        #[max_length = 1000]
        webhook -> Varchar,
        #[max_length = 1000]
        description -> Varchar,
        #[max_length = 50]
        status -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    auth_attempts (auth_attempt_id) {
        auth_attempt_id -> Uuid,
        user_id -> Nullable<Uuid>,
        #[max_length = 150]
        email -> Varchar,
        #[max_length = 100]
        ip_address -> Nullable<Varchar>,
        #[max_length = 250]
        user_agent -> Nullable<Varchar>,
        #[max_length = 1000]
        auth_error -> Nullable<Varchar>,
        #[max_length = 10]
        verification_code -> Nullable<Varchar>,
        verification_code_trials -> Int2,
        #[max_length = 20]
        status -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    file_uploads (file_upload_id) {
        file_upload_id -> Uuid,
        uploader_id -> Uuid,
        owner_id -> Uuid,
        #[max_length = 50]
        owner_type -> Varchar,
        #[max_length = 255]
        orig_name -> Varchar,
        #[max_length = 255]
        file_name -> Varchar,
        #[max_length = 1000]
        file_path -> Varchar,
        #[max_length = 255]
        file_ext -> Varchar,
        #[max_length = 250]
        description -> Nullable<Varchar>,
        #[max_length = 250]
        additional_info -> Nullable<Varchar>,
        is_temp -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    mail_addresses (mail_address_id) {
        mail_address_id -> Uuid,
        mail_id -> Uuid,
        #[max_length = 200]
        name -> Varchar,
        #[max_length = 200]
        email -> Varchar,
        #[max_length = 50]
        addr_type -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::table! {
    mail_errors (mail_error_id) {
        mail_error_id -> Uuid,
        mail_id -> Uuid,
        smtp_error -> Text,
        created_at -> Timestamp,
    }
}

diesel::table! {
    mails (mail_id) {
        mail_id -> Uuid,
        created_by -> Uuid,
        application_id -> Uuid,
        #[max_length = 1000]
        subject -> Varchar,
        message -> Text,
        #[max_length = 200]
        from_name -> Varchar,
        #[max_length = 200]
        from_email -> Varchar,
        #[max_length = 200]
        reply_to_name -> Nullable<Varchar>,
        #[max_length = 200]
        reply_to_email -> Nullable<Varchar>,
        trials -> Int2,
        #[max_length = 50]
        status -> Varchar,
        sent_at -> Nullable<Timestamp>,
        next_retrial_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    notifications (notification_id) {
        notification_id -> Uuid,
        receiver_id -> Uuid,
        #[max_length = 250]
        title -> Varchar,
        #[max_length = 3000]
        url -> Varchar,
        #[max_length = 6000]
        content -> Varchar,
        #[max_length = 30]
        status -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    password_resets (password_reset_id) {
        password_reset_id -> Uuid,
        user_id -> Uuid,
        #[max_length = 100]
        email -> Varchar,
        #[max_length = 100]
        token -> Varchar,
        #[max_length = 50]
        ip_address -> Nullable<Varchar>,
        #[max_length = 500]
        user_agent -> Nullable<Varchar>,
        #[max_length = 30]
        status -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    permissions (permission_id) {
        permission_id -> Uuid,
        created_by -> Uuid,
        #[max_length = 250]
        permission_name -> Varchar,
        #[max_length = 250]
        guard_name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    personal_access_tokens (pat_id) {
        pat_id -> Uuid,
        user_id -> Uuid,
        #[max_length = 1000]
        title -> Varchar,
        #[max_length = 1500]
        comment -> Varchar,
        #[max_length = 1500]
        token -> Varchar,
        #[max_length = 50]
        status -> Varchar,
        expired_at -> Timestamp,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    role_permissions (role_permission_id) {
        role_permission_id -> Uuid,
        created_by -> Uuid,
        role_id -> Uuid,
        permission_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    roles (role_id) {
        role_id -> Uuid,
        created_by -> Uuid,
        #[max_length = 250]
        role_name -> Varchar,
        #[max_length = 250]
        guard_name -> Varchar,
        #[max_length = 50]
        status -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    ui_menu_items (ui_menu_item_id) {
        ui_menu_item_id -> Uuid,
        created_by -> Uuid,
        ui_menu_id -> Uuid,
        #[max_length = 50]
        mi_name -> Varchar,
        mi_priority -> Int4,
        #[max_length = 250]
        mi_desc -> Nullable<Varchar>,
        #[max_length = 3000]
        mi_url -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    ui_menus (ui_menu_id) {
        ui_menu_id -> Uuid,
        created_by -> Uuid,
        #[max_length = 50]
        m_name -> Varchar,
        m_priority -> Int4,
        #[max_length = 250]
        m_desc -> Nullable<Varchar>,
        #[max_length = 3000]
        m_url -> Nullable<Varchar>,
        m_has_items -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    user_apps (user_app_id) {
        user_app_id -> Uuid,
        created_by -> Uuid,
        user_id -> Uuid,
        application_id -> Uuid,
        #[max_length = 1000]
        comment -> Nullable<Varchar>,
        #[max_length = 50]
        status -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    user_permissions (user_permission_id) {
        user_permission_id -> Uuid,
        created_by -> Uuid,
        user_id -> Uuid,
        permission_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    user_roles (user_role_id) {
        user_role_id -> Uuid,
        created_by -> Uuid,
        role_id -> Uuid,
        user_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    user_ui_menu_items (user_ui_menu_item_id) {
        user_ui_menu_item_id -> Uuid,
        created_by -> Uuid,
        ui_menu_id -> Uuid,
        ui_menu_item_id -> Uuid,
        user_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (user_id) {
        user_id -> Uuid,
        created_by -> Nullable<Uuid>,
        #[max_length = 150]
        username -> Varchar,
        #[max_length = 150]
        first_name -> Nullable<Varchar>,
        #[max_length = 150]
        last_name -> Nullable<Varchar>,
        #[max_length = 100]
        email -> Varchar,
        #[max_length = 250]
        password -> Varchar,
        #[max_length = 700]
        profile_picture -> Nullable<Varchar>,
        #[max_length = 10]
        verification_code -> Nullable<Varchar>,
        #[max_length = 50]
        verification_token -> Nullable<Varchar>,
        verified_at -> Nullable<Timestamp>,
        is_verified -> Bool,
        is_password_locked -> Bool,
        has_started_password_reset -> Bool,
        #[max_length = 50]
        temp_password_status -> Varchar,
        #[max_length = 50]
        status -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(announcements -> users (sender_id));
diesel::joinable!(app_keys -> applications (application_id));
diesel::joinable!(app_keys -> users (created_by));
diesel::joinable!(applications -> users (created_by));
diesel::joinable!(auth_attempts -> users (user_id));
diesel::joinable!(file_uploads -> users (uploader_id));
diesel::joinable!(mail_addresses -> mails (mail_id));
diesel::joinable!(mail_errors -> mails (mail_id));
diesel::joinable!(mails -> applications (application_id));
diesel::joinable!(mails -> users (created_by));
diesel::joinable!(notifications -> users (receiver_id));
diesel::joinable!(password_resets -> users (user_id));
diesel::joinable!(permissions -> users (created_by));
diesel::joinable!(personal_access_tokens -> users (user_id));
diesel::joinable!(role_permissions -> permissions (permission_id));
diesel::joinable!(role_permissions -> roles (role_id));
diesel::joinable!(role_permissions -> users (created_by));
diesel::joinable!(roles -> users (created_by));
diesel::joinable!(ui_menu_items -> ui_menus (ui_menu_id));
diesel::joinable!(ui_menu_items -> users (created_by));
diesel::joinable!(ui_menus -> users (created_by));
diesel::joinable!(user_apps -> applications (application_id));
diesel::joinable!(user_permissions -> permissions (permission_id));
diesel::joinable!(user_roles -> roles (role_id));
diesel::joinable!(user_ui_menu_items -> ui_menu_items (ui_menu_item_id));
diesel::joinable!(user_ui_menu_items -> ui_menus (ui_menu_id));

diesel::allow_tables_to_appear_in_same_query!(
    announcements,
    app_keys,
    applications,
    auth_attempts,
    file_uploads,
    mail_addresses,
    mail_errors,
    mails,
    notifications,
    password_resets,
    permissions,
    personal_access_tokens,
    role_permissions,
    roles,
    ui_menu_items,
    ui_menus,
    user_apps,
    user_permissions,
    user_roles,
    user_ui_menu_items,
    users,
);
