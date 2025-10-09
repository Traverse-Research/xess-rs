#![doc = include_str!("../README.md")]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::missing_safety_doc)]

#[cfg(feature = "xefg_swapchain")]
pub mod xefg_swapchain;
#[cfg(feature = "xell")]
pub mod xell;
#[cfg(feature = "xess")]
pub mod xess;
