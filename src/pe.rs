use crate::{
    debug::debug_objects,
    structs::{CharVec, ParsingError, ULDDObj, ULDDObjResult},
    types::{PeOS, PeSubsystem, PE_ARCH, PE_SUBSYSTEM},
};
use goblin::pe::{characteristic::IMAGE_FILE_DEBUG_STRIPPED, PE};
use std::ptr::null_mut;
use crate::debug::option_to_c_string;
use crate::impls::StringToCString;

fn find_os_pe(pe: &PE<'_>) -> *mut i8 {
    let Some(optional_header) = pe
        .header
        .optional_header
        .and_then(|h| PE_SUBSYSTEM.get(&h.windows_fields.subsystem))
    else {
        return null_mut();
    };

    let os = match optional_header {
        PeSubsystem::Xbox => PeOS::Xbox,
        PeSubsystem::EFIApplication
        | PeSubsystem::EFIBootServiceDriver
        | PeSubsystem::EFIRom
        | PeSubsystem::EFIRuntimeDriver => PeOS::UEFI,
        PeSubsystem::WindowsCUI
        | PeSubsystem::WindowsGUI
        | PeSubsystem::Native
        | PeSubsystem::OS2CUI
        | PeSubsystem::PosixCUI
        | PeSubsystem::NativeWindows
        | PeSubsystem::WindowsCEGUI
        | PeSubsystem::WindowsBootApplication => PeOS::Windows,
        PeSubsystem::Unknown => return null_mut(),
    };

    os.to_c_string()
}

pub(crate) fn parse_pe(
    file_name: &str,
    pe: PE,
    member_names: &mut Vec<&str>,
    debugging: bool,
) -> ULDDObjResult {
    let is_stripped = pe.header.coff_header.characteristics & IMAGE_FILE_DEBUG_STRIPPED
        == IMAGE_FILE_DEBUG_STRIPPED;
    let cpu_type = option_to_c_string(PE_ARCH.get(&pe.header.coff_header.machine));
    let file_type = pe
        .header
        .optional_header
        .and_then(|h| PE_SUBSYSTEM.get(&h.windows_fields.subsystem));
    let interpreter = {
        if let Some(optional_header) = pe.header.optional_header {
            let linker_major_version = optional_header
                .windows_fields
                .major_operating_system_version;
            let linker_minor_version = optional_header
                .windows_fields
                .minor_operating_system_version;
            let linker_version = format!("{}.{}", linker_major_version, linker_minor_version);
            linker_version.to_c_string()
        } else {
            null_mut()
        }
    };
    let executable_format = if pe.is_64 {
        debug_objects(file_name, member_names, "a PE32+ binary", debugging);
        "PE32+".to_c_string()
    } else {
        debug_objects(file_name, member_names, "a PE32 binary", debugging);
        "PE32".to_c_string()
    };
    ULDDObjResult {
        error: ParsingError::default(),
        obj: ULDDObj {
            file_name: file_name.to_c_string(),
            member_name: CharVec::from(member_names),
            executable_format,
            is_64: pe.is_64,
            os_type: find_os_pe(&pe),
            file_type: option_to_c_string(file_type),
            is_stripped,
            cpu_type,
            cpu_subtype: null_mut(),
            interpreter,
            libraries: CharVec::from(pe.libraries),
        },
    }
}
