#![feature(generic_const_exprs)]

struct Struct;
trait Trait {
    const CONST: usize = 0;
}
impl Trait for Struct {}

fn f_<T>() {}

trait DynTrait {
    fn f(&self);
}

impl<T> DynTrait for T
where
    T: Trait,
    [(); T::CONST]:,
{
    fn f(&self) {
        f_::<T>()
    }
}

fn main() {
    Box::new(Struct).f();
}
