use crate::either::Either;
use crate::semigroup::{SemiTest, Semigroup};

pub fn of<T>(elt: T) -> Option<T> {
    Option::Some(elt)
}

/// Map function ready to be composed
pub fn map<T, R, F>(f: F) -> impl FnOnce(Option<T>) -> Option<R>
where
    F: FnOnce(T) -> R,
{
    move |elt: Option<T>| match elt {
        Some(x) => Some(f(x)),
        None => None,
    }
}

/// Convert a Either to an Option
pub fn from_either<E, A>(either: Either<E, A>) -> Option<A> {
    either.ok()
}

/// <A, B>(f: (a: A) => Option<B>) => (ma: Option<A>) => Option<B>
pub fn chain<A, B, F: FnOnce(A) -> Option<B>>(f: F) -> impl FnOnce(Option<A>) -> Option<B> {
    move |elt| match elt {
        Some(x) => f(x),
        None => None,
    }
}

///
pub fn get_or_else<A, F: FnOnce() -> A>(on_none: F) -> impl FnOnce(Option<A>) -> A {
    move |elt| match elt {
        None => on_none(),
        Some(x) => x,
    }
}

/*
pub fn getApplySemigroup<A>(s: impl Semigroup<A>) -> impl Semigroup<Option<A>> {
    SemiTest<Option<A>> {
        concat: Box::new(|x: Option<A>, y: Option<A>| {
            match (x, y) {
                (Some(xval), Some(yval)) => Some(s::concat(xval, yval)),
                _ => None,
            }
        })
    }
}
*/
