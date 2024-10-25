use std::ffi::c_char;
use crate::{
    debug::{debug_objects, merge_members},
    structs::{CharVec, Debugging, ParsingError, ULDDObj, ULDDObjResult},
    types::{
        MachOCpuType, MachOOs, MACH_O_ARM_CPU_SUBTYPE, MACH_O_CPUTYPE, MACH_O_FILE_TYPE,
        MACH_O_X86_CPU_SUBTYPE,
    },
};
use goblin::mach::{load_command::CommandVariant::BuildVersion, Mach, MachO};
use std::ptr::null_mut;
use crate::debug::option_to_c_string;
use crate::impls::{ErrorToInt, StringToCString};

fn find_os_mach(mach: &MachO<'_>) -> *mut c_char {
    for lc in &mach.load_commands {
        if let BuildVersion(build_version) = lc.command {
            let os = match build_version.platform {
                0x01 => MachOOs::MacOS,
                0x02 => MachOOs::IOS,
                0x03 => MachOOs::AppleTVBox,
                0x04 => MachOOs::AppleWatch,
                0x05 => MachOOs::BridgeOS,
                0x06 => MachOOs::MacCatalyst,
                0x07 => MachOOs::IOSSimulator,
                0x08 => MachOOs::AppleTVSimulator,
                0x09 => MachOOs::AppleWatchSimulator,
                0x0A => MachOOs::DriverKit,
                0x0B => MachOOs::AppleVisionPro,
                0x0C => MachOOs::AppleVisionProSimulator,
                _ => return null_mut(),
            };
            return os.to_c_string();
        }
    }

    null_mut()
}

/* 
I will leave it there because I may use later

fn decode_further(mach: &MachO<'_>) {
     for lc in &mach.load_commands {
         if let BuildVersion(build_version) = lc.command {
             let os = match build_version.platform {
                 0x01 => MachOOs::MacOS,
                 0x02 => MachOOs::IOS,
                 0x03 => MachOOs::AppleTVBox,
                 0x04 => MachOOs::AppleWatch,
                 0x05 => MachOOs::BridgeOS,
                 0x06 => MachOOs::MacCatalyst,
                 0x07 => MachOOs::IOSSimulator,
                 0x08 => MachOOs::AppleTVSimulator,
                 0x09 => MachOOs::AppleWatchSimulator,
                 0x0A => MachOOs::DriverKit,
                 0x0B => MachOOs::AppleVisionPro,
                 0x0C => MachOOs::AppleVisionProSimulator,
                 _ => MachOOs::Undefined,
             };
             let os_ver = {
                 let [_, x, y, z] = build_version.minos.to_be_bytes();
                 format!("{x}.{y}.{z}")
             };
             let sdk_ver = {
                 let [_, x, y, z] = build_version.sdk.to_be_bytes();
                 format!("{x}.{y}.{z}")
             };
             let tool_type = match build_version.ntools {
                 0x1 => "Clang",
                 0x2 => "Swift",
                 0x3 => "Linked with ld",
                 _ => "Unknown"
             };
         }
     }
}
*/

pub(crate) fn parse_mach<'a>(
    file_name: &'a str,
    buffer: &[u8],
    mach: Mach,
    member_names: &mut Vec<&'a str>,
    objects: &mut Vec<ULDDObjResult>,
    debugging: bool,
) {
    match mach {
        Mach::Fat(fat) => {
            debug_objects(
                file_name,
                member_names,
                "a multi architecture Mach-O",
                debugging,
            );
            let fat_arches = match fat.arches() {
                Ok(arches) => arches,
                Err(error) => {
                    Debugging::Error(format!("Error while reading the multi architecture Mach-O binary named '{}'{}\nDetails:\n{}",
                            file_name,
                            merge_members(member_names),
                            error)).print(debugging);

                    return objects.push(ULDDObjResult {
                        error: ParsingError {
                            code: error.to_int(),
                            explanation: error.to_c_string(),
                        },
                        obj: ULDDObj {
                            file_name: file_name.to_c_string(),
                            member_name: CharVec::from(member_names),
                            executable_format: "Mach-O".to_c_string(),
                            ..Default::default()
                        },
                    });
                }
            };

            for (index, arch) in fat_arches.iter().enumerate() {
                match MachO::parse(buffer, arch.offset as usize) {
                    Ok(mach_o) => {
                        member_names.push(file_name);
                        objects.push(parse_mach_o(
                            &format!("{}. file", index + 1),
                            member_names,
                            mach_o,
                            debugging,
                        ))
                    }
                    Err(error) => {
                        Debugging::Error(format!("Error while processing the multi architecture Mach-O binary named '{}'{}\nDetails:\n{}", file_name, merge_members(member_names),
                            error)).print(debugging);
                        objects.push(ULDDObjResult {
                            error: ParsingError {
                                code: error.to_int(),
                                explanation: error.to_c_string(),
                            },
                            obj: ULDDObj {
                                file_name: file_name.to_c_string(),
                                member_name: CharVec::from(member_names.clone()),
                                executable_format: "Mach-O".to_c_string(),
                                ..Default::default()
                            },
                        })
                    }
                }
            }
        }
        Mach::Binary(binary) => {
            objects.push(parse_mach_o(file_name, member_names, binary, debugging))
        }
    }
}

fn parse_mach_o(
    file_name: &str,
    member_names: &mut Vec<&str>,
    mach_o: MachO,
    debugging: bool,
) -> ULDDObjResult {
    let mut mach_o = mach_o;
    let file_type = option_to_c_string(MACH_O_FILE_TYPE.get(&mach_o.header.filetype));
    let (cpu_type, cpu_subtype) = {
        if let Some(mach_o_cpu_type) = MACH_O_CPUTYPE.get(&mach_o.header.cputype) {
            let mach_o_cpu_subtype = {
                match mach_o_cpu_type {
                    MachOCpuType::ARM | MachOCpuType::ARM64 => {
                        option_to_c_string(MACH_O_ARM_CPU_SUBTYPE.get(&mach_o.header.cpusubtype))
                    }
                    MachOCpuType::X86 | MachOCpuType::X86_64 => {
                        option_to_c_string(MACH_O_X86_CPU_SUBTYPE.get(&mach_o.header.cpusubtype))
                    }
                    _ => null_mut(),
                }
            };
            (
                mach_o_cpu_type.to_c_string(),
                mach_o_cpu_subtype,
            )
        } else {
            (null_mut(), null_mut())
        }
    };

    let is_stripped = !mach_o
        .symbols
        .as_ref()
        .is_some_and(|v| v.iter().any(|s| s.is_ok_and(|(x, _)| x.contains("debug"))));

    mach_o.libs.retain(|lib| lib != &"self");
    debug_objects(file_name, member_names, "a Mach-O binary", debugging);

    ULDDObjResult {
        error: ParsingError::default(),
        obj: ULDDObj {
            file_name: file_name.to_c_string(),
            member_name: CharVec::from(member_names),
            executable_format: "Mach-O".to_c_string(),
            is_64: mach_o.is_64,
            os_type: find_os_mach(&mach_o),
            file_type,
            is_stripped,
            cpu_type,
            cpu_subtype,
            interpreter: null_mut(),
            libraries: CharVec::from(mach_o.libs),
        },
    }
}
