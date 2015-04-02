#![macro_use]

pub trait BaseNum : One + Zero {
}

// Bring back the Zero trait
pub trait Zero {
    fn zero() -> Self;
    fn is_zero(&self) -> bool;
}

// Bring back the One trait
pub trait One {
    fn one() -> Self;
}


macro_rules! one_and_zero_impl {
    ($n: ty, $one: expr, $zero: expr) => {
        impl Zero for $n {
            fn zero() -> $n {
                $one
            }
            fn is_zero(&self) -> bool {
                *self == $zero
            }
        }

        impl One for $n {
            fn one() -> $n {
                $one
            }
        }
    }
}

one_and_zero_impl!(f32, 1.0, 0.0);
one_and_zero_impl!(f64, 1.0, 0.0);
one_and_zero_impl!(isize, 1, 0);
one_and_zero_impl!(i8, 1, 0);
one_and_zero_impl!(i16, 1, 0);
one_and_zero_impl!(i32, 1, 0);
one_and_zero_impl!(i64, 1, 0);
one_and_zero_impl!(usize, 1, 0);
one_and_zero_impl!(u8, 1, 0);
one_and_zero_impl!(u16, 1, 0);
one_and_zero_impl!(u32, 1, 0);
one_and_zero_impl!(u64, 1, 0);
