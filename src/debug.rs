use std::ffi::c_char;
use std::fmt::Display;
use std::ptr::null_mut;
use crate::impls::StringToCString;
use crate::structs::Debugging;

pub(crate) fn option_to_c_string<T>(option: Option<T>) -> *mut c_char where T: Display  {
    option.map(|v| v.to_c_string()).unwrap_or(null_mut())
}

pub(crate) fn merge_members(member_names: &mut [&str]) -> String {
    if !member_names.is_empty() {
        format!(" (Member of: {})", member_names.join(" -> "))
    } else {
        String::new()
    }
}

pub(crate) fn debug_objects(
    file_name: &str,
    member_names: &mut [&str],
    object_name: &str,
    debugging: bool,
) {
    Debugging::Info(format!(
        "The binary named '{}'{} is {}",
        file_name,
        merge_members(member_names),
        object_name
    ))
    .print(debugging)
}
