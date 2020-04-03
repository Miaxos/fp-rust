pub trait HKT<A> {
    type Current;
    type Target;
}

macro_rules! derive_hkt {
    ($t:ident) => {
        impl<T, A> HKT<A> for $t<T> {
            type Current = T;
            type Target = $t<A>;
        }
    };
}

pub trait HKT2<E, A> {
    type Current;
    type Target;
    type Error;
}

macro_rules! derive_hkt2 {
    ($t:ident) => {
        impl<T, E, A> HKT2<E, A> for $t<T> {
            type Current = T;
            type Target = $t<A, E>;
        }
    };
}

derive_hkt!(Option);
derive_hkt!(Vec);

// HKT For result
impl<T, U, E> HKT<U> for Result<T, E> {
    type Current = T;
    type Target = Result<U, E>;
}

/*
impl<URI, E, A> HKT<URI, A> for Result<A, E> {
    type Current = URI;
    type Target = Result<A, E>;
}

impl<URI, E, A> HKT<URI, A> for HKT2<URI, E, A> {
    type Current = URI;
    type Target = A;
}
*/

/*
 *
 *
 *
pub trait HKT3<URI, R, E, A> {
    type Current = URI;
    type Target = A;
    type Error = E;
}

impl<URI, E, A> HKT<URI, A> for HKT2<URI, E, A> {
    type Current = URI;
    type Error = E;
}

macro_rules! derive_hkt {
    ($t: ident) => {
        impl<T, U> HKT<U> for $t<T> {
            type Current = T;
            type Target = $t<U>;
        }
    };
}

derive_hkt!(Option);
derive_hkt!(Vec);

impl<T, U, E> HKT<U> for Result<T, E> {
    type Current = T;
    type Target = Result<U, E>;
}

pub trait HKT3<U1, U2> {
    type Current1;
    type Current2;
    type Target;
}

macro_rules! derive_hkt3 {
    ($t:ident) => {
        impl<T1, T2, U1, U2> HKT3<U1, U2> for $t<T1, T2> {
            // The currently contained types
            type Current1 = T1;
            type Current2 = T2;
            // How the U's get filled in.
            type Target = $t<U1, U2>;
        }
    };
}
*/
