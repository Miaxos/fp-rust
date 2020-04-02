use crate::hkt::HKT;

pub trait Functor<A>: HKT<A> {
    fn fmap<F>(self, f: F) -> Self::Target
    where
        F: FnOnce(Self::Current) -> A;
}

impl<T, U> Functor<U> for Option<T> {
    fn fmap<F>(self, f: F) -> Option<U>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            Some(value) => Some(f(value)),
            None => None,
        }
    }
}
