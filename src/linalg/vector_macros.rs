#![macro_use]

macro_rules! new_impl {
    ($t: ident, $( $s:ident ), +) => {
        impl<N : BaseNum> $t<N> {
            #[inline]
            pub fn new($( $s: N ), +) -> $t<N> {
                $t {
                    $( $s: $s ), +
                }
            }
        }
    }
}

macro_rules! add_impl {
    ($t: ident, $( $s:ident ), +) => {
        impl<N : BaseNum> Add for $t<N> {
            type Output = $t<N>;

            fn add(self, rhs: $t<N>) -> $t<N> {
                $t {
                    $( $s: self.$s + rhs.$s ), +
                }
            }
        }
    }
}


macro_rules! sub_impl {
    ($t: ident, $( $s:ident ), +) => {
        impl<N : BaseNum> Sub for $t<N> {
            type Output = $t<N>;

            fn sub(self, rhs: $t<N>) -> $t<N> {
                $t {
                    $( $s: self.$s - rhs.$s ), +
                }
            }
        }
    }
}


macro_rules! div_impl {
    ($t: ident, $( $s:ident ), +) => {

        impl<N : BaseNum> Div<N> for $t<N> {
            type Output = $t<N>;

            fn div(self, rhs: N) -> $t<N> {
                assert!(rhs != N::zero());
                let inv = N::one() / rhs;
                $t {
                    $( $s: self.$s * inv ), +
                }
            }
        }
    }
}


macro_rules! mul_impl {
    ($t: ident, $( $s:ident ), +) => {

        impl<N : BaseNum> Mul<N> for $t<N> {
            type Output = $t<N>;

            fn mul(self, rhs: N) -> $t<N> {
                $t {
                    $( $s: self.$s * rhs ), +
                }
            }
        }
    }
}


macro_rules! neg_impl {
    ($t: ident, $( $s:ident ), +) => {

        impl<N : BaseNum> Neg for $t<N> {
            type Output = $t<N>;

            fn neg(self) -> $t<N> {
                $t {
                    $( $s: -self.$s ), +
                }
            }
        }
    }
}

macro_rules! add (
    // base case
    ($x:expr) => {
        $x
    };
    // `$x` followed by at least one `$y,`
    ($x:expr, $($y:expr),+) => {
        // call min! on the tail `$y`
        Add::add($x, add!($($y),+))
    }
);

macro_rules! dot_impl {
    ($t: ident, $( $s:ident ), +) => {

        impl<N : BaseNum> $t<N> {
            #[inline]
            fn dot(&self, rhs: &$t<N>) -> N {
                add!($( self.$s * rhs.$s ), + )
            }
        }
    }
}

macro_rules! len_impl {
    ($t: ident, $( $s:ident ), +) => {
        impl<N : BaseNum> $t<N> {

            pub fn length_squared(&self) -> N {
                add!($( self.$s * self.$s ), + )
            }

            pub fn length(&self) -> N {
                NumCast::from(self.length_squared().to_f64().unwrap().sqrt()).unwrap()
            }
        }
    }
}

macro_rules! zero_impl {
    ($t: ident, $( $s:ident ), +) => {
        impl<N : BaseNum> Zero for $t<N> {
            #[inline]
            fn zero() -> Self {
                $t {
                    $( $s: N::zero() ), +
                }
            }
            #[inline]
            fn is_zero(&self) -> bool {
                $( self.$s.is_zero() ) && +
            }
        }
    }
}

macro_rules! one_impl {
    ($t: ident, $( $s:ident ), +) => {
        impl<N : BaseNum> One for $t<N> {
            #[inline]
            fn one() -> Self {
                $t {
                    $( $s: N::one() ), +
                }
            }
        }
    }
}

macro_rules! approx_eq_impl {
    ($t: ident, $( $s: ident ), +) => {
        impl<N : BaseNum> ApproxEq<N> for $t<N> {
            fn approx_eq(&self, other: &$t<N>, eps: &N) -> bool {
                $(::approx_eq(&self.$s, &other.$s, eps))&&+
            }
        }
    }
}

macro_rules! arbitrary_impl {
    ($t: ident, $( $s: ident), +) => {
        #[cfg(test)]
        impl<N : BaseNum + Arbitrary> Arbitrary for $t<N> {
            #[inline]
            fn arbitrary<G: Gen>(g: &mut G) -> $t<N> {
                $t { $($s: Arbitrary::arbitrary(g),)* }
            }
        }
    }
}

macro_rules! impl_to_array {
    ($t: ident, $len: expr) => {
        impl<N> $t<N> {
            fn to_array(&self) -> &[N; $len] {
                unsafe {
                    mem::transmute(self)
                }
            }
        }
    }
}
