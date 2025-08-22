use strum_macros::{Display, EnumString, VariantNames};

#[derive(Display, Debug, EnumString, VariantNames)]
#[strum(serialize_all = "snake_case")]
pub enum Entities {
    Temp,
    User,
    Role,
    Permission,
}
