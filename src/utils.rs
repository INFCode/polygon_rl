// Helper type & trait around to provide a clearer compile error message
// and as well a clearer way to express extra bounds for const generics
pub struct Require<const C: bool>;
pub trait Cond {}
impl Cond for Require<true> {}
