use strum_macros::{Display, EnumString, EnumVariantNames};

#[derive(Display, Debug, EnumString, EnumVariantNames)]
pub enum Days {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}
