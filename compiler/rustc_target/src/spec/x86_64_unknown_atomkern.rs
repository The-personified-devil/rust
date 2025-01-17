use crate::spec::{Target, PanicStrategy};

pub fn target() -> Target {
    let mut base = super::atomkern_base::opts();
    base.cpu = "x86-64".into();
    base.disable_redzone = true;
    base.panic_strategy = PanicStrategy::Abort;
    base.features = "-mmx,-sse,+soft-float".into();

    Target {
        llvm_target: "x86_64-unknown-none".into(),
        pointer_width: 64,
        data_layout: "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128".into(),
        arch: "x86_64".into(),
        options: base,
    }
}
