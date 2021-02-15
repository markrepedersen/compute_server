use std::{ffi::OsStr, path::Path};

pub fn is_valid(filename: &str) -> bool {
    unimplemented!();
}

fn get_file_extension(filename: &str) -> Option<&str> {
    Path::new(filename).extension().and_then(OsStr::to_str)
}
