use anyhow::Result;
use std::path::Path;

#[cfg(target_os = "android")]
mod android {
    use crate::defs::BINARY_DIR;
    use crate::utils::ensure_binary;
    use const_format::concatcp;

    pub const RESETPROP_PATH: &str = concatcp!(BINARY_DIR, "resetprop");
    pub const BUSYBOX_PATH: &str = concatcp!(BINARY_DIR, "busybox");
    pub const BOOTCTL_PATH: &str = concatcp!(BINARY_DIR, "bootctl");

    // 不再需要ensure_binaries函数，因为不内置ko和ksuinit
}

#[cfg(target_os = "android")]
pub use android::*;
