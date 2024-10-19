use crate::{
    debug::debug_objects,
    structs::{CharVec, ParsingError, StringPtr, ULDDObj, ULDDObjResult},
    types::PE_ARCH,
};
use goblin::pe::{
    characteristic::{IMAGE_FILE_32BIT_MACHINE, IMAGE_FILE_DEBUG_STRIPPED},
    Coff,
};
use std::ptr::null_mut;

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
    let cpu_type = StringPtr::from(PE_ARCH.get(&coff.header.machine)).0;
    debug_objects(file_name, member_names, "a COFF binary", debugging);
    ULDDObjResult {
        error: ParsingError::default(),
        obj: ULDDObj {
            file_name: StringPtr::from(file_name).0,
            member_name: CharVec::from(member_names),
            executable_format: StringPtr::from("COFF").0,
            is_64,
            os_type: StringPtr::from("Windows").0,
            file_type: StringPtr::from("Windows object file").0,
            is_stripped,
            cpu_type,
            cpu_subtype: null_mut(),
            interpreter: null_mut(),
            libraries: CharVec::default(),
        },
    }
}
