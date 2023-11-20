use strum_macros::{Display, EnumString, EnumVariantNames};

#[derive(Clone, Display, Debug, EnumString, EnumVariantNames)]
#[strum(serialize_all = "snake_case")]
pub enum Permissions {
    MiscUploadTempFile,

    UserList,
    UserCreate,
    UserRead,
    UserUpdate,
    UserDelete,
    UserActivate,
    UserDeactivate,
    UserChangePassword,
    UserUploadPassport,

    UserMyProfileUpdate,
    UserMyProfileListAuthAttempt,
    UserMyProfileUploadPassport,

    AuthAttemptList,
    AuthAttemptRead,

    RoleList,
    RoleCreate,
    RoleRead,
    RoleUpdate,
    RoleDelete,
    RoleActivate,
    RoleDeactivate,
    RoleUserList,

    PermissionList,
    PermissionCreate,
    PermissionRead,
    PermissionUpdate,
    PermissionDelete,

    RolePermissionList,
    RolePermissionCreate,
    RolePermissionRead,
    RolePermissionUpdate,
    RolePermissionDelete,

    UserAuthAttemptList,

    UserRoleList,
    UserRoleAssign,
    UserRoleRead,
    UserRoleUnAssign,

    UserPermissionList,
    UserPermissionCreate,
    UserPermissionRead,
    UserPermissionUpdate,
    UserPermissionDelete,

    UiMenuList,
    UiMenuCreate,
    UiMenuRead,
    UiMenuUpdate,
    UiMenuDelete,

    UiMenuItemList,
    UiMenuItemCreate,
    UiMenuItemRead,
    UiMenuItemUpdate,
    UiMenuItemDelete,

    UserUiMenuItemList,
    UserUiMenuItemCreate,
    UserUiMenuItemRead,
    UserUiMenuItemUpdate,
    UserUiMenuItemDelete,

    AnnouncementList,
    AnnouncementRead,
    AnnouncementSend,

    ApplicationList,
    ApplicationCreate,
    ApplicationRead,
    ApplicationUpdate,
    ApplicationDelete,
    ApplicationActivate,
    ApplicationDeactivate,

    ApplicationKeyList,
    ApplicationKeyRead,
    ApplicationKeyGenerate,
    ApplicationKeyApprove,
    ApplicationKeyReject,
    ApplicationKeyRemove,

    MailList,
    MailSend,
    MailReSend,
}
