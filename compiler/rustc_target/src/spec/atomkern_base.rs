use crate::spec::{LinkerFlavor, TargetOptions, StackProbeType};
use super::Lld;
use super::Cc;

pub fn opts() -> TargetOptions {
    TargetOptions {
        os: "atomkern".into(),
        executables: true,
        linker: Some("rust-lld".into()),
        linker_flavor: LinkerFlavor::Gnu(Cc::No, Lld::Yes),
        stack_probes: StackProbeType::Inline,
        ..Default::default()
    }
}
