use libc::stat;
use std::mem;

fn main() {

    println!("sizeof(libc::stat) = {} bytes", size_of::<libc::stat>());
    println!("alignof(libc::stat) = {} bytes", align_of::<libc::stat>())

}

// Concernant la struct stat
    //     pub struct stat { 
    //     pub st_dev: crate::dev_t, u64 - al 8 - offset 0
    //     pub st_ino: crate::ino_t, u64 - al 8 - offset 8
    //     pub st_nlink: crate::nlink_t, u64 - al 8 - offset 16
    //     pub st_mode: crate::mode_t, u32 - al 4 - offset 24
    //     pub st_uid: crate::uid_t, u32 - al 4 - offset 28
    //     pub st_gid: crate::gid_t, u32- al 4 - offset 32
    //     __pad0: c_int, i32 - al 4 - padding de 36 a 39
    //     pub st_rdev: crate::dev_t,  u64 - al 8 - offset 40
    //     pub st_size: off_t, i64 - al 8 - offset 48
    //     pub st_blksize: crate::blksize_t, i64 - al 8 - offset 56
    //     pub st_blocks: crate::blkcnt_t, i64 - al 8 - offset 64
    //     pub st_atime: crate::time_t, i64 - al 8 - offset 72
    //     pub st_atime_nsec: i64, i64 - al 8 - offset 80
    //     pub st_mtime: crate::time_t, i64 - al 8 - offset 88
    //     pub st_mtime_nsec: i64, i64 - al 8 - offset 96
    //     pub st_ctime: crate::time_t, i64 - al 8 - offset 104
    //     pub st_ctime_nsec: i64, i64 - al 8 - offset 112
    //     __unused: [i64; 3], i64 - al 8 - offset 120 -> 128 -> 136 -> 144
    // }
    //prediction alignof -> 8 // size of -> 144 bytes



    