use std::ptr::null_mut;

use crate::{
    debug::debug_objects,
    structs::{CharVec, ParsingError, ULDDObj, ULDDObjResult},
    types::{ElfFileType, ElfOS, E_MACHINE, E_TYPE},
};
use goblin::elf::Elf;
use crate::debug::option_to_c_string;
use crate::impls::StringToCString;

fn find_os_from_strtab_elf(elf: &Elf<'_>, pat: &[&str]) -> bool {
    [
        elf.strtab.to_vec().unwrap_or(vec![""]),
        elf.shdr_strtab.to_vec().unwrap_or(vec![""]),
        elf.dynstrtab.to_vec().unwrap_or(vec![""]),
    ]
    .iter()
    .flatten()
    .any(|s| pat.iter().any(|i| s.to_lowercase().contains(i)))
}

fn find_os_elf(elf: &Elf<'_>, os_abi: u8) -> (ElfOS, *mut i8) {
    let os = {
        match os_abi {
            0x00 => match true {
                _ if find_os_from_strtab_elf(elf, &["fbsd"]) => ElfOS::FreeBSD,
                _ if find_os_from_strtab_elf(elf, &["openbsd"]) => ElfOS::OpenBSD,
                _ if find_os_from_strtab_elf(elf, &["musl", "glibc", "linux"]) => ElfOS::Linux,
                _ if find_os_from_strtab_elf(elf, &["android"]) => ElfOS::Android,
                _ if find_os_from_strtab_elf(elf, &["netbsd"]) => ElfOS::NetBSD,
                _ if find_os_from_strtab_elf(elf, &["solaris"]) => ElfOS::Solaris,
                _ if find_os_from_strtab_elf(elf, &["illumos"]) => ElfOS::Illumos,
                _ if elf.interpreter.is_some_and(|v| v.contains("Loader.so")) => ElfOS::SerenityOS,
                _ => return (ElfOS::Undefined, null_mut()),
            },
            0x01 => ElfOS::HPUX,
            0x02 => ElfOS::NetBSD,
            0x03 => ElfOS::Linux,
            0x04 => ElfOS::GNUHurd,
            0x06 => {
                if find_os_from_strtab_elf(elf, &["illumos"]) {
                    ElfOS::Illumos
                } else {
                    ElfOS::Solaris
                }
            }
            0x07 => ElfOS::AIXMonterey,
            0x08 => ElfOS::IRIX,
            0x09 => ElfOS::FreeBSD,
            0x10 => ElfOS::FenixOS,
            0x11 => ElfOS::CloudABI,
            0x12 => ElfOS::OpenVOS,
            0x0A => ElfOS::Tru64,
            0x0B => ElfOS::NovellModesto,
            0x0C => ElfOS::OpenBSD,
            0x0D => ElfOS::OpenVMS,
            0x0E => ElfOS::NonStopKernel,
            0x0F => ElfOS::AROS,
            _ => return (ElfOS::Undefined, null_mut()),
        }
    };

    (os, os.to_c_string())
}

fn find_linux_vdso(e_machine: u16, bit_type: bool) -> Option<&'static str> {
    match e_machine {
        0x3E => Some("linux-vdso.so.1"),
        0x03 => Some("linux-vdso.so.1"),
        0x2A => Some("linux-gate.so.1"),
        0x16 => {
            if bit_type {
                Some("linux-vdso64.so.1")
            } else {
                Some("linux-vdso32.so.1")
            }
        }
        0xF3 => Some("linux-vdso.so.1"),
        0x15 => Some("linux-vdso64.so.1"),
        0x14 => Some("linux-vdso32.so.1"),
        0x08 => Some("linux-vdso.so.1"),
        0x32 => Some("linux-gate.so.1"),
        0x28 => Some("linux-vdso.so.1"),
        0xB7 => Some("linux-vdso.so.1"),
        _ => None,
    }
}

fn convert_libraries_into_char_vec(elf: &mut Elf, os_abi: u8) -> CharVec {
    let mut vector = std::mem::take(&mut elf.libraries);
    if let (Some(vdso), ElfOS::Linux) = (
        find_linux_vdso(elf.header.e_machine, elf.is_64),
        find_os_elf(elf, os_abi).0,
    ) {
        vector.push(vdso)
    }

    CharVec::from(vector)
}

pub(crate) fn parse_elf(
    file_name: &str,
    elf: Elf,
    os_abi: u8,
    member_names: &mut Vec<&str>,
    debugging: bool,
) -> ULDDObjResult {
    let mut elf = elf;
    let cpu_type = option_to_c_string(E_MACHINE.get(&elf.header.e_machine));
    let file_type = match E_TYPE.get(&elf.header.e_type) {
        _ if elf.header.e_type == 0x03 && elf.interpreter.is_some() => {
            ElfFileType::Executable.to_c_string()
        }
        rest => option_to_c_string(rest),
    };
    let interpreter = option_to_c_string(elf.interpreter);
    debug_objects(file_name, member_names, "an ELF binary", debugging);
    ULDDObjResult {
        error: ParsingError::default(),
        obj: ULDDObj {
            file_name: file_name.to_c_string(),
            member_name: CharVec::from(member_names),
            executable_format: "ELF".to_c_string(),
            is_64: elf.is_64,
            os_type: find_os_elf(&elf, os_abi).1,
            file_type,
            is_stripped: elf.syms.is_empty(),
            cpu_type,
            cpu_subtype: null_mut(),
            interpreter,
            libraries: convert_libraries_into_char_vec(&mut elf, os_abi),
        },
    }
}
