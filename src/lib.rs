use std::fmt::Debug;
use std::ops::{Add, Index, IndexMut};

/// A circular array that allows infinite pushes into a fixed-size array.
#[derive(Debug)]
pub struct CircularArray<const N: usize, T> {
    arr: [T;N],
    start: usize,
    seq: usize,
}

impl<const N: usize, T> CircularArray<N, T> where T: Copy + Default + Debug {
    pub fn new() -> Self {
        Self {
            arr: [T::default(); N],
            start: 0,
            seq: 0,
        }
    }

    /// # example
    /// ```
    ///     #[test]
    ///     #[allow(non_snake_case)]
    ///     fn test_Index_and_IndexMut() {
    ///             let mut arr = CircularArray::<3, u32>::new();
    ///             arr.push(0);
    ///             arr.push(0);
    ///             arr.push(0);
    ///             arr.push(0);
    ///             arr.push(0);
    ///             arr[0] = 1;
    ///             arr[1] = 2;
    ///             arr[2] = 3;
    ///             assert_eq!(arr[0], 1);
    ///             assert_eq!(arr[1], 2);
    ///             assert_eq!(arr[2], 3);
    ///         }

    pub fn push(&mut self, item: T) {
        if self.seq >= N {
            self.arr[self.start] = item;
        } else {
            self.arr[self.seq] = item;
        }
        self.start = (self.start + 1) % N;
        self.seq += 1;
    }

    /// ## Examples
    /// ```
    ///     #[test]
    ///     fn test_to_array() {
    ///         let mut arr = CircularArray::<3, u32>::new();
    ///         arr.push(1);
    ///         arr.push(2);
    ///         arr.push(3);
    ///         assert_eq!(arr.to_array(), [1, 2, 3]);
    ///         arr.push(4);
    ///         assert_eq!(arr.to_array(), [2, 3, 4]);
    ///     }
    /// ```
    pub fn to_array(&self) -> [T;N] {
        unsafe {
            let mut arr = [T::default(); N];

            let src_ptr = self.arr.as_ptr();
            let dest_ptr = arr.as_mut_ptr();

            if self.seq >= N && self.start > 0 {
                std::ptr::copy_nonoverlapping(src_ptr.add(self.start), dest_ptr, N - self.start);
                std::ptr::copy_nonoverlapping(src_ptr, dest_ptr.add(N - self.start), N - self.start);
            } else {
                std::ptr::copy_nonoverlapping(src_ptr, dest_ptr, N);
            }
            arr
        }
    }
}


impl<T, const N: usize> Index<usize> for CircularArray<N, T> where [T]: Index<usize>, T: Default + Copy
{
    type Output = <[T] as Index<usize>>::Output;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        if self.seq >= N {
            &self.arr[(self.start + index) % N]
        } else {
            &self.arr[index]
        }
    }
}

impl<T, const N: usize> IndexMut<usize> for CircularArray<N, T>
    where [T]: Index<usize>,
          T: Default + Copy, usize: Add<usize> {

    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if self.seq >= N {
            &mut self.arr[(self.start + index) % N]
        } else {
            &mut self.arr[index]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push() {
        let mut arr = CircularArray::<3, u32>::new();
        arr.push(1);
        arr.push(2);
        arr.push(3);
        assert_eq!(arr.arr, [1, 2, 3]);
        arr.push(3);
        assert_eq!(arr.arr, [2, 3, 4]);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_Index_and_IndexMut() {
        let mut arr = CircularArray::<3, u32>::new();
        arr.push(0);
        arr.push(0);
        arr.push(0);
        arr.push(0);
        arr.push(0);
        arr[0] = 1;
        arr[1] = 2;
        arr[2] = 3;
        assert_eq!(arr[0], 1);
        assert_eq!(arr[1], 2);
        assert_eq!(arr[2], 3);
    }

    #[test]
    fn test_to_array() {
        let mut arr = CircularArray::<3, u32>::new();
        arr.push(1);
        arr.push(2);
        arr.push(3);
        assert_eq!(arr.to_array(), [1, 2, 3]);
        arr.push(4);
        assert_eq!(arr.to_array(), [2, 3, 4]);
    }
}

