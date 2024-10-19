use crate::{
    structs::{CharVec, Debugging, ParsingError, StringPtr, ULDDObj},
    ULDDObjResult, ULDDObjResultVec,
};
use anstream::{eprintln as a_eprintln, println as a_println};
use owo_colors::OwoColorize;
use std::{fmt::Display, mem::ManuallyDrop, ptr::null_mut, ffi::{c_char, CString}};

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

impl From<String> for StringPtr {
    fn from(value: String) -> Self {
        let mut value = value;
        value.push('\0');
        let c_string = match CString::from_vec_with_nul(value.into_bytes()) {
            Ok(string) => string,
            Err(error) => {
                Debugging::Fatal("converting the string into a C string".to_owned()).print(true);
                panic!("{}", error)
            }
        };
        StringPtr(c_string.into_raw())
    }
}

impl From<&str> for StringPtr {
    fn from(value: &str) -> Self {
        StringPtr::from(value.to_owned())
    }
}

impl<T> From<Option<T>> for StringPtr
where
    T: Display,
{
    fn from(value: Option<T>) -> Self {
        let Some(t) = value else {
            return StringPtr(null_mut());
        };
        StringPtr::from(t.to_string())
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
