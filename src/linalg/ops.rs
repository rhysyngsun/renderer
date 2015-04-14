

trait Cross {
    type Output;

    fn cross(&self, other: &Self) -> Self::Output;
}
