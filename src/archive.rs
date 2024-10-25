use crate::{
    debug::{debug_objects, merge_members},
    parse_objects,
    structs::{CharVec, Debugging, ParsingError, ULDDObj, ULDDObjResult},
};
use goblin::archive::Archive;
use crate::impls::{ErrorToInt, StringToCString};

pub(crate) fn parse_archive<'a>(
    file_name: &'a str,
    buffer: &'a [u8],
    archive: Archive<'a>,
    member_names: &mut Vec<&'a str>,
    objects: &mut Vec<ULDDObjResult>,
    debugging: bool,
) {
    for member in archive.members() {
        member_names.push(file_name);
        let member_buffer = match archive.extract(member, buffer) {
            Ok(buf) => buf,
            Err(error) => {
                Debugging::Error(format!("Error while extracting the bytes of the member named '{}' from buffer of the file named '{}'{}\nDetails:\n{}",
                    member,
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
                        file_type: "Archive".to_c_string(),
                        ..Default::default()
                    },
                });
            }
        };
        debug_objects(file_name, member_names, "an archive file", debugging);
        parse_objects(member, member_buffer, member_names, objects, debugging);
    }
}
