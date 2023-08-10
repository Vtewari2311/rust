use crate::spec::{cvs, RelroLevel, TargetOptions};

pub fn opts() -> TargetOptions {
    TargetOptions {
        os: "hurd".into(),
        dynamic_linking: true,
        // executables: true,
        families: cvs!["unix"],
        // linker_is_gnu: true,
        has_rpath: true,
        position_independent_executables: true,
        relro_level: RelroLevel::Full,
        has_thread_local: true,
        // has_elf_tls: true,
        crt_static_respected: true,
        /*
        supported_split_debuginfo: Cow::Borrowed(&[
            SplitDebuginfo::Packed,
            SplitDebuginfo::Unpacked,
            SplitDebuginfo::Off,
        ]),
        */
        ..Default::default()
    }
}