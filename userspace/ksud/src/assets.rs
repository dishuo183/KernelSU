#[cfg(target_os = "android")]
mod android {
    use crate::defs::BINARY_DIR;
    use const_format::concatcp;

    pub const RESETPROP_PATH: &str = concatcp!(BINARY_DIR, "resetprop");
    pub const BUSYBOX_PATH: &str = concatcp!(BINARY_DIR, "busybox");
    pub const BOOTCTL_PATH: &str = concatcp!(BINARY_DIR, "bootctl");
}

#[cfg(target_os = "android")]
pub use android::*;
