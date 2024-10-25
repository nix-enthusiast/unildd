use phf::phf_map;
use std::fmt::{Display, Formatter, Result as FmtResult};

/*
Source:
https://en.wikipedia.org/wiki/Executable_and_Linkable_Format
https://en.wikipedia.org/wiki/Cell_(processor)
https://en.wikipedia.org/wiki/ETRAX_CRIS
https://developer.fedoraproject.org/deployment/secondary_architectures/s390.html#:~:text=s390%20is%2031%2Dbit%2Daddress,known%20as%20IBM%20System%20z.
https://www.infineon.com/cms/en/product/microcontroller/
*/

/// MP Stands for Microprocessor
/// BPF Stands for Berkeley Packet Filter
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug)]
pub enum ElfInstructionSet {
    Undefined,
    WE32100,
    SPARC,
    X86,
    M68k,
    M88k,
    IntelMCU,
    I860,
    MIPS,
    IBMSystem370,
    MIPSRS3000Le,
    FutureUse,
    HPPPRISC,
    I960,
    PPC,
    PPC64,
    S390,
    S390x,
    // Wikipedia or Google doesn't provide healthy info about what IBM SPU/SPC is
    IBMSPUSPC,
    NECV800,
    FR20,
    RH32,
    MotorolaRCE,
    Arm32,
    DigitalAlpha,
    SuperH,
    SPARCVersion9,
    SiemensTriCore,
    ArgonautRISCCore,
    H8300,
    H8300H,
    H8S,
    H8500,
    IA64,
    StanfordMIPSX,
    MotorolaColdFire,
    MotorolaM68HC12,
    FujitsuMMA,
    SiemensPCP,
    SonyCellCPU,
    DensoNDR1,
    MotorolaStarCore,
    ToyotaME16,
    STMicroelectronicsST100,
    AdvancedLogicCorpTinyJ,
    X86_64,
    SonyDSP,
    PDP10,
    PDP11,
    SiemensFX66,
    STMicroelectronicsST9Plus,
    STMicroelectronicsST7,
    MC68HC16,
    MC68HC11,
    MC68HC08,
    MC68HC05,
    SiliconGraphicsSVx,
    STMicroelectronicsST19,
    DigitalVAX,
    ETRAXCRIS,
    InfineonTechnologiesMP32,
    Element14DSP64,
    LSILogicDSP16,
    TMS320C6000,
    MCSTElbrusE2k,
    Arm64,
    ZilogZ80,
    RISCV,
    BPF,
    WDC65C816,
}

// Source: https://en.wikipedia.org/wiki/Executable_and_Linkable_Format
pub const E_MACHINE: phf::Map<u16, ElfInstructionSet> = phf_map! {
    0x00_u16 => ElfInstructionSet::Undefined,
    0x01_u16 => ElfInstructionSet::WE32100,
    0x02_u16 => ElfInstructionSet::SPARC,
    0x03_u16 => ElfInstructionSet::X86,
    0x04_u16 => ElfInstructionSet::M68k,
    0x05_u16 => ElfInstructionSet::M88k,
    0x06_u16 => ElfInstructionSet::IntelMCU,
    0x07_u16 => ElfInstructionSet::I860,
    0x08_u16 => ElfInstructionSet::MIPS,
    0x09_u16 => ElfInstructionSet::IBMSystem370,
    0x0A_u16 => ElfInstructionSet::MIPSRS3000Le,
    0x0B_u16 => ElfInstructionSet::FutureUse,
    0x0C_u16 => ElfInstructionSet::FutureUse,
    0x0D_u16 => ElfInstructionSet::FutureUse,
    0x0E_u16 => ElfInstructionSet::FutureUse,
    0x0F_u16 => ElfInstructionSet::HPPPRISC,
    0x13_u16 => ElfInstructionSet::I960,
    0x14_u16 => ElfInstructionSet::PPC,
    0x15_u16 => ElfInstructionSet::PPC64,
    0x16_u16 => ElfInstructionSet::S390,
    0x17_u16 => ElfInstructionSet::S390x,
    0x18_u16 => ElfInstructionSet::FutureUse,
    0x19_u16 => ElfInstructionSet::FutureUse,
    0x20_u16 => ElfInstructionSet::FutureUse,
    0x21_u16 => ElfInstructionSet::FutureUse,
    0x22_u16 => ElfInstructionSet::FutureUse,
    0x23_u16 => ElfInstructionSet::FutureUse,
    0x24_u16 => ElfInstructionSet::NECV800,
    0x25_u16 => ElfInstructionSet::FR20,
    0x26_u16 => ElfInstructionSet::RH32,
    0x27_u16 => ElfInstructionSet::MotorolaRCE,
    0x28_u16 => ElfInstructionSet::Arm32,
    0x29_u16 => ElfInstructionSet::DigitalAlpha,
    0x2A_u16 => ElfInstructionSet::SuperH,
    0x2B_u16 => ElfInstructionSet::SPARCVersion9,
    0x2C_u16 => ElfInstructionSet::SiemensTriCore,
    0x2D_u16 => ElfInstructionSet::ArgonautRISCCore,
    0x2E_u16 => ElfInstructionSet::H8300,
    0x2F_u16 => ElfInstructionSet::H8300H,
    0x30_u16 => ElfInstructionSet::H8S,
    0x31_u16 => ElfInstructionSet::H8500,
    0x32_u16 => ElfInstructionSet::IA64,
    0x33_u16 => ElfInstructionSet::StanfordMIPSX,
    0x34_u16 => ElfInstructionSet::MotorolaColdFire,
    0x35_u16 => ElfInstructionSet::MotorolaM68HC12,
    0x36_u16 => ElfInstructionSet::FujitsuMMA,
    0x37_u16 => ElfInstructionSet::SiemensPCP,
    0x38_u16 => ElfInstructionSet::SonyCellCPU,
    0x39_u16 => ElfInstructionSet::DensoNDR1,
    0x3A_u16 => ElfInstructionSet::MotorolaStarCore,
    0x3B_u16 => ElfInstructionSet::ToyotaME16,
    0x3C_u16 => ElfInstructionSet::STMicroelectronicsST100,
    0x3D_u16 => ElfInstructionSet::AdvancedLogicCorpTinyJ,
    0x3E_u16 => ElfInstructionSet::X86_64,
    0x3F_u16 => ElfInstructionSet::SonyDSP,
    0x40_u16 => ElfInstructionSet::PDP10,
    0x41_u16 => ElfInstructionSet::PDP11,
    0x42_u16 => ElfInstructionSet::SiemensFX66,
    0x43_u16 => ElfInstructionSet::STMicroelectronicsST9Plus,
    0x44_u16 => ElfInstructionSet::STMicroelectronicsST7,
    0x45_u16 => ElfInstructionSet::MC68HC16,
    0x46_u16 => ElfInstructionSet::MC68HC11,
    0x47_u16 => ElfInstructionSet::MC68HC08,
    0x48_u16 => ElfInstructionSet::MC68HC05,
    0x49_u16 => ElfInstructionSet::SiliconGraphicsSVx,
    0x4A_u16 => ElfInstructionSet::STMicroelectronicsST19,
    0x4B_u16 => ElfInstructionSet::DigitalVAX,
    0x4C_u16 => ElfInstructionSet::ETRAXCRIS,
    0x4D_u16 => ElfInstructionSet::InfineonTechnologiesMP32,
    0x4E_u16 => ElfInstructionSet::Element14DSP64,
    0x4F_u16 => ElfInstructionSet::LSILogicDSP16,
    0x8C_u16 => ElfInstructionSet::TMS320C6000,
    0xAF_u16 => ElfInstructionSet::MCSTElbrusE2k,
    0xB7_u16 => ElfInstructionSet::Arm64,
    0xDC_u16 => ElfInstructionSet::ZilogZ80,
    0xF3_u16 => ElfInstructionSet::RISCV,
    0xF7_u16 => ElfInstructionSet::BPF,
    0x101_u16 => ElfInstructionSet::WDC65C816,
};

// Source: https://en.wikipedia.org/wiki/Executable_and_Linkable_Format
#[allow(clippy::upper_case_acronyms)]
#[derive(Clone, Copy, Debug)]
pub enum ElfOS {
    SystemV,
    HPUX,
    NetBSD,
    Linux,
    GNUHurd,
    Solaris,
    AIXMonterey,
    IRIX,
    FreeBSD,
    Tru64,
    NovellModesto,
    OpenBSD,
    OpenVMS,
    NonStopKernel,
    AROS,
    FenixOS,
    CloudABI,
    OpenVOS,
    Illumos,
    SerenityOS,
    Android,
    Undefined,
}

/*
Source:
    https://en.wikipedia.org/wiki/Executable_and_Linkable_Format
    https://stackoverflow.com/a/49248689
*/
#[derive(Debug)]
pub enum ElfFileType {
    Undefined,
    Relocatable,
    Executable,
    SharedObject,
    CoreFile,
    OsSpecific,
    ProcessorSpecific,
}

// Source: https://en.wikipedia.org/wiki/Executable_and_Linkable_Format
pub const E_TYPE: phf::Map<u16, ElfFileType> = phf_map! {
    0x00_u16 => ElfFileType::Undefined,
    0x01_u16 => ElfFileType::Relocatable,
    0x02_u16 => ElfFileType::Executable,
    0x03_u16 => ElfFileType::SharedObject,
    0x04_u16 => ElfFileType::CoreFile,
    0xFE00_u16 => ElfFileType::OsSpecific,
    0xFEFF_u16 => ElfFileType::OsSpecific,
    0xFF00_u16 => ElfFileType::ProcessorSpecific,
    0xFFFF_u16 => ElfFileType::ProcessorSpecific,
};

// Source: https://en.wikipedia.org/wiki/Mach-O
#[derive(Debug)]
pub enum MachOCpuType {
    VAX,
    ROMP,
    NS32032,
    NS32332,
    MC680x0,
    X86,
    MIPS,
    NS32352,
    MC98000,
    HPPA,
    ARM,
    MC88000,
    SPARC,
    I860Be,
    I860Le,
    RS6000,
    PPC,
    ARM64,
    X86_64,
    Unknown,
}

/*
Source:
    https://en.wikipedia.org/wiki/Mach-O
    https://www.jviotti.com/2021/07/23/a-deep-dive-on-macos-universal-binaries.html
*/
pub const MACH_O_CPUTYPE: phf::Map<u32, MachOCpuType> = phf_map! {
    0x01_u32 => MachOCpuType::VAX,
    0x02_u32 => MachOCpuType::ROMP,
    0x04_u32 => MachOCpuType::NS32032,
    0x05_u32 => MachOCpuType::NS32332,
    0x06_u32 => MachOCpuType::MC680x0,
    0x07_u32 => MachOCpuType::X86,
    0x08_u32 => MachOCpuType::MIPS,
    0x09_u32 => MachOCpuType::NS32352,
    0x0A_u32 => MachOCpuType::MC98000,
    0x0B_u32 => MachOCpuType::HPPA,
    0x0C_u32 => MachOCpuType::ARM,
    0x0D_u32 => MachOCpuType::MC88000,
    0x0E_u32 => MachOCpuType::SPARC,
    0x0F_u32 => MachOCpuType::I860Be,
    0x10_u32 => MachOCpuType::I860Le,
    0x11_u32 => MachOCpuType::RS6000,
    0x12_u32 => MachOCpuType::PPC,
    0x1000007_u32 => MachOCpuType::X86_64,
    0x100000C_u32 => MachOCpuType::ARM64,
};

// Source: https://en.wikipedia.org/wiki/Mach-O
#[derive(Debug)]
pub enum MachOArmSubType {
    All,
    A500ArchOrNewer,
    A500OrNewer,
    A440OrNewer,
    M4OrNewer,
    V4TOrNewer,
    V6OrNewer,
    V5TEJOrNewer,
    XScaleOrNewer,
    V7OrNewer,
    V7FCortexA9OrNewer,
    V7SSwiftOrNewer,
    V7KKirkwood40OrNewer,
    V8OrNewer,
    V6MOrNewer,
    V7MOrNewer,
    V7EMOrNewer,
    Unknown,
}

// Source: https://en.wikipedia.org/wiki/Mach-O
pub const MACH_O_ARM_CPU_SUBTYPE: phf::Map<u32, MachOArmSubType> = phf_map! {
        0x00_u32 => MachOArmSubType::All,
        0x01_u32 => MachOArmSubType::A500ArchOrNewer,
        0x02_u32 => MachOArmSubType::A500OrNewer,
        0x03_u32 => MachOArmSubType::A440OrNewer,
        0x04_u32 => MachOArmSubType::M4OrNewer,
        0x05_u32 => MachOArmSubType::V4TOrNewer,
        0x06_u32 => MachOArmSubType::V6OrNewer,
        0x07_u32 => MachOArmSubType::V5TEJOrNewer,
        0x08_u32 => MachOArmSubType::XScaleOrNewer,
        0x09_u32 => MachOArmSubType::V7OrNewer,
        0x0A_u32 => MachOArmSubType::V7FCortexA9OrNewer,
        0x0B_u32 => MachOArmSubType::V7SSwiftOrNewer,
        0x0C_u32 => MachOArmSubType::V7KKirkwood40OrNewer,
        0x0D_u32 => MachOArmSubType::V8OrNewer,
        0x0E_u32 => MachOArmSubType::V6MOrNewer,
        0x0F_u32 => MachOArmSubType::V7MOrNewer,
        0x10_u32 => MachOArmSubType::V7EMOrNewer,
};

// Source: https://en.wikipedia.org/wiki/Mach-O
#[derive(Debug)]
pub enum MachOX86SubType {
    All,
    I486OrNewer,
    I486SXOrNewer,
    PentiumM5OrNewer,
    CeleronOrNewer,
    CeleronMobile,
    Pentium3OrNewer,
    Pentium3MOrNewer,
    Pentium3XeonOrNewer,
    Pentium4OrNewer,
    ItaniumOrNewer,
    Itanium2OrNewer,
    XeonOrNewer,
    XeonMPOrNewer,
    Undefined,
}

// Source: https://en.wikipedia.org/wiki/Mach-O
pub const MACH_O_X86_CPU_SUBTYPE: phf::Map<u32, MachOX86SubType> = phf_map! {
    0x03_u32 => MachOX86SubType::All,
    0x04_u32 => MachOX86SubType::I486OrNewer,
    0x84_u32 => MachOX86SubType::I486SXOrNewer,
    0x56_u32 => MachOX86SubType::PentiumM5OrNewer,
    0x67_u32 => MachOX86SubType::CeleronOrNewer,
    0x77_u32 => MachOX86SubType::CeleronMobile,
    0x08_u32 => MachOX86SubType::Pentium3OrNewer,
    0x18_u32 => MachOX86SubType::Pentium3MOrNewer,
    0x28_u32 => MachOX86SubType::Pentium3XeonOrNewer,
    0x0A_u32 => MachOX86SubType::Pentium4OrNewer,
    0x0B_u32 => MachOX86SubType::ItaniumOrNewer,
    0x1B_u32 => MachOX86SubType::Itanium2OrNewer,
    0x0C_u32 => MachOX86SubType::XeonOrNewer,
    0x1C_u32 => MachOX86SubType::XeonMPOrNewer,
};

#[derive(Debug)]
pub enum MachOCpuSubType {
    Arm(MachOArmSubType),
    X86(MachOX86SubType),
}

// Source: https://en.wikipedia.org/wiki/Mach-O
#[derive(Debug)]
pub enum MachOOs {
    MacOS,
    IOS,
    AppleTVBox,
    AppleWatch,
    BridgeOS,
    MacCatalyst,
    IOSSimulator,
    AppleTVSimulator,
    AppleWatchSimulator,
    DriverKit,
    AppleVisionPro,
    AppleVisionProSimulator,
    Undefined,
}

// Source: https://en.wikipedia.org/wiki/Mach-O
pub const MACH_O_OS: phf::Map<u32, MachOOs> = phf_map! {
    0x1_u32 => MachOOs::MacOS,
    0x2_u32 => MachOOs::IOS,
    0x3_u32 => MachOOs::AppleTVBox,
    0x4_u32 => MachOOs::AppleWatch,
    0x5_u32 => MachOOs::BridgeOS,
    0x6_u32 => MachOOs::MacCatalyst,
    0x7_u32 => MachOOs::IOSSimulator,
    0x8_u32 => MachOOs::AppleTVSimulator,
    0x9_u32 => MachOOs::AppleWatchSimulator,
    0xA_u32 => MachOOs::DriverKit,
    0xB_u32 => MachOOs::AppleVisionPro,
    0xC_u32 => MachOOs::AppleVisionProSimulator,
};

// Source: https://en.wikipedia.org/wiki/Mach-O
#[derive(Debug)]
pub enum MachOFileType {
    RelocatableObjectFile,
    DemandPagedExecutableFile,
    FixedVMSharedLibraryFile,
    CoreFile,
    PreloadedExecutableFile,
    DynamicallyBoundSharedLibraryFile,
    DynamicLinkEditor,
    DynamicallyBoundBundleFile,
    SharedLibraryStub,
    CompanionFileWithDebugSections,
    X86_64Kexts,
    ComposedFile,
    Undefined,
}

// Source: https://en.wikipedia.org/wiki/Mach-O
pub const MACH_O_FILE_TYPE: phf::Map<u32, MachOFileType> = phf_map! {
    0x1_u32 => MachOFileType::RelocatableObjectFile,
    0x2_u32 => MachOFileType::DemandPagedExecutableFile,
    0x3_u32 => MachOFileType::FixedVMSharedLibraryFile,
    0x4_u32 => MachOFileType::CoreFile,
    0x5_u32 => MachOFileType::PreloadedExecutableFile,
    0x6_u32 => MachOFileType::DynamicallyBoundSharedLibraryFile,
    0x7_u32 => MachOFileType::DynamicLinkEditor,
    0x8_u32 => MachOFileType::DynamicallyBoundBundleFile,
    0x9_u32 => MachOFileType::SharedLibraryStub,
    0xA_u32 => MachOFileType::CompanionFileWithDebugSections,
    0xB_u32 => MachOFileType::X86_64Kexts,
    0xC_u32 => MachOFileType::ComposedFile,
};

// Source: https://learn.microsoft.com/en-us/windows/win32/debug/pe-format
#[derive(Debug)]
pub enum PeArch {
    Unknown,
    AlphaAXP32,
    AlphaAXP64,
    MatsushitaAM33,
    X64,
    ARM,
    ARM64,
    ARMNT,
    AXP64,
    EBC,
    I386,
    IA64,
    LoongArch32,
    LoongArch64,
    MitsubishiM32R,
    MIPS16,
    MIPSFPU,
    MIPSFPU16,
    PowerPC,
    PowerPCFP,
    MIPS,
    RISCV32,
    RISCV64,
    RISCV128,
    HitachiSH3,
    HitachiSH3DSP,
    HitachiSH4,
    HitachiSH5,
    Thumb,
    MIPSWCE,
}

// Source: https://learn.microsoft.com/en-us/windows/win32/debug/pe-format
pub const PE_ARCH: phf::Map<u16, PeArch> = phf_map! {
    0x0000_u16 => PeArch::Unknown,
    0x0184_u16 => PeArch::AlphaAXP32,
    0x0284_u16 => PeArch::AlphaAXP64,
    0x01d3_u16 => PeArch::MatsushitaAM33,
    0x8664_u16 => PeArch::X64,
    0x01c0_u16 => PeArch::ARM,
    0xaa64_u16 => PeArch::ARM64,
    0x01c4_u16 => PeArch::ARMNT,
    // Look at source to learn why there is a duplicate value
    //0x0284_u16 => PeArch::AXP64,
    0xebc_u16 => PeArch::EBC,
    0x014c_u16 => PeArch::I386,
    0x0200_u16 => PeArch::IA64,
    0x6232_u16 => PeArch::LoongArch32,
    0x6264_u16 => PeArch::LoongArch64,
    0x9041_u16 => PeArch::MitsubishiM32R,
    0x0266_u16 => PeArch::MIPS16,
    0x0366_u16 => PeArch::MIPSFPU,
    0x0466_u16 => PeArch::MIPSFPU16,
    0x01f0_u16 => PeArch::PowerPC,
    0x01f1_u16 => PeArch::PowerPCFP,
    0x0166_u16 => PeArch::MIPS,
    0x5032_u16 => PeArch::RISCV32,
    0x5064_u16 => PeArch::RISCV64,
    0x5128_u16 => PeArch::RISCV128,
    0x01a2_u16 => PeArch::HitachiSH3,
    0x01a3_u16 => PeArch::HitachiSH3DSP,
    0x01a6_u16 => PeArch::HitachiSH4,
    0x01a8_u16 => PeArch::HitachiSH5,
    0x01c2_u16 => PeArch::Thumb,
    0x0169_u16 => PeArch::MIPSWCE,
};

// Source: https://learn.microsoft.com/en-us/windows/win32/debug/pe-format
#[derive(Debug)]
pub enum PeSubsystem {
    Unknown,
    Native,
    WindowsGUI,
    WindowsCUI,
    OS2CUI,
    PosixCUI,
    NativeWindows,
    WindowsCEGUI,
    EFIApplication,
    EFIBootServiceDriver,
    EFIRuntimeDriver,
    EFIRom,
    Xbox,
    WindowsBootApplication,
}

// Source: https://learn.microsoft.com/en-us/windows/win32/debug/pe-format
pub const PE_SUBSYSTEM: phf::Map<u16, PeSubsystem> = phf_map! {
    0x0000_u16 => PeSubsystem::Unknown,
    0x0001_u16 => PeSubsystem::Native,
    0x0002_u16 => PeSubsystem::WindowsGUI,
    0x0003_u16 => PeSubsystem::WindowsCUI,
    0x0005_u16 => PeSubsystem::OS2CUI,
    0x0007_u16 => PeSubsystem::PosixCUI,
    0x0008_u16 => PeSubsystem::NativeWindows,
    0x0009_u16 => PeSubsystem::WindowsCEGUI,
    0x000A_u16 => PeSubsystem::EFIApplication,
    0x000B_u16 => PeSubsystem::EFIBootServiceDriver,
    0x000C_u16 => PeSubsystem::EFIRuntimeDriver,
    0x000D_u16 => PeSubsystem::EFIRom,
    0x000E_u16 => PeSubsystem::Xbox,
    0x0010_u16 => PeSubsystem::WindowsBootApplication,
};

#[derive(Debug)]
pub enum PeOS {
    Xbox,
    Windows,
    UEFI,
    Undefined,
}

// Source of the names: https://en.wikipedia.org/wiki/Executable_and_Linkable_Format
impl Display for ElfOS {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            ElfOS::SystemV => write!(f, "System V"),
            ElfOS::HPUX => write!(f, "HP-UX"),
            ElfOS::NetBSD => write!(f, "NetBSD"),
            ElfOS::Linux => write!(f, "Linux"),
            ElfOS::GNUHurd => write!(f, "GNU Hurd"),
            ElfOS::Solaris => write!(f, "Solaris"),
            ElfOS::AIXMonterey => write!(f, "AIX (Monterey)"),
            ElfOS::IRIX => write!(f, "IRIX"),
            ElfOS::FreeBSD => write!(f, "FreeBSD"),
            ElfOS::Tru64 => write!(f, "Tru64"),
            ElfOS::NovellModesto => write!(f, "Novell Modesto"),
            ElfOS::OpenBSD => write!(f, "OpenBSD"),
            ElfOS::OpenVMS => write!(f, "OpenVMS"),
            ElfOS::NonStopKernel => write!(f, "NonStop Kernel"),
            ElfOS::AROS => write!(f, "AROS"),
            ElfOS::FenixOS => write!(f, "FenixOS"),
            ElfOS::CloudABI => write!(f, "Nuxi CloudABI"),
            ElfOS::OpenVOS => write!(f, "OpenVOS"),
            ElfOS::Illumos => write!(f, "Illumos"),
            ElfOS::SerenityOS => write!(f, "SerenityOS"),
            ElfOS::Android => write!(f, "Android"),
            ElfOS::Undefined => write!(f, "Undefined"),
        }
    }
}

// Source of the names: https://en.wikipedia.org/wiki/Executable_and_Linkable_Format
impl Display for ElfInstructionSet {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            ElfInstructionSet::Undefined => write!(f, "Undefined"), //0x00 no specific instruction set is defined
            ElfInstructionSet::WE32100 => write!(f, "AT&T WE 32100"),
            ElfInstructionSet::SPARC => write!(f, "SPARC"),
            ElfInstructionSet::X86 => write!(f, "x86"),
            ElfInstructionSet::M68k => write!(f, "Motorola 68000 (M68k)"),
            ElfInstructionSet::M88k => write!(f, "Motorola 88000 (M88k)"),
            ElfInstructionSet::IntelMCU => write!(f, "Intel MCU"),
            ElfInstructionSet::I860 => write!(f, "Intel 80860"),
            ElfInstructionSet::MIPS => write!(f, "MIPS"),
            ElfInstructionSet::IBMSystem370 => write!(f, "IBM System/370"),
            ElfInstructionSet::MIPSRS3000Le => write!(f, "MIPS RS3000 (Little-endian)"),
            ElfInstructionSet::FutureUse => write!(f, "Reserved for future use"),
            ElfInstructionSet::HPPPRISC => write!(f, "HP PA-RISC"),
            ElfInstructionSet::I960 => write!(f, "Intel 80960"),
            ElfInstructionSet::PPC => write!(f, "PowerPC"),
            ElfInstructionSet::PPC64 => write!(f, "PowerPC (64-Bit)"),
            ElfInstructionSet::S390 => write!(f, "S390"),
            ElfInstructionSet::S390x => write!(f, "S390x"),
            ElfInstructionSet::NECV800 => write!(f, "NEC V800"),
            ElfInstructionSet::FR20 => write!(f, "Fujitsu FR20"),
            ElfInstructionSet::RH32 => write!(f, "TRW RH-32"),
            ElfInstructionSet::MotorolaRCE => write!(f, "Motorola RCE"),
            ElfInstructionSet::Arm32 => write!(f, "Arm (32-Bit)"),
            ElfInstructionSet::DigitalAlpha => write!(f, "Digital Alpha"),
            ElfInstructionSet::SuperH => write!(f, "SuperH"),
            ElfInstructionSet::SPARCVersion9 => write!(f, "SPARC version 9"),
            ElfInstructionSet::SiemensTriCore => write!(f, "Siemens TriCore"),
            ElfInstructionSet::ArgonautRISCCore => write!(f, "Argonaut RISC Core"),
            ElfInstructionSet::H8300 => write!(f, "Hitachi H8/300"),
            ElfInstructionSet::H8300H => write!(f, "Hitachi H8/300H"),
            ElfInstructionSet::H8S => write!(f, "Hitachi H8S"),
            ElfInstructionSet::H8500 => write!(f, "Hitachi H8/500"),
            ElfInstructionSet::IA64 => write!(f, "Intel Itanium"),
            ElfInstructionSet::StanfordMIPSX => write!(f, "Stanford MIPS-X"),
            ElfInstructionSet::MotorolaColdFire => write!(f, "Motorola ColdFire"),
            ElfInstructionSet::MotorolaM68HC12 => write!(f, "Motorola M68HC12"),
            ElfInstructionSet::FujitsuMMA => write!(f, "Fujitsu MMA multimedia accelerator"),
            ElfInstructionSet::SiemensPCP => write!(f, "Siemens PCP"),
            ElfInstructionSet::SonyCellCPU => write!(f, "Sony nCPU Embedded RISC"),
            ElfInstructionSet::DensoNDR1 => write!(f, "Denso NDR1"),
            ElfInstructionSet::MotorolaStarCore => write!(f, "Motorola Star*Core"),
            ElfInstructionSet::ToyotaME16 => write!(f, "Toyota ME16"),
            ElfInstructionSet::STMicroelectronicsST100 => {
                write!(f, "STMicroelectronics ST100")
            }
            ElfInstructionSet::AdvancedLogicCorpTinyJ => {
                write!(f, "Advanced Logic Corp. TinyJ")
            }
            ElfInstructionSet::X86_64 => write!(f, "x86-64"),
            ElfInstructionSet::SonyDSP => write!(f, "Sony DSP processor"),
            ElfInstructionSet::PDP10 => write!(f, "Digital Equipment Corp. PDP-10"),
            ElfInstructionSet::PDP11 => write!(f, "Digital Equipment Corp. PDP-11"),
            ElfInstructionSet::SiemensFX66 => write!(f, "Siemens FX66 microcontroller"),
            ElfInstructionSet::STMicroelectronicsST9Plus => {
                write!(f, "STMicroelectronics ST9+ 8/16-Bit microcontroller")
            }
            ElfInstructionSet::STMicroelectronicsST7 => {
                write!(f, "STMicroelectronics ST7 8-Bit microcontroller")
            }
            ElfInstructionSet::MC68HC16 => write!(f, "Motorola MC68HC16 microcontroller"),
            ElfInstructionSet::MC68HC11 => write!(f, "Motorola MC68HC11 microcontroller"),
            ElfInstructionSet::MC68HC08 => write!(f, "Motorola MC68HC08 microcontroller"),
            ElfInstructionSet::MC68HC05 => write!(f, "Motorola MC68HC05 microcontroller"),
            ElfInstructionSet::SiliconGraphicsSVx => write!(f, "Silicon Graphics SVx"),
            ElfInstructionSet::STMicroelectronicsST19 => {
                write!(f, "STMicroelectronics ST19 8-Bit microcontroller")
            }
            ElfInstructionSet::DigitalVAX => write!(f, "Digital VAX"),
            ElfInstructionSet::ETRAXCRIS => {
                write!(f, "Axis Communications (32-Bit) embedded processor")
            }
            ElfInstructionSet::InfineonTechnologiesMP32 => {
                write!(f, "Infineon Technologies (32-Bit) embedded processor")
            }
            ElfInstructionSet::Element14DSP64 => write!(f, "Element 14 (64-Bit) DSP processor"),
            ElfInstructionSet::LSILogicDSP16 => write!(f, "LSI Logic 16-Bit DSP processor"),
            ElfInstructionSet::TMS320C6000 => write!(f, "TMS320C6000 family"),
            ElfInstructionSet::MCSTElbrusE2k => write!(f, "MCST Elbrus e2k"),
            ElfInstructionSet::Arm64 => write!(f, "Arm (64-Bit)"),
            ElfInstructionSet::ZilogZ80 => write!(f, "Zilog Z80"),
            ElfInstructionSet::RISCV => write!(f, "RISC-V"),
            ElfInstructionSet::BPF => write!(f, "Berkeley packet filter"),
            ElfInstructionSet::IBMSPUSPC => write!(f, "IBM SPU/SPC"),
            ElfInstructionSet::WDC65C816 => write!(f, "WDC 65C816"),
        }
    }
}

/*
Sources:
    https://en.wikipedia.org/wiki/Executable_and_Linkable_Format
    https://refspecs.linuxbase.org/elf/gabi4+/ch4.intro.html
*/
impl Display for ElfFileType {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            ElfFileType::Undefined => write!(f, "Undefined"),
            ElfFileType::Relocatable => write!(f, "Object file"),
            ElfFileType::Executable => write!(f, "Executable"),
            ElfFileType::SharedObject => write!(f, "Shared object"),
            ElfFileType::CoreFile => write!(f, "Core file"),
            // I couldn't get healthy info. about "OS/CPU specific"
            ElfFileType::OsSpecific => write!(f, "OS-specific"),
            ElfFileType::ProcessorSpecific => write!(f, "CPU-specific"),
        }
    }
}

// Source: https://learn.microsoft.com/en-us/windows/win32/debug/pe-format
impl Display for PeArch {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            PeArch::Unknown => write!(f, "Unknown"),
            PeArch::AlphaAXP32 => write!(f, "Alpha AXP (32-Bit)"),
            PeArch::AlphaAXP64 | PeArch::AXP64 => write!(f, "Alpha 64 (64-Bit)"),
            PeArch::MatsushitaAM33 => write!(f, "Matsushita AM33"),
            PeArch::X64 => write!(f, "x64"),
            PeArch::ARM => write!(f, "Arm"),
            PeArch::ARM64 => write!(f, "Arm (64-Bit)"),
            PeArch::ARMNT => write!(f, "ARM Thumb-2"),
            PeArch::EBC => write!(f, "EFI bytecode"),
            PeArch::I386 => write!(f, "Intel 386"),
            PeArch::IA64 => write!(f, "Intel Itanium"),
            PeArch::LoongArch32 => write!(f, "LoongArch (32-Bit)"),
            PeArch::LoongArch64 => write!(f, "LoongArch (64-Bit)"),
            PeArch::MitsubishiM32R => write!(f, "Mitsubishi M32R"),
            PeArch::MIPS16 => write!(f, "MIPS16"),
            PeArch::MIPSFPU => write!(f, "MIPS with FPU"),
            PeArch::MIPSFPU16 => write!(f, "MIPS16 with FPU"),
            PeArch::PowerPC => write!(f, "PowerPC"),
            PeArch::PowerPCFP => write!(f, "PowerPC with floating point support"),
            PeArch::MIPS => write!(f, "MIPS"),
            PeArch::RISCV32 => write!(f, "RISC-V (32-Bit)"),
            PeArch::RISCV64 => write!(f, "RISC-V (64-Bit)"),
            PeArch::RISCV128 => write!(f, "RISC-V 128-Bit"),
            PeArch::HitachiSH3 => write!(f, "Hitachi SH3"),
            PeArch::HitachiSH3DSP => write!(f, "Hitachi SH3 DSP"),
            PeArch::HitachiSH4 => write!(f, "Hitachi SH4"),
            PeArch::HitachiSH5 => write!(f, "Hitachi SH5"),
            PeArch::Thumb => write!(f, "Thumb"),
            PeArch::MIPSWCE => write!(f, "MIPS (Little-endian) WCE v2"),
        }
    }
}

/*
    Sources:
        https://learn.microsoft.com/en-us/windows/win32/debug/pe-format
        CLI != CUI: https://github.com/avelino/awesome-go/issues/282#issuecomment-73395067
*/
impl Display for PeSubsystem {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            PeSubsystem::Unknown => write!(f, "Undefined"),
            PeSubsystem::Native => write!(f, "Device drivers and native windows processes"),
            PeSubsystem::WindowsGUI => write!(f, "GUI application"),
            PeSubsystem::WindowsCUI => write!(f, "Windows CUI application"),
            PeSubsystem::OS2CUI => write!(f, "OS/2 CUI application"),
            PeSubsystem::PosixCUI => write!(f, "Posix CUI application"),
            PeSubsystem::NativeWindows => write!(f, "Native Win9x driver"),
            PeSubsystem::WindowsCEGUI => write!(f, "Windows CE application"),
            PeSubsystem::EFIApplication => write!(f, "EFI application"),
            PeSubsystem::EFIBootServiceDriver => write!(f, "EFI driver with boot services"),
            PeSubsystem::EFIRuntimeDriver => write!(f, "EFI driver with runtime services"),
            PeSubsystem::EFIRom => write!(f, "EFI ROM image"),
            PeSubsystem::Xbox => write!(f, "XBOX"),
            PeSubsystem::WindowsBootApplication => write!(f, "Windows boot application."),
        }
    }
}

// Source: https://learn.microsoft.com/en-us/windows/win32/debug/pe-format
impl Display for PeOS {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            PeOS::Xbox => write!(f, "Xbox"),
            PeOS::Windows => write!(f, "Windows"),
            PeOS::UEFI => write!(f, "UEFI"),
            PeOS::Undefined => write!(f, "Undefined"),
        }
    }
}

// Source: https://en.wikipedia.org/wiki/Mach-O
impl Display for MachOArmSubType {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            MachOArmSubType::All => write!(f, "All ARM processors"),
            MachOArmSubType::A500ArchOrNewer => write!(f, "ARM-A500 ARCH or newer"),
            MachOArmSubType::A500OrNewer => write!(f, "ARM-A500 or newer"),
            MachOArmSubType::A440OrNewer => write!(f, "ARM-A440 or newer"),
            MachOArmSubType::M4OrNewer => write!(f, "ARM-M4 or newer"),
            MachOArmSubType::V4TOrNewer => write!(f, "ARM-V4T or newer"),
            MachOArmSubType::V6OrNewer => write!(f, "ARM-V6 or newer"),
            MachOArmSubType::V5TEJOrNewer => write!(f, "ARM-V5TEJ or newer"),
            MachOArmSubType::XScaleOrNewer => write!(f, "ARM-XSCALE or newer"),
            MachOArmSubType::V7OrNewer => write!(f, "ARM-V7 or newer"),
            MachOArmSubType::V7FCortexA9OrNewer => write!(f, "ARM-V7F (Cortex A9) or newer"),
            MachOArmSubType::V7SSwiftOrNewer => write!(f, "ARM-V7S (Swift) or newer"),
            MachOArmSubType::V7KKirkwood40OrNewer => write!(f, "ARM-V7K (Kirkwood40) or newer"),
            MachOArmSubType::V8OrNewer => write!(f, "ARM-V8 or newer"),
            MachOArmSubType::V6MOrNewer => write!(f, "ARM-V6M or newer"),
            MachOArmSubType::V7MOrNewer => write!(f, "ARM-V7M or newer"),
            MachOArmSubType::V7EMOrNewer => write!(f, "ARM-V7EM or newer"),
            MachOArmSubType::Unknown => write!(f, "Unknown"),
        }
    }
}

// Source: https://en.wikipedia.org/wiki/Mach-O
impl Display for MachOX86SubType {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            MachOX86SubType::All => write!(f, "All x86 processors"),
            MachOX86SubType::I486OrNewer => write!(f, "486 or newer"),
            MachOX86SubType::I486SXOrNewer => write!(f, "486SX or newer"),
            MachOX86SubType::PentiumM5OrNewer => write!(f, "Pentium M5 or newer"),
            MachOX86SubType::CeleronOrNewer => write!(f, "Celeron or newer"),
            MachOX86SubType::CeleronMobile => write!(f, "Celeron Mobile"),
            MachOX86SubType::Pentium3OrNewer => write!(f, "Pentium 3 or newer"),
            MachOX86SubType::Pentium3MOrNewer => write!(f, "Pentium 3-M or newer"),
            MachOX86SubType::Pentium3XeonOrNewer => write!(f, "Pentium 3-XEON or newer"),
            MachOX86SubType::Pentium4OrNewer => write!(f, "Pentium-4 or newer"),
            MachOX86SubType::ItaniumOrNewer => write!(f, "Itanium or newer"),
            MachOX86SubType::Itanium2OrNewer => write!(f, "Itanium-2 or newer"),
            MachOX86SubType::XeonOrNewer => write!(f, "XEON or newer"),
            MachOX86SubType::XeonMPOrNewer => write!(f, "XEON-MP or newer"),
            MachOX86SubType::Undefined => write!(f, "Undefined"),
        }
    }
}

// Source: https://en.wikipedia.org/wiki/Mach-O
impl Display for MachOCpuSubType {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            MachOCpuSubType::Arm(arm) => write!(f, "{}", arm),
            MachOCpuSubType::X86(x86) => write!(f, "{}", x86),
        }
    }
}

// Source: https://en.wikipedia.org/wiki/Mach-O
impl Display for MachOCpuType {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            MachOCpuType::VAX => write!(f, "VAX"),
            MachOCpuType::ROMP => write!(f, "ROMP"),
            MachOCpuType::NS32032 => write!(f, "NS32032"),
            MachOCpuType::NS32332 => write!(f, "NS32332"),
            MachOCpuType::MC680x0 => write!(f, "MC680x0"),
            MachOCpuType::X86 => write!(f, "x86"),
            MachOCpuType::MIPS => write!(f, "MIPS"),
            MachOCpuType::NS32352 => write!(f, "NS32352"),
            MachOCpuType::MC98000 => write!(f, "MC98000"),
            MachOCpuType::HPPA => write!(f, "HP-PA"),
            MachOCpuType::ARM => write!(f, "ARM"),
            MachOCpuType::MC88000 => write!(f, "MC88000"),
            MachOCpuType::SPARC => write!(f, "SPARC"),
            MachOCpuType::I860Be => write!(f, "i860 (Big-endian)"),
            MachOCpuType::I860Le => write!(f, "i860 (Little-endian)"),
            MachOCpuType::RS6000 => write!(f, "RS/6000"),
            MachOCpuType::PPC => write!(f, "PowerPC"),
            MachOCpuType::ARM64 => write!(f, "ARM64"),
            MachOCpuType::X86_64 => write!(f, "x86_64"),
            MachOCpuType::Unknown => write!(f, "Unknown"),
        }
    }
}

// Source: https://en.wikipedia.org/wiki/Mach-O
impl Display for MachOFileType {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            MachOFileType::RelocatableObjectFile => write!(f, "Object file"),
            MachOFileType::DemandPagedExecutableFile => write!(f, "Executable"),
            MachOFileType::FixedVMSharedLibraryFile => write!(f, "Fixed VM shared library file"),
            MachOFileType::CoreFile => write!(f, "Core file"),
            MachOFileType::PreloadedExecutableFile => write!(f, "Preloaded executable file"),
            MachOFileType::DynamicallyBoundSharedLibraryFile => write!(f, "Shared object"),
            MachOFileType::DynamicLinkEditor => write!(f, "Dynamic link editor"),
            MachOFileType::DynamicallyBoundBundleFile => write!(f, "Dynamically bound bundle file"),
            MachOFileType::SharedLibraryStub => write!(f, "Shared library stub for static linking only, no section contents"),
            MachOFileType::CompanionFileWithDebugSections => write!(f, "Companion file with only debug sections"),
            MachOFileType::X86_64Kexts => write!(f, "x86_64 kext"),
            MachOFileType::ComposedFile => write!(f, "File composed of other Mach-Os to be run in the same userspace sharing a single linkedit"),
            MachOFileType::Undefined => write!(f, "Undefined"),
        }
    }
}

// Source: https://en.wikipedia.org/wiki/Mach-O
impl Display for MachOOs {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            MachOOs::MacOS => write!(f, "MacOS"),
            MachOOs::IOS => write!(f, "IOS"),
            MachOOs::AppleTVBox => write!(f, "Apple TV Box"),
            MachOOs::AppleWatch => write!(f, "Apple Watch"),
            MachOOs::BridgeOS => write!(f, "Bridge OS"),
            MachOOs::MacCatalyst => write!(f, "Mac Catalyst"),
            MachOOs::IOSSimulator => write!(f, "IOS simulator"),
            MachOOs::AppleTVSimulator => write!(f, "Apple TV simulator"),
            MachOOs::AppleWatchSimulator => write!(f, "Apple watch simulator"),
            MachOOs::DriverKit => write!(f, "Driver KIT"),
            MachOOs::AppleVisionPro => write!(f, "Apple Vision Pro"),
            MachOOs::AppleVisionProSimulator => write!(f, "Apple Vision Pro simulator"),
            MachOOs::Undefined => write!(f, "Undefined"),
        }
    }
}
