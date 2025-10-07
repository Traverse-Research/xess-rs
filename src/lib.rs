#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::missing_safety_doc)]

#[cfg(feature = "xell")]
pub mod xell;
#[cfg(feature = "xess")]
pub mod xess;
#[cfg(feature = "xess_fg")]
pub mod xess_fg;
