use once_cell::sync::Lazy;
use std::sync::atomic::{AtomicBool, Ordering};

static GLOBAL_DRY_RUN: Lazy<AtomicBool> = Lazy::new(|| AtomicBool::new(false));

pub fn set_global_dry_run(value: bool) {
    GLOBAL_DRY_RUN.store(value, Ordering::SeqCst);
}

pub fn global_dry_run() -> bool {
    GLOBAL_DRY_RUN.load(Ordering::SeqCst)
}
