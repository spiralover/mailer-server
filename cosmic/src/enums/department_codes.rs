use strum_macros::{Display, EnumString, VariantNames};

#[derive(Display, Debug, EnumString, VariantNames)]
#[strum(serialize_all = "snake_case")]
pub enum DepartmentCodes {
    HumanResource,
    InformationTechnology,
    InternalControlUnit,
}

pub const DEPT_CODE_HUMAN_RESOURCE: &str = "SOV/DEPT/HR";
#[allow(dead_code)]
pub const DEPT_CODE_INTERNAL_CONTROL_UNIT: &str = "SOV/DEPT/ICU";
#[allow(dead_code)]
pub const DEPT_CODE_INFORMATION_TECHNOLOGY: &str = "SOV/DEPT/IT";
