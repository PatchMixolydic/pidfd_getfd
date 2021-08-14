#![deny(unsafe_op_in_unsafe_fn)]

#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "linux")]
pub use linux::*;
#[cfg(not(target_os = "linux"))]
compile_error!("pidfds are currently only supported on Linux");
