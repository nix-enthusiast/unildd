use crate::{
    debug::debug_objects,
    structs::{CharVec, ParsingError, ULDDObj, ULDDObjResult},
    types::PE_ARCH,
};
use goblin::pe::{
    characteristic::{IMAGE_FILE_32BIT_MACHINE, IMAGE_FILE_DEBUG_STRIPPED},
    Coff,
};
use std::ptr::null_mut;
use crate::debug::option_to_c_string;
use crate::impls::StringToCString;

pub(crate) fn parse_coff(
    file_name: &str,
    coff: Coff,
    member_names: &mut Vec<&str>,
    debugging: bool,
) -> ULDDObjResult {
    // Thanks to developers of goblin for making me to find out that I can "bitwise and" characteristics and wanted characteristics to find out if the COFF file has the one we want
    let is_64 = coff.header.characteristics & IMAGE_FILE_32BIT_MACHINE != IMAGE_FILE_32BIT_MACHINE;
    let is_stripped =
        coff.header.characteristics & IMAGE_FILE_DEBUG_STRIPPED == IMAGE_FILE_DEBUG_STRIPPED;
    let cpu_type = option_to_c_string(PE_ARCH.get(&coff.header.machine));
    debug_objects(file_name, member_names, "a COFF binary", debugging);
    ULDDObjResult {
        error: ParsingError::default(),
        obj: ULDDObj {
            file_name: file_name.to_c_string(),
            member_name: CharVec::from(member_names),
            executable_format: "COFF".to_c_string(),
            is_64,
            os_type: "Windows".to_c_string(),
            file_type: "Windows object file".to_c_string(),
            is_stripped,
            cpu_type,
            cpu_subtype: null_mut(),
            interpreter: null_mut(),
            libraries: CharVec::default(),
        },
    }
}
