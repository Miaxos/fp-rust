use crate::hkt::HKT2;

pub trait Semigroupoid {
    type Current;
    fn compose<E, A, B, AB: HKT2<A, B>, LA: HKT2<E, A>, R: HKT2<E, B>>(ab: AB, la: LA) -> R;
}
