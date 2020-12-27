pub mod file;
pub mod window;

use strum_macros::{EnumString, EnumVariantNames, IntoStaticStr};

#[derive(Debug, Copy, Clone, PartialEq, EnumString, EnumVariantNames, IntoStaticStr)]
#[strum(serialize_all = "kebab_case")]
pub enum Sink {
    Window,
    File,
}
