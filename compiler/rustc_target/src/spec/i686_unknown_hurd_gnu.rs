use crate::spec::{Cc, LinkerFlavor, Lld, StackProbeType, Target};

pub fn target() -> Target {
    let mut base = super::hurd_gnu_base::opts();
    base.cpu = "pentiumpro".into();
    base.max_atomic_width = Some(64);
    // base.supported_sanitizers = SanitizerSet::ADDRESS;
    // base.pre_link_args.entry(LinkerFlavor::Gcc).or_default().push("-m32".to_string());
    base.add_pre_link_args(LinkerFlavor::Gnu(Cc::Yes, Lld::No), &["-m32"]);
    base.stack_probes = StackProbeType::InlineOrCall { min_llvm_version_for_inline: (11, 0, 1) };

    Target {
        llvm_target: "i686-unknown-hurd-gnu".into(),
        pointer_width: 32,
        data_layout: "e-m:e-p:32:32-p270:32:32-p271:32:32-p272:64:64-\
            f64:32:64-f80:32-n8:16:32-S128"
            .into(),
        arch: "x86".into(),
        options: base,
    }
}
