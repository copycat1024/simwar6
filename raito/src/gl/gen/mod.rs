#[allow(dead_code)]
#[allow(clippy::too_many_arguments)]
mod cmd;

#[allow(dead_code)]
#[allow(non_upper_case_globals)]
pub mod consts;

#[allow(dead_code)]
#[allow(non_upper_case_globals)]
#[allow(clippy::enum_variant_names)]
pub mod enums;
mod load;

use load::load;

pub use cmd::GlCmd;
