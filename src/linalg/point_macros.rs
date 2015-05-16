#![macro_use]


macro_rules! pnt_add_vec_impl {
    ($t: ident, $tv: ident,  $($s: ident), +) => (
        impl<N : BaseNum> Add<$tv<N>> for $t<N> where N : Add<Output = N> {
            type Output = $t<N>;

            fn add(self, rhs: $tv<N>) -> $t<N> {
                $t::new($(self.$s + rhs.$s), +)
            }
        }
    )
}

macro_rules! pnt_sub_vec_impl {
    ($t: ident, $tv: ident,  $($s: ident), +) => (
        impl<N : BaseNum> Sub<$tv<N>> for $t<N> where N : Sub<Output = N> {
            type Output = $t<N>;

            fn sub(self, rhs: $tv<N>) -> $t<N> {
                $t::new($(self.$s - rhs.$s), +)
            }
        }
    )
}

macro_rules! pnt_mul_vec_impl {
    ($t: ident, $tv: ident,  $($s: ident), +) => (
        impl<N : BaseNum> Mul<$tv<N>> for $t<N> where N : Mul<Output = N> {
            type Output = $t<N>;

            fn mul(self, rhs: $tv<N>) -> $t<N> {
                $t::new($(self.$s * rhs.$s), +)
            }
        }
    )
}

