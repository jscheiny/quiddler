use num::PrimInt;

#[derive(Debug, Clone, Copy)]
pub struct BitSet<T: PrimInt>(pub T);

impl<T> BitSet<T>
where
    T: PrimInt + std::ops::BitOrAssign<T> + std::ops::ShrAssign<usize>,
{
    pub fn new() -> BitSet<T> {
        BitSet(T::zero())
    }

    pub fn from(value: T) -> BitSet<T> {
        BitSet(value)
    }

    pub fn add(&mut self, value: usize) {
        self.0 |= Self::flag(value)
    }

    pub fn without(&self, value: usize) -> BitSet<T> {
        BitSet(self.0 & !Self::flag(value))
    }

    pub fn contains(&self, value: usize) -> bool {
        self.0 & Self::flag(value) != T::zero()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_zero()
    }

    pub fn len(&self) -> usize {
        self.0.count_ones() as usize
    }

    pub fn decrement(&self) -> BitSet<T> {
        BitSet(self.0.shr(1))
    }

    pub fn intersection(&self, other: BitSet<T>) -> BitSet<T> {
        BitSet(self.0 & other.0)
    }

    pub fn invert(&self, max_value: usize) -> BitSet<T> {
        let max_flag = Self::flag(max_value);
        let full_ones = max_flag - T::one();
        BitSet(full_ones & !self.0)
    }

    pub fn for_each<F: FnMut(usize)>(&self, mut f: F) {
        let mut value = self.0;
        let mut index = 0;
        while !value.is_zero() {
            let trailing = value.trailing_zeros() as usize;
            index += trailing;
            f(index);
            value >>= trailing + 1;
            index += 1;
        }
    }

    fn flag(value: usize) -> T {
        T::one() << value
    }
}
