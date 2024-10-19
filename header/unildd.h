#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 *
 * An error struct for making error handling easy.
 *
 * ## Error Codes
 * - \>0: Magic number of the unknown object (as `i64` (or `ìnt64_t))
 * - -1: Binary is corrupted
 * - -2: Unknown/Bad magic number
 * - -3: Error at reading and interpreting bytes
 * - -4: I/O Error at parsing the object
 * - -5: Buffer is too short to hold
 * - -6: Unknown error[^1]
 * - -7: Unimplemented executable format
 *
 * [^1]: All errors thrown by goblin crate and my code are covered. Because of matching goblin's [`Error`](goblin::error::Error) is non-exhaustive, I included non-exhaustive path too.
 *
 */
typedef struct ParsingError {
  int64_t code;
  char *explanation;
} ParsingError;

/**
 * A C-compatible vector for `Vec<String>`.
 */
typedef struct CharVec {
  uintptr_t capacity;
  uintptr_t length;
  char **vec;
} CharVec;

/**
 *
 * A struct contains detailed information about the object.
 *
 * It contains some information even the object is an erroneous one to make error handling more verbose.
 *
 * If the error occurs on parsing:
 * - A file: `file_name` and `member_name`
 * - A Muti Architecture Mach-O file: `file_name`, `member_name` and `executable_format`
 * - An archive: `file_name`, `member_name` and `file_type`
 *
 * fields will be filled correctly and the rest will be:
 * - null (the fields which are string)
 * - blank (`member_name` and `libraries`)
 * - `false` (`is_64` and `is_stripped`).
 *
 */
typedef struct ULDDObj {
  /**
   * The name of the object.
   *
   * Objects inside Muti Architecture Mach-O files will be named as "n. file" due to they don't have file names.
   */
  char *file_name;
  /**
   * The location of objects in recursive files.
   *
   * This field is empty if the object is not in a recursive file (Like: Archives and Muti Architecture Mach-O files).
   *
   * The names in the vector is sorted as outer to inner.
   */
  struct CharVec member_name;
  /**
   * The type of the executable format of the object.
   */
  char *executable_format;
  /**
   * The field is true if the object is 64 bit otherwise it is 32 bit or the object is an erroneous one.
   */
  bool is_64;
  /**
   * The name of the OS it was compiled for.
   */
  char *os_type;
  /**
   * The type of the object.
   */
  char *file_type;
  /**
   * The field is true if the object was stripped from debug symbols otherwise it is not stripped or the object is an erroneous one .
   */
  bool is_stripped;
  /**
   * The ISA (CPU Architecture) the object compiled for.
   */
  char *cpu_type;
  /**
   * The specific CPU model the object compiled for.
   *
   * macOS only field. It is null pointer in other executable formats.
   */
  char *cpu_subtype;
  /**
   * The name/version of the linker.
   *
   * ELF/PE only field. It is null pointer in other executable formats.
   *
   * It returns the version of the linker in PE files.
   */
  char *interpreter;
  /**
   * A vector of libraries linked against the object.
   *
   * It is blank in COFF files because they are mostly PE object files therefore they don't have linked libraries against them.
   */
  struct CharVec libraries;
} ULDDObj;

/**
 * A struct packs (empty or filled) error and (successfully or not) read object.
 */
typedef struct ULDDObjResult {
  struct ParsingError error;
  struct ULDDObj obj;
} ULDDObjResult;

/**
 * A C-compatible vector for [`ULDDObjResult`].
 */
typedef struct ULDDObjResultVec {
  uintptr_t capacity;
  uintptr_t length;
  struct ULDDObjResult *vec;
} ULDDObjResultVec;

/**
 *
 * Parses the given buffer and returns a vector of parsed binaries.
 *
 * # Safety
 *
 * This function is null pointer-safe. If the file name is an invalid UTF-8 string and/or buffer pointer is a null pointer it will panic.
 *
 * Since the function returns a [`ULDDObjResultVec`] created by rust it has to be [deallocated](free_obj) by rust if it is done by other languages errors may occur.
 *
 */
struct ULDDObjResultVec read_obj(const char *file_name,
                                 const uint8_t *buffer,
                                 uintptr_t buffer_size,
                                 bool debugging);

/**
 *
 * # Safety
 *
 * This function is designed for deallocating [`ULDDObjResultVec`] created by rust. Trying to deallocating [`ULDDObjResultVec`] created by other languages may result with errors.
 *
 * It is null pointer-safe.
 *
 * ## Error codes:
 * - 0: No errors
 * - 1: `vec` field of [`ULDDObjResultVec`] is a null pointer
 *
 */
uint8_t free_obj(struct ULDDObjResultVec obj,
                 bool debugging);
