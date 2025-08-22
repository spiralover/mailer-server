use strum_macros::{Display, EnumString, VariantNames};

#[derive(Display, Debug, Clone, EnumString, VariantNames)]
#[strum(serialize_all = "snake_case")]
pub enum AuthRole {
    SuperAdmin,
    Admin,
    Staff,
    HeadOfDepartment,
    HumanResource,
    ManagingDirector,
}
