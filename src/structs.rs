use std::ffi::c_char;

/// A C-compatible vector for `Vec<String>`.
#[repr(C)]
pub struct CharVec {
    pub capacity: usize,
    pub length: usize,
    pub vec: *mut *mut c_char,
}

///
/// An error struct for making error handling easy.
///
/// ## Error Codes
/// - \>0: Magic number of the unknown object (as `i64` (or `ìnt64_t))
/// - -1: Binary is corrupted
/// - -2: Unknown/Bad magic number
/// - -3: Error at reading and interpreting bytes
/// - -4: I/O Error at parsing the object
/// - -5: Buffer is too short to hold
/// - -6: Unknown error[^1]
/// - -7: Unimplemented executable format
///
/// [^1]: All errors thrown by goblin crate and my code are covered. Because of matching goblin's [`Error`](goblin::error::Error) is non-exhaustive, I included non-exhaustive path too.
///
#[repr(C)]
pub struct ParsingError {
    pub code: i64,
    pub explanation: *mut c_char,
}

///
/// A struct contains detailed information about the object.
///
/// It contains some information even the object is an erroneous one to make error handling more verbose.
///
/// If the error occurs on parsing:
/// - A file: `file_name` and `member_name`
/// - A Muti Architecture Mach-O file: `file_name`, `member_name` and `executable_format`
/// - An archive: `file_name`, `member_name` and `file_type`
///
/// fields will be filled correctly and the rest will be:
/// - null (the fields which are string)
/// - blank (`member_name` and `libraries`)
/// - `false` (`is_64` and `is_stripped`).
///
#[repr(C)]
pub struct ULDDObj {
    /// The name of the object.
    ///
    /// Objects inside Muti Architecture Mach-O files will be named as "n. file" due to they don't have file names.
    pub file_name: *mut c_char,
    /// The location of objects in recursive files.
    ///
    /// This field is empty if the object is not in a recursive file (Like: Archives and Muti Architecture Mach-O files).
    ///
    /// The names in the vector is sorted as outer to inner.
    pub member_name: CharVec,
    /// The type of the executable format of the object.
    pub executable_format: *mut c_char,
    /// The field is true if the object is 64 bit otherwise it is 32 bit or the object is an erroneous one.
    pub is_64: bool,
    /// The name of the OS it was compiled for.
    pub os_type: *mut c_char,
    /// The type of the object.
    pub file_type: *mut c_char,
    /// The field is true if the object was stripped from debug symbols otherwise it is not stripped or the object is an erroneous one .
    pub is_stripped: bool,
    /// The ISA (CPU Architecture) the object compiled for.
    pub cpu_type: *mut c_char,
    /// The specific CPU model the object compiled for.
    ///
    /// macOS only field. It is null pointer in other executable formats.
    pub cpu_subtype: *mut c_char,
    /// The name/version of the linker.
    ///
    /// ELF/PE only field. It is null pointer in other executable formats.
    ///
    /// It returns the version of the linker in PE files.
    pub interpreter: *mut c_char,
    /// A vector of libraries linked against the object.
    ///
    /// It is blank in COFF files because they are mostly PE object files therefore they don't have linked libraries against them.
    pub libraries: CharVec,
}

/// A struct packs (empty or filled) error and (successfully or not) read object.
#[repr(C)]
pub struct ULDDObjResult {
    pub error: ParsingError,
    pub obj: ULDDObj,
}

/// A C-compatible vector for [`ULDDObjResult`].
#[repr(C)]
pub struct ULDDObjResultVec {
    pub capacity: usize,
    pub length: usize,
    pub vec: *mut ULDDObjResult,
}

#[doc(hidden)]
pub struct StringPtr(pub *mut i8);

#[doc(hidden)]
pub(crate) enum Debugging {
    Info(String),
    Affirmative(String),
    Error(String),
    Fatal(String),
}
