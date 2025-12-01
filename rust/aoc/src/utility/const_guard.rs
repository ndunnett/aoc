pub struct Guard<const ASSERTION: bool>;

pub trait True {}
impl True for Guard<true> {}

pub trait False {}
impl False for Guard<false> {}
