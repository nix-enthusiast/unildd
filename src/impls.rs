use crate::{
    structs::{CharVec, Debugging, ParsingError, ULDDObj},
    ULDDObjResult, ULDDObjResultVec,
};
use anstream::{eprintln as a_eprintln, println as a_println};
use owo_colors::OwoColorize;
use std::{
    ffi::{c_char, CString},
    fmt::Display,
    mem::ManuallyDrop,
    ptr::null_mut,
};

pub trait StringToCString {
    fn to_c_string(self) -> *mut c_char;
}

pub trait DropCString {
    fn drop_c_string(self);
}

pub trait ErrorToInt {
    fn to_int(&self) -> i64;
}

impl From<Vec<*mut c_char>> for CharVec {
    fn from(value: Vec<*mut c_char>) -> Self {
        CharVec {
            capacity: value.capacity(),
            length: value.len(),
            vec: if value.is_empty() {
                null_mut()
            } else {
                ManuallyDrop::new(value).as_mut_ptr()
            },
        }
    }
}

impl From<Vec<ULDDObjResult>> for ULDDObjResultVec {
    fn from(value: Vec<ULDDObjResult>) -> Self {
        ULDDObjResultVec {
            capacity: value.capacity(),
            length: value.len(),
            vec: if value.is_empty() {
                null_mut()
            } else {
                ManuallyDrop::new(value).as_mut_ptr()
            },
        }
    }
}

impl Default for CharVec {
    fn default() -> Self {
        Self {
            capacity: 0,
            length: 0,
            vec: null_mut(),
        }
    }
}

impl From<Vec<&str>> for CharVec {
    fn from(val: Vec<&str>) -> Self {
        let vector: Vec<*mut c_char> = val
            .into_iter()
            .map(|item| unsafe {
                CString::from_vec_unchecked(item.to_string().into_bytes()).into_raw()
            })
            .collect();

        CharVec::from(vector)
    }
}

impl From<&mut Vec<&str>> for CharVec {
    fn from(val: &mut Vec<&str>) -> Self {
        let vector: Vec<*mut c_char> = std::mem::take(val)
            .into_iter()
            .map(|item| unsafe {
                CString::from_vec_unchecked(item.to_string().into_bytes()).into_raw()
            })
            .collect();

        CharVec::from(vector)
    }
}

impl From<CharVec> for Vec<*mut c_char> {
    fn from(value: CharVec) -> Self {
        if value.vec.is_null() || value.length == 0 {
            return Vec::new();
        };

        unsafe { Vec::from_raw_parts(value.vec, value.length, value.capacity) }
    }
}

impl<T> StringToCString for T
where
    T: Display,
{
    fn to_c_string(self) -> *mut c_char {
        let mut value = self.to_string();
        value.push('\0');
        let c_string = match CString::from_vec_with_nul(value.into_bytes()) {
            Ok(string) => string,
            Err(error) => {
                Debugging::Fatal("converting the string into a C string".to_owned()).print(true);
                panic!("{}", error)
            }
        };
        c_string.into_raw()
    }
}

impl DropCString for *mut c_char {
    fn drop_c_string(self) {
        unsafe {
            if !self.is_null() {
                let _ = CString::from_raw(self);
            }
        }
    }
}

impl DropCString for CharVec {
    fn drop_c_string(self) {
        let vector: Vec<*mut c_char> = self.into();
        
        for item in vector {
            item.drop_c_string();
        }
    }
}

impl ErrorToInt for goblin::error::Error {
    fn to_int(&self) -> i64 {
        match self {
            goblin::error::Error::Malformed(_) => -1,
            goblin::error::Error::BadMagic(_) => -2,
            goblin::error::Error::Scroll(_) => -3,
            goblin::error::Error::BufferTooShort(_, _) => -4,
            goblin::error::Error::IO(_) => -5,
            _ => -6,
        }
    }
}

impl Default for ParsingError {
    fn default() -> Self {
        Self {
            code: 0,
            explanation: null_mut(),
        }
    }
}

impl Default for ULDDObj {
    fn default() -> Self {
        Self {
            file_name: null_mut(),
            member_name: Default::default(),
            executable_format: null_mut(),
            is_64: false,
            os_type: null_mut(),
            file_type: null_mut(),
            is_stripped: false,
            cpu_type: null_mut(),
            cpu_subtype: null_mut(),
            interpreter: null_mut(),
            libraries: Default::default(),
        }
    }
}

impl Debugging {
    pub(crate) fn print(self, debugging: bool) {
        if debugging {
            match self {
                Debugging::Info(msg) => {
                    a_println!("{} {}", "[INFO]".yellow().bold(), msg);
                }
                Debugging::Affirmative(msg) => {
                    a_println!("{} {}", "[OK]".green().bold(), msg);
                }
                Debugging::Error(msg) => {
                    a_eprintln!("{} {}", "[ERROR]".red().bold(), msg);
                }
                Debugging::Fatal(msg) => {
                    a_eprintln!("{} Library got a fatal error while {}. Panic function will halt the library and provide a stacktrace.", "[FATAL]".red().bold(), msg);
                }
            }
        }
    }
}
