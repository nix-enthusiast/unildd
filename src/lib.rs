#![doc(
    html_favicon_url = "https://github.com/nix-enthusiast/unildd/blob/main/media/emblems/UniLDD-%25100.png?raw=true"
)]
#![doc(
    html_logo_url = "https://github.com/nix-enthusiast/unildd/blob/main/media/emblems/UniLDD-%25400.png?raw=true"
)]
#![doc(html_playground_url = "https://play.rust-lang.org/")]
//!
//! ![banner](https://github.com/nix-enthusiast/unildd/blob/main/media/banner/UniLDD%20Banner.png?raw=true)
//!
//! ### UniLDD is designed for bringing parsing objects to any language (has a C FFI library).
//!
//! ### ⭐️ Features:
//!  - Detailed information! Some of them are:
//!     - Name of the OS
//!     - File type (Core dump, shared library, executable, etc.)
//!     - ISA type (X86_64, Aarch64, RISC-V, etc.)
//!     - CPU Subtype[^1]
//!     - Name of the linker[^2]
//!     - Which libraries are linked
//!  - Parses without loading objects. Therefore, you can even parse shady objects like malwares![^3]
//!  - Error codes and explanations to make error handling easier.
//!  - A Basic and built-in logger to get real-time information.
//!
//! [^1]:  CPU subtype is a macOS-only feature which tells what kind of CPU model the code is optimized for.
//!
//! [^2]: It has some caveats. See [`ULDDObj`] for further details.
//!
//! [^3]: That doesn't mean I am liable for any damages done by this project and files you parsed. Take your own risk!
//!
use crate::impls::{DropCString, ErrorToInt, StringToCString};
use archive::parse_archive;
use coff::parse_coff;
use debug::merge_members;
use elf::parse_elf;
use goblin::Object;
use mach::parse_mach;
use owo_colors::OwoColorize;
use pe::parse_pe;
use std::ffi::{c_char, CStr};
use structs::{CharVec, Debugging, ParsingError, ULDDObj, ULDDObjResult, ULDDObjResultVec};

#[doc(hidden)]
pub mod archive;
#[doc(hidden)]
pub mod coff;
#[doc(hidden)]
pub mod debug;
#[doc(hidden)]
pub mod elf;
#[doc(hidden)]
pub mod impls;
#[doc(hidden)]
pub mod mach;
#[doc(hidden)]
pub mod pe;

pub mod structs;
pub mod types;

fn parse_objects<'a>(
    file_name: &'a str,
    buffer: &'a [u8],
    member_names: &mut Vec<&'a str>,
    objects: &mut Vec<ULDDObjResult>,
    debugging: bool,
) {
    match Object::parse(buffer) {
        Ok(Object::Archive(archive)) => {
            parse_archive(file_name, buffer, archive, member_names, objects, debugging)
        }

        Ok(Object::Mach(mach)) => {
            parse_mach(file_name, buffer, mach, member_names, objects, debugging)
        }

        Ok(Object::Elf(elf)) => {
            objects.push(parse_elf(
                file_name,
                elf,
                buffer[0x7],
                member_names,
                debugging,
            ));
        }

        Ok(Object::PE(pe)) => objects.push(parse_pe(file_name, pe, member_names, debugging)),

        Ok(Object::COFF(coff)) => {
            objects.push(parse_coff(file_name, coff, member_names, debugging));
        }

        Ok(Object::Unknown(magic_number)) => {
            let msg = format!(
                "The binary named '{}'{} has a unknown magic number (in big-endian): {}",
                file_name,
                merge_members(member_names),
                format!("{:02X?}", magic_number.to_be_bytes()).replace(['[', ']', ','], "")
            );
            Debugging::Error(msg.to_owned()).print(debugging);
            objects.push(ULDDObjResult {
                error: ParsingError {
                    code: magic_number as i64,
                    explanation: msg.to_c_string(),
                },
                obj: ULDDObj {
                    file_name: file_name.to_c_string(),
                    member_name: CharVec::from(member_names),
                    ..Default::default()
                },
            })
        }

        Ok(_) => {
            let msg = format!(
                "The executable format of the file named '{}'{} is not yet implemented",
                file_name,
                merge_members(member_names),
            );
            Debugging::Error(msg.to_string()).print(debugging);
            Debugging::Info(format!(
                "First 16 bytes of the file named '{}' are {}",
                file_name,
                format!("{:02X?}", &buffer[0..17]).replace(['[', ']', ','], "")
            ))
            .print(debugging);

            objects.push(ULDDObjResult {
                error: ParsingError {
                    code: -7,
                    explanation: msg.to_c_string(),
                },
                obj: ULDDObj {
                    file_name: file_name.to_c_string(),
                    member_name: CharVec::from(member_names),
                    ..Default::default()
                },
            })
        }

        Err(error) => {
            Debugging::Error(format!(
                "Error while parsing the bytes of the given file named '{}'{}\nDetails:\n{}",
                file_name,
                merge_members(member_names),
                error
            ))
            .print(debugging);

            objects.push(ULDDObjResult {
                error: ParsingError {
                    code: error.to_int(),
                    explanation: error.to_c_string(),
                },
                obj: ULDDObj {
                    file_name: file_name.to_c_string(),
                    member_name: CharVec::from(member_names),
                    ..Default::default()
                },
            })
        }
    };
}

///
/// Parses the given buffer and returns a vector of parsed binaries.
///
/// # Safety
///
/// This function is null pointer-safe. If the file name is an invalid UTF-8 string and/or buffer pointer is a null pointer it will panic.
///
/// Since the function returns a [`ULDDObjResultVec`] created by rust it has to be [deallocated](free_obj) by rust if it is done by other languages errors may occur.
///
#[no_mangle]
pub unsafe extern "C" fn read_obj(
    file_name: *const c_char,
    buffer: *const u8,
    buffer_size: usize,
    debugging: bool,
) -> ULDDObjResultVec {
    let (buf, f_name) = unsafe {
        let s = match CStr::from_ptr(file_name).to_str() {
            Ok(string_slice) => string_slice,
            Err(error) => {
                Debugging::Fatal("converting the C string to a &str".to_owned()).print(true);
                panic!("{}", error)
            }
        };
        let b = std::slice::from_raw_parts(buffer, buffer_size);
        (b, s)
    };

    let mut objects = vec![];
    parse_objects(f_name, buf, &mut vec![], &mut objects, debugging);
    let (total, success, failed): (usize, usize, usize) = {
        let t = objects.len();
        let (mut s, mut f) = (0, 0);
        objects.iter().for_each(|o| {
            if o.error.code != 0 {
                f += 1;
            } else {
                s += 1
            }
        });
        (t, s, f)
    };

    Debugging::Affirmative(format!(
        "{} binaries from the file(s) are parsed. Success/Fail rate of parsing(s) is {}/{}",
        total,
        success.green(),
        failed.red()
    ))
    .print(debugging);

    ULDDObjResultVec::from(objects)
}

///
/// # Safety
///
/// This function is designed for deallocating [`ULDDObjResultVec`] created by rust. Trying to deallocate [`ULDDObjResultVec`] created by other languages may result with errors.
///
/// It is null pointer-safe.
///
/// ## Error codes:
/// - 0: No errors
/// - 1: `vec` field of [`ULDDObjResultVec`] is a null pointer
///
#[no_mangle]
pub unsafe extern "C" fn free_obj(obj: ULDDObjResultVec, debugging: bool) -> u8 {
    if obj.vec.is_null() {
        Debugging::Error("Given object vector is invalid".to_owned()).print(debugging);

        Debugging::Error("Deallocation(s) is failed".to_owned()).print(debugging);

        return 1;
    };

    let object_vector = Vec::from_raw_parts(obj.vec, obj.length, obj.capacity);
    for (index, object) in object_vector.into_iter().enumerate() {
        Debugging::Info(format!("{}. object is being deallocated", index + 1)).print(debugging);

        let o = object.obj;
        
        object.error.explanation.drop_c_string();
        o.file_name.drop_c_string();
        o.executable_format.drop_c_string();
        o.os_type.drop_c_string();
        o.file_type.drop_c_string();
        o.cpu_type.drop_c_string();
        o.cpu_subtype.drop_c_string();
        o.interpreter.drop_c_string();
        o.member_name.drop_c_string();
        o.libraries.drop_c_string();

        Debugging::Affirmative(format!("{}. object is deallocated", index + 1)).print(debugging);
    }

    Debugging::Affirmative(format!(
        "Deallocation(s) is successful. {} object(s) is freed.",
        obj.length
    ))
    .print(debugging);

    0
}
