#![macro_use]


macro_rules! pnt_add_vec_impl {
    ($t: ident, $tv: ident,  $($s: ident), +) => (
        impl Add<$tv> for $t {
            type Output = $t;

            fn add(self, rhs: $tv) -> $t {
                $t::new($(self.$s + rhs.$s), +)
            }
        }
    )
}

macro_rules! pnt_sub_pnt_impl {
    ($t: ident, $tv: ident,  $($s: ident), +) => (
        impl Sub<$t> for $t {
            type Output = $tv;

            fn sub(self, rhs: $t) -> $tv {
                $tv::new($(self.$s - rhs.$s), +)
            }
        }
    )
}

macro_rules! pnt_sub_vec_impl {
    ($t: ident, $tv: ident,  $($s: ident), +) => (
        impl Sub<$tv> for $t {
            type Output = $t;

            fn sub(self, rhs: $tv) -> $t {
                $t::new($(self.$s - rhs.$s), +)
            }
        }
    )
}

macro_rules! pnt_mul_vec_impl {
    ($t: ident, $tv: ident,  $($s: ident), +) => (
        impl Mul<$tv> for $t {
            type Output = $t;

            fn mul(self, rhs: $tv) -> $t {
                $t::new($(self.$s * rhs.$s), +)
            }
        }
    )
}

macro_rules! pnt_distance_impl {
    ($t: ident) => (
        pub fn distance(p1: &$t, p2: &$t) -> f64 {
            (*p1 - *p2).length()
        }

        pub fn distance_squared(p1: &$t, p2: &$t) -> f64 {
            (*p1 - *p2).length_squared()
        }
    )
}
