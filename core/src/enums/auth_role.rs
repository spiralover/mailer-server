use strum_macros::{Display, EnumString, EnumVariantNames};

#[derive(Display, Debug, Clone, EnumString, EnumVariantNames)]
#[strum(serialize_all = "snake_case")]
pub enum AuthRole {
    SuperAdmin,
    Admin,
    Staff,
    HeadOfDepartment,
    HumanResource,
    ManagingDirector,
}
