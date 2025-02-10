use std::ops::Index;

pub trait Length {
    fn length(&self) -> usize;
}

impl<T, const N: usize> Length for [T; N] {
    fn length(&self) -> usize {
        N
    }
}

pub struct CombIter<'a, T: Index<usize> + Sized, const N: usize> {
    values: &'a T,
    indices: [usize; N],
    done: bool,
}

pub trait IterComb: Index<usize> + Length + Sized {
    fn iter_comb<const N: usize>(&self) -> CombIter<'_, Self, N> {
        CombIter {
            values: self,
            indices: core::array::from_fn(|i| i),
            done: false,
        }
    }
}

impl<T: Index<usize> + Length + Sized> IterComb for T {}

impl<T, const N: usize> CombIter<'_, T, N>
where
    T: Index<usize> + Length + Sized,
    <T as Index<usize>>::Output: Sized,
{
    fn increment_indices(&mut self) -> bool {
        if const { N == 0 } {
            true
        } else if const { N == 1 } {
            if self.indices[0] + 1 == self.values.length() {
                true
            } else {
                self.indices[0] += 1;
                false
            }
        } else {
            let mut i = const { N - 1 };
            while self.indices[i] == i + self.values.length() - N {
                if i > 0 {
                    i -= 1;
                } else {
                    return true;
                }
            }
            self.indices[i] += 1;
            for j in i + 1..N {
                self.indices[j] = self.indices[j - 1] + 1;
            }
            false
        }
    }
}

impl<'a, T, const N: usize> Iterator for CombIter<'a, T, N>
where
    T: Index<usize> + Length + Sized,
    <T as Index<usize>>::Output: Sized,
{
    type Item = [&'a <T as Index<usize>>::Output; N];

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            None
        } else {
            let result = self.indices.map(|i| &self.values[i]);
            self.done = self.increment_indices();
            Some(result)
        }
    }
}
