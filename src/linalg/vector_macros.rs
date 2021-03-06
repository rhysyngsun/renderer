#![macro_use]

macro_rules! scalar_new_impl {
    ($t: ident, $( $s:ident ), +) => {
        impl $t {
            #[inline]
            pub fn new($( $s: f64 ), +) -> $t {
                $t {
                    $( $s: $s ), +
                }
            }
        }
    }
}

macro_rules! scalar_add_impl {
    ($t: ident, $( $s:ident ), +) => {
        impl Add for $t {
            type Output = $t;

            #[inline]
            fn add(self, rhs: $t) -> $t {
                $t {
                    $( $s: self.$s + rhs.$s ), +
                }
            }
        }
    }
}


macro_rules! scalar_sub_impl {
    ($t: ident, $( $s:ident ), +) => {
        impl Sub for $t {
            type Output = $t;

            #[inline]
            fn sub(self, rhs: $t) -> $t {
                $t {
                    $( $s: self.$s - rhs.$s ), +
                }
            }
        }
    }
}


macro_rules! scalar_div_impl {
    ($t: ident, $( $s:ident ), +) => {

        impl Div<f64> for $t {
            type Output = $t;

            #[inline]
            fn div(self, rhs: f64) -> $t {
                assert!(!ApproxEq::approx_eq(&rhs, &0.0));
                let inv = 1.0 / rhs;
                $t {
                    $( $s: self.$s * inv ), +
                }
            }
        }
    }
}


macro_rules! scalar_mul_impl {
    ($t: ident, $( $s:ident ), +) => {

        impl Mul<f64> for $t {
            type Output = $t;

            #[inline]
            fn mul(self, rhs: f64) -> $t {
                $t {
                    $( $s: self.$s * rhs ), +
                }
            }
        }
    }
}


macro_rules! scalar_neg_impl {
    ($t: ident, $( $s:ident ), +) => {
        impl Neg for $t {
            type Output = $t;
            #[inline]
            fn neg(self) -> $t {
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
        impl $t {
            #[inline]
            pub fn dot(&self, rhs: &$t) -> f64 {
                add!($( self.$s * rhs.$s ), + )
            }
        }
    }
}

macro_rules! len_impl {
    ($t: ident, $( $s:ident ), +) => {
        impl $t {
            #[inline]
            pub fn length_squared(&self) -> f64 {
                add!($( self.$s * self.$s ), + )
            }
            #[inline]
            pub fn length(&self) -> f64 {
                f64::sqrt(self.length_squared())
            }
        }
    }
}

macro_rules! normalize_impl {
    ($t: ident, $( $s:ident ), +) => {
        impl $t {
            #[inline]
            pub fn normalize(self) -> $t {
                let len = self.length();
                self / len
            }
        }
    }
}

macro_rules! zero_impl {
    ($t: ident, $( $s:ident ), +) => {
        impl Zero for $t {
            #[inline]
            fn zero() -> Self {
                $t {
                    $( $s: 0.0 ), +
                }
            }
        }
    }
}

macro_rules! one_impl {
    ($t: ident, $( $s:ident ), +) => {
        impl One for $t {
            #[inline]
            fn one() -> Self {
                $t {
                    $( $s: 1.0 ), +
                }
            }
        }
    }
}

macro_rules! approx_eq_impl {
    ($t: ident, $( $s: ident ), +) => {
        impl ApproxEq<f64> for $t {
            fn approx_eq(&self, other: &$t) -> bool {
                $(self.$s.approx_eq(&other.$s))&&+
            }

            fn approx_eps() -> f64 {
                f64::approx_eps()
            }
        }
    }
}

macro_rules! arbitrary_impl {
    ($t: ident, $( $s: ident), +) => {
        #[cfg(test)]
        impl Arbitrary for $t {
            #[inline]
            fn arbitrary<G: Gen>(g: &mut G) -> $t {
                $t { $($s: Arbitrary::arbitrary(g),) + }
            }
        }
    }
}

macro_rules! scalar_to_array_impl {
    ($t: ident, $len: expr) => {
        impl $t {
            fn to_array(&self) -> &[f64; $len] {
                unsafe {
                    mem::transmute(self)
                }
            }

            fn to_array_mut(&mut self) -> &mut[f64; $len] {
                unsafe {
                    mem::transmute(self)
                }
            }
        }
    }
}

macro_rules! scalar_index_impl {
    ($t: ident) => (
        impl Index<usize> for $t {
            type Output = f64;

            fn index(&self, i: usize) -> &f64 {
                &self.to_array()[i]
            }
        }

        impl IndexMut<usize> for $t {
            fn index_mut(&mut self, i: usize) -> &mut f64 {
                &mut self.to_array_mut()[i]
            }
        }
    )

}
