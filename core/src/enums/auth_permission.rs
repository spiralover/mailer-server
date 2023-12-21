use strum_macros::{Display, EnumString, EnumVariantNames};

#[derive(Clone, Display, Debug, EnumString, EnumVariantNames)]
#[strum(serialize_all = "snake_case")]
pub enum AuthPermission {
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

    ApplicationList,
    ApplicationCreate,
    ApplicationRead,
    ApplicationUpdate,
    ApplicationDelete,
    ApplicationActivate,
    ApplicationDeactivate,

    ApplicationUserList,

    ApplicationKeyList,
    ApplicationKeyGenerate,

    UserAppList,
    UserAppCreate,
    UserAppRead,
    UserAppUpdate,
    UserAppDelete,

    AuthAttemptList,
    AuthAttemptRead,

    AppKeyRead,
    AppKeyGenerate,

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

    DepartmentList,
    DepartmentCreate,
    DepartmentRead,
    DepartmentUpdate,
    DepartmentDelete,
    DepartmentAssignHead,
    DepartmentHeadList,
    DepartmentUserList,

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

    UserDepartmentList,
    UserDepartmentAssign,

    LeaveTypeList,
    LeaveTypeCreate,
    LeaveTypeRead,
    LeaveTypeUpdate,
    LeaveTypeDelete,

    UserLeaveList,
    UserLeaveCreate,
    UserLeaveRead,
    UserLeaveUpdate,
    UserLeaveDelete,
    UserLeaveClose,
    UserLeaveHodList,
    UserLeaveHodApprove,
    UserLeaveHodReject,
    UserLeaveHrList,
    UserLeaveHrApprove,
    UserLeaveHrReject,

    AnnouncementList,
    AnnouncementRead,
    AnnouncementSend,

    SalaryList,
    SalaryCreate,
    SalaryRead,
    SalaryUpdate,
    SalaryDelete,

    SalaryItemList,
    SalaryItemCreate,
    SalaryItemRead,
    SalaryItemUpdate,
    SalaryItemDelete,

    UserSalaryList,
    UserSalaryCreate,
    UserSalaryRead,
    UserSalaryDelete,

    JobFieldList,
    JobFieldCreate,
    JobFieldRead,
    JobFieldUpdate,
    JobFieldDelete,

    JobTitleList,
    JobTitleCreate,
    JobTitleRead,
    JobTitleUpdate,
    JobTitleDelete,

    StandoffList,
    StandoffCreate,
    StandoffRead,
    StandoffUpdate,
    StandoffDelete,

    UserStandoffList,
    UserStandoffCreate,
    UserStandoffRead,
    UserStandoffDelete,

    UserJobTitleRead,
    UserJobTitleAssign,
    UserJobTitleDelete,
    MailSend,
}
