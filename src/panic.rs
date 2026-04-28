#[cold]
#[track_caller]
pub(crate) const fn add() -> ! {
    panic!("attempt to add with overflow")
}

#[cold]
#[track_caller]
pub(crate) const fn div() -> ! {
    panic!("attempt to divide with overflow")
}

#[cold]
#[track_caller]
pub(crate) const fn from() -> ! {
    panic!("attempt to convert with overflow")
}

#[cold]
#[track_caller]
pub(crate) const fn ilog2() -> ! {
    panic!("attempt to compute integer binary logarithm with overflow")
}

#[cold]
#[track_caller]
pub(crate) const fn mul() -> ! {
    panic!("attempt to multiply with overflow")
}

#[cold]
#[track_caller]
pub(crate) const fn rescale() -> ! {
    panic!("attempt to rescale with overflow")
}

#[cold]
#[track_caller]
pub(crate) const fn sub() -> ! {
    panic!("attempt to subtract with overflow")
}
