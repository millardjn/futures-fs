extern crate futures_fs;

use futures_fs::{FsPool, FsReadStream, FsWriteSink};

// The following is a compilation test
// to assert a Sync + Send bound on the public types

trait SyncSend: Sync + Send {}

impl SyncSend for FsPool {}

impl SyncSend for FsReadStream {}

impl SyncSend for FsWriteSink {}


// The following line will not compile if uncommented,
// showing that the above impl blocks correctly
// bound the public types.

// impl SyncSend for NotSyncOrSend{}

#[allow(unused)]
struct NotSyncOrSend {
    raw_pointer: *const i32
}

