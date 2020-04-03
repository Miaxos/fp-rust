pub struct SemiTest<A> {
    concat: Box<dyn Fn(&dyn Fn(A, A) -> A)>,
}

/// A `Semigroup` is a `Magma` where `concat` is associative
pub trait Semigroup<A> {
    fn concat(x: A, y: A) -> A;
}

/// Boolean semigroup under conjunction
pub struct SemigroupAll;

impl Semigroup<bool> for SemigroupAll {
    fn concat(x: bool, y: bool) -> bool {
        x && y
    }
}

/// Boolean semigroup under disjunction
pub struct SemigroupAny;

impl Semigroup<bool> for SemigroupAny {
    fn concat(x: bool, y: bool) -> bool {
        x || y
    }
}

/*
*/

/// Number `Semigroup` under addition
pub struct SemigroupSum;

macro_rules! semigroup_numeric_impl {
    ($($t:ty)*) => ($(
        impl Semigroup<$t> for SemigroupSum {
            fn concat(x: $t, y: $t) -> $t {
                x + y
            }
        }
    )*)
}

semigroup_numeric_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 f64 }

/// Number `Semigroup` under multiplication
pub struct SemigroupProduct;

macro_rules! semigroup_numeric_impl_multiplication {
    ($($t:ty)*) => ($(
        impl Semigroup<$t> for SemigroupProduct {
            fn concat(x: $t, y: $t) -> $t {
                x * y
            }
        }
    )*)
}

semigroup_numeric_impl_multiplication! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 f64 }

/// String `Semigroup` under concatenation
pub struct SemigroupString;

impl Semigroup<String> for SemigroupString {
    fn concat(x: String, y: String) -> String {
        format!("{}{}", x, y)
    }
}

/*
macro_rules! semigroup_numeric_impl {
    ($($t:ty)*) => ($(
        impl Semigroup for $t {
            fn combine(self, other: Self) -> Self {
                self + other
            }
        }
    )*)
}
*/
