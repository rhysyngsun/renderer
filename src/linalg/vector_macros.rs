#![macro_use]

macro_rules! scalar_new_impl {
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

macro_rules! scalar_add_impl {
    ($t: ident, $( $s:ident ), +) => {
        impl<N : BaseNum> Add for $t<N> {
            type Output = $t<N>;

            #[inline]
            fn add(self, rhs: $t<N>) -> $t<N> {
                $t {
                    $( $s: self.$s + rhs.$s ), +
                }
            }
        }
    }
}


macro_rules! scalar_sub_impl {
    ($t: ident, $( $s:ident ), +) => {
        impl<N : BaseNum> Sub for $t<N> {
            type Output = $t<N>;

            #[inline]
            fn sub(self, rhs: $t<N>) -> $t<N> {
                $t {
                    $( $s: self.$s - rhs.$s ), +
                }
            }
        }
    }
}


macro_rules! scalar_div_impl {
    ($t: ident, $( $s:ident ), +) => {

        impl<N : BaseNum> Div<N> for $t<N> {
            type Output = $t<N>;

            #[inline]
            fn div(self, rhs: N) -> $t<N> {
                assert!(!::approx_eq(&rhs, &N::zero(), &N::approx_eps()));
                let inv = N::one() / rhs;
                $t {
                    $( $s: self.$s * inv ), +
                }
            }
        }
    }
}


macro_rules! scalar_mul_impl {
    ($t: ident, $( $s:ident ), +) => {

        impl<N : BaseNum> Mul<N> for $t<N> {
            type Output = $t<N>;

            #[inline]
            fn mul(self, rhs: N) -> $t<N> {
                $t {
                    $( $s: self.$s * rhs ), +
                }
            }
        }
    }
}


macro_rules! scalar_neg_impl {
    ($t: ident, $( $s:ident ), +) => {
        impl<N : BaseNum> Neg for $t<N> {
            type Output = $t<N>;
            #[inline]
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
            pub fn dot(&self, rhs: &$t<N>) -> N {
                add!($( self.$s * rhs.$s ), + )
            }
        }
    }
}

macro_rules! len_impl {
    ($t: ident, $( $s:ident ), +) => {
        impl<N : BaseNum> $t<N> {
            #[inline]
            pub fn length_squared(&self) -> N {
                add!($( self.$s * self.$s ), + )
            }
            #[inline]
            pub fn length(&self) -> N {
                N::sqrt(&self.length_squared())
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

            fn approx_eps() -> N {
                N::approx_eps()
            }
        }
    }
}

macro_rules! arbitrary_impl {
    ($t: ident, $( $s: ident), +) => {
        #[cfg(test)]
        impl<N : Arbitrary> Arbitrary for $t<N> {
            #[inline]
            fn arbitrary<G: Gen>(g: &mut G) -> $t<N> {
                $t { $($s: Arbitrary::arbitrary(g),) + }
            }
        }
    }
}

macro_rules! scalar_to_array_impl {
    ($t: ident, $len: expr) => {
        impl<N> $t<N> {
            fn to_array(&self) -> &[N; $len] {
                unsafe {
                    mem::transmute(self)
                }
            }

            fn to_array_mut(&mut self) -> &mut[N; $len] {
                unsafe {
                    mem::transmute(self)
                }
            }
        }
    }
}

macro_rules! scalar_index_impl {
    ($t: ident) => (
        impl<N : BaseNum> Index<usize> for $t<N> {
            type Output = N;

            fn index(&self, i: usize) -> &N {
                &self.to_array()[i]
            }
        }

        impl<N : BaseNum> IndexMut<usize> for $t<N> {
            fn index_mut(&mut self, i: usize) -> &mut N {
                &mut self.to_array_mut()[i]
            }
        }
    )

}
