use strum_macros::{Display, EnumString, EnumVariantNames};

#[derive(Display, Debug, EnumString, EnumVariantNames)]
#[strum(serialize_all = "snake_case")]
pub enum Entities {
    Temp,
    User,
    Role,
    Permission,
}
