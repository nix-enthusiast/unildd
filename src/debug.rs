use goblin::error::Error as ObjectError;

use crate::structs::Debugging;

pub(crate) fn find_error_type(error: &ObjectError) -> i64 {
    match error {
        ObjectError::Malformed(_) => -1,
        ObjectError::BadMagic(_) => -2,
        ObjectError::Scroll(_) => -3,
        ObjectError::BufferTooShort(_, _) => -4,
        ObjectError::IO(_) => -5,
        _ => -6,
    }
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
