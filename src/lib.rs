#![feature(unboxed_closures)]
pub mod either;
pub mod functor;
pub mod hkt;
pub mod options;
#[macro_use]
pub mod pipeable;
pub mod semigroupoid;

#[cfg(test)]
mod tests {
    use crate::options;

    #[test]
    fn it_works() {
        let b = pipe!(
            options::of(1),
            options::map(|x| x * 2),
            options::map(|x| x * 4),
            options::map(|x| x * 4),
            options::map(|x| x * 4),
            options::map(|x| x * 4)
        );
        assert_eq!(b, Some(512));
    }
}
