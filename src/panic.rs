#[cold]
#[track_caller]
pub(crate) const fn mul() -> ! {
    panic!("attempt to multiply with overflow")
}

#[cold]
#[track_caller]
pub(crate) const fn ilog2() -> ! {
    panic!("attempt to compute integer binary logarithm with overflow")
}
