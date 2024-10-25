![banner](media/banner/UniLDD%20Banner.png)
 
### UniLDD is designed to bring parsing objects to any language (has a C FFI library).

### ⭐️ Features:
  - Detailed information! Some of them are:
    - Name of the OS (Windows, macOS, Linux, etc.)
    - File type (Core dump, shared library, executable, etc.)
    - ISA type (X86_64, ARM64, RISC-V, etc.)
    - CPU Subtype[^1]
    - Name of the linker[^2]
    - Which libraries are linked against
  - Parses without loading objects. Therefore, you can even parse shady objects like malwares![^3]
  - Error codes and explanations to make error handling easier.
  - A Basic and built-in logger to get real-time information.

### Installation
Basically: 
- Clone the git repository:

  `git clone https://github.com/nix-enthusiast/unildd.git`

- Go into the git repository:

  `cd unildd`

- Compile build with cargo:
  
  `cargo build --release`

- Put the output files to desired destination:
  
  `cp target/release/{libunildd.so,libunildd.a} /my/amazing/project/`

- Also do not forget to copy the header file:
  
  `cp header/unildd.h /my/amazing/project/`  

### License
This library is licensed under [BSD-3 Clause License](https://choosealicense.com/licenses/bsd-3-clause/) 

The resources used to make this library are cited as comments in the respective source files which they were used.

### 🎉 Thanks to:
  - [m4b](https://github.com/m4b) for the [goblin](https://github.com/m4b/goblin) crate which this library gets its power by!

[^1]:  CPU subtype is a macOS-only feature which tells what kind of CPU model the code is optimized for.

[^2]: It has some caveats. See the wiki (currently work-in-progress) for further details.

[^3]: That doesn't mean I am liable for any damages done by this project and files you parsed. Take your own risk!
