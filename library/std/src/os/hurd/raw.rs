//! Hurd-specific raw type definitions

#![stable(feature = "raw_ext", since = "1.1.0")]
#![deprecated(
    since = "1.8.0",
    note = "these type aliases are no longer supported by \
              the standard library, the `libc` crate on \
              crates.io should be used instead for the correct \
              definitions"
)]
#![allow(deprecated)]

use crate::os::raw::{c_int, c_long, c_uint, c_ulong};
use crate::os::unix::raw::{gid_t, uid_t};

#[stable(feature = "raw_ext", since = "1.1.0")]
pub type blkcnt_t = u64;
#[stable(feature = "raw_ext", since = "1.1.0")]
pub type blksize_t = c_long;
#[stable(feature = "raw_ext", since = "1.1.0")]
pub type dev_t = c_ulong;
#[stable(feature = "raw_ext", since = "1.1.0")]
pub type ino_t = u64;
#[stable(feature = "raw_ext", since = "1.1.0")]
pub type mode_t = c_uint;
#[stable(feature = "raw_ext", since = "1.1.0")]
pub type nlink_t = c_ulong;
#[stable(feature = "raw_ext", since = "1.1.0")]
pub type off_t = u64;
#[stable(feature = "raw_ext", since = "1.1.0")]
pub type time_t = c_long;

#[stable(feature = "pthread_t", since = "1.8.0")]
pub type pthread_t = c_long;

#[repr(C)]
#[derive(Clone)]
#[stable(feature = "raw_ext", since = "1.1.0")]
pub struct stat {
    #[stable(feature = "raw_ext", since = "1.1.0")]
    pub st_fstype: c_int,
    #[stable(feature = "raw_ext", since = "1.1.0")]
    pub st_dev: u64,
    #[stable(feature = "raw_ext", since = "1.1.0")]
    pub st_ino: u64,
    #[stable(feature = "raw_ext", since = "1.1.0")]
    pub st_gen: c_uint,
    #[stable(feature = "raw_ext", since = "1.1.0")]
    pub st_rdev: c_ulong,
    #[stable(feature = "raw_ext", since = "1.1.0")]
    pub st_mode: c_uint,
    #[stable(feature = "raw_ext", since = "1.1.0")]
    pub st_nlink: c_ulong,
    #[stable(feature = "raw_ext", since = "1.1.0")]
    pub st_uid: uid_t,
    #[stable(feature = "raw_ext", since = "1.1.0")]
    pub st_gid: gid_t,
    #[stable(feature = "raw_ext", since = "1.1.0")]
    pub st_size: i64,
    #[stable(feature = "raw_ext", since = "1.1.0")]
    pub st_atime: c_long,
    #[stable(feature = "raw_ext", since = "1.1.0")]
    pub st_atime_nsec: c_long,
    #[stable(feature = "raw_ext", since = "1.1.0")]
    pub st_mtime: c_long,
    #[stable(feature = "raw_ext", since = "1.1.0")]
    pub st_mtime_nsec: c_long,
    #[stable(feature = "raw_ext", since = "1.1.0")]
    pub st_ctime: c_long,
    #[stable(feature = "raw_ext", since = "1.1.0")]
    pub st_ctime_nsec: c_long,
    #[stable(feature = "raw_ext", since = "1.1.0")]
    pub st_blksize: c_long,
    #[stable(feature = "raw_ext", since = "1.1.0")]
    pub st_blocks: i64,
    #[stable(feature = "raw_ext", since = "1.1.0")]
    pub st_author: uid_t,
    #[stable(feature = "raw_ext", since = "1.1.0")]
    pub st_flags: c_uint,
    st_spare: [u32; 11],
}
