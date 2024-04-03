use crate::CircularArray;



pub struct CircularArrayIter<'a, const N: usize, T: 'a> {
    circular_array: &'a CircularArray<N, T>,
    index: usize,
}

impl <'a, const N: usize, T: 'a> CircularArrayIter<'a, N, T> {
    pub fn new(circular_array: &'a CircularArray<N, T>) -> Self {
        CircularArrayIter {
            circular_array,
            index: 0,
        }
    }
}
impl<'a, const N: usize, T> Iterator for CircularArrayIter<'a, N, T> where T: Default + Copy {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let r = if self.index < self.circular_array.seq {
            Some(&self.circular_array[self.index])
        } else {
            None
        };
        self.index += 1;
        r
    }
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_circular_array_iter() {
        let mut arr = CircularArray::<3, u32>::new();
        arr.push(1);
        arr.push(2);
        arr.push(3);
        let mut iter: CircularArrayIter<3, u32> = arr.iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);

        let mut r = 0;
        for &item in arr.iter() {
            r += item;
        }
        assert_eq!(r, 6);

        let r: u32 = arr.iter().sum();
        assert_eq!(r, 6);
    }
}