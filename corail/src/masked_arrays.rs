use std::ops::Index;


pub struct MaskedArray<'a, T> {
    pub data: &'a Vec<T>,
    mask: Vec<usize>,
}

impl<'a, T> MaskedArray<'a, T> {
    pub fn new(data: &Vec<T>, mask: Vec<usize>) -> MaskedArray<T> {
        MaskedArray {
            data,
            mask
        }
    }

    pub fn len(&self) -> usize {
        self.mask.len()
    }

    pub fn iter(&self) -> MaskedArrayIterator<'a, T> {
        MaskedArrayIterator::new(self)
    }
}


impl<'a, T> Index<usize> for MaskedArray<'a, T> {
    type Output = T;

    fn index(&self, ind: usize) -> &Self::Output {
        &self.data[self.mask[ind]]
    }
}


pub struct MaskedArrayIterator<'a, T> {
    current: usize,
    data: &'a MaskedArray<'a, T>,
}

impl<'a, T> MaskedArrayIterator<'a, T> {
    pub fn new(data: &MaskedArray<'a, T>) -> MaskedArrayIterator<'a, T> {
        let current: usize = 0;
        MaskedArrayIterator {
            current,
            data,
        }
    }
}

impl<'a, T> Iterator for MaskedArrayIterator<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.data.len() {
            self.current += 1;
            Some(self.data[self.current-1])
        } else {
            None
        }
    }
}