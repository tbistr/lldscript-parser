use std::string;

// define commands keywords
pub enum Command {
    Entry,
    Include,
    Input,
    Group,
    AsNeeded,
    Output,
    SearchDir,
    Startup,
    OutputFormat,
    Target,
    RegionAlias,
    Assert,
    Extern,
    ForceCommonAllocation,
    InhibitCommonAllocation,
    ForceGroupAllocation,
    Insert,
    Nocrossrefs,
    NocrossrefsTo,
    OutputArch,
    LdFeatuer,
    Section,
    Memory,
    Phdrs,
    Version,
}

impl string::ToString for Command {
    fn to_string(&self) -> String {
        match self {
            Command::Entry => "ENTRY",
            Command::Include => "INCLUDE",
            Command::Input => "INCLUDE",
            Command::Group => "GROUP",
            Command::AsNeeded => "AS_NEEDED",
            Command::Output => "OUTPUT",
            Command::SearchDir => "SEARCH_DIR",
            Command::Startup => "STARTUP",
            Command::OutputFormat => "OUTPUT_FORMAT",
            Command::Target => "TARGET",
            Command::RegionAlias => "REGION_ALIAS",
            Command::Assert => "ASSERT",
            Command::Extern => "EXTERN",
            Command::ForceCommonAllocation => "FORCE_COMMON_ALLOCATION",
            Command::InhibitCommonAllocation => "INHIBIT_COMMON_ALLOCATION",
            Command::ForceGroupAllocation => "FORCE_G",
            Command::Insert => "",
            Command::Nocrossrefs => "",
            Command::NocrossrefsTo => "",
            Command::OutputArch => "",
            Command::LdFeatuer => "",
            Command::Section => "",
            Command::Memory => "",
            Command::Phdrs => "",
            Command::Version => "",
        }
        .to_string()
    }
}
enum AssignmentAttr {
    Hidden,
    Provide,
    ProvideHidden,
}
