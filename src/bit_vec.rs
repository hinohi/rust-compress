const BITS: usize = 8;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct BitVec {
    bit: u8,
    data: Vec<u8>,
}

impl BitVec {
    /// Constructs a new, empty `BitVec`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rust_compress::bit_vec::BitVec;
    /// let mut v = BitVec::new();
    /// ```
    pub fn new() -> BitVec {
        Self::with_capacity(0)
    }

    /// Constructs a new, empty `BitVec` with the specified capacity.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rust_compress::bit_vec::BitVec;
    /// let mut v = BitVec::with_capacity(8);
    ///
    /// // The vector contains no items
    /// assert_eq!(v.len(), 0);
    ///
    /// // These are all done without reallocating...
    /// for _ in 0..8 {
    ///     v.push(true);
    /// }
    ///
    /// // ...but this may make the vector reallocate
    /// v.push(true);
    /// ```
    pub fn with_capacity(capacity: usize) -> BitVec {
        let bytes = Self::byte_pos(capacity + BITS - 1);
        BitVec {
            bit: BITS as u8,
            data: Vec::with_capacity(bytes),
        }
    }

    #[inline]
    fn byte_pos(index: usize) -> usize {
        index / BITS
    }

    /// Returns the number of elements the vector can hold without
    /// reallocating.
    ///
    /// Capacity is reserved in multiples of 8.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rust_compress::bit_vec::BitVec;
    /// let v = BitVec::with_capacity(10);
    /// assert!(v.capacity() >= 10);
    /// ```
    pub fn capacity(&self) -> usize {
        self.data.capacity() * BITS
    }

    /// Returns the number of elements the vector hold.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rust_compress::bit_vec::BitVec;
    /// let mut v = BitVec::new();
    /// assert_eq!(v.len(), 0);
    /// v.push(true);
    /// v.push(false);
    /// assert_eq!(v.len(), 2)
    /// ```
    pub fn len(&self) -> usize {
        self.data.len() * BITS + self.bit as usize - BITS
    }

    /// Appends an element to the back of a collection.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rust_compress::bit_vec::BitVec;
    /// let mut v = BitVec::new();
    ///
    /// // Push `1`
    /// v.push(true);
    /// // Push `0`
    /// v.push(false);
    /// ```
    pub fn push(&mut self, value: bool) {
        if self.bit == BITS as u8 {
            self.data.push(0);
            self.bit = 0;
        }
        if value {
            *self.data.last_mut().unwrap() ^= 1 << self.bit;
        }
        self.bit += 1;
    }

    /// Split the vector into two part.
    /// Right part is fragment of bytes.
    /// Left part is other.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rust_compress::bit_vec::BitVec;
    /// let mut v = BitVec::new();
    /// for i in 0..12 {
    ///     v.push(i % 2 == 0);
    /// }
    /// let (rest, last) = v.split_rest();
    /// assert_eq!(rest.into_bytes(), vec![0b01010101]);
    /// assert_eq!(last.into_bytes(), vec![0b00000101]);
    /// ```
    pub fn split_rest(self) -> (BitVec, BitVec) {
        if self.bit == BITS as u8 {
            (self, BitVec::new())
        } else {
            let mut rest = self;
            let mut last = BitVec::with_capacity(8);
            let b = rest.data.pop().unwrap();
            for i in 0..rest.bit {
                last.push((b >> i) & 1 == 1);
            }
            rest.bit = BITS as u8;
            (rest, last)
        }
    }

    /// Convert into bytes `Vec<u8>`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rust_compress::bit_vec::BitVec;
    /// let mut v = BitVec::new();
    /// v.push(true);
    /// v.push(false);
    /// v.push(true);
    /// assert_eq!(v.into_bytes(), vec![0b00000101]);
    /// ```
    pub fn into_bytes(self) -> Vec<u8> {
        self.data
    }

    pub fn iter(&self) -> Iter {
        Iter {
            pos: 0,
            last_bit: self.bit,
            bit: 0,
            data: &self.data,
        }
    }
}

impl From<Vec<bool>> for BitVec {
    fn from(bits: Vec<bool>) -> BitVec {
        let mut v = BitVec::with_capacity(bits.len());
        for bit in bits {
            v.push(bit);
        }
        v
    }
}

impl From<&[bool]> for BitVec {
    fn from(bits: &[bool]) -> BitVec {
        let mut v = BitVec::with_capacity(bits.len());
        for bit in bits {
            v.push(*bit);
        }
        v
    }
}

pub struct Iter<'a> {
    pos: usize,
    last_bit: u8,
    bit: u8,
    data: &'a [u8],
}

impl<'a> Iterator for Iter<'a> {
    type Item = bool;
    fn next(&mut self) -> Option<Self::Item> {
        if self.bit == BITS as u8 || self.pos + 1 == self.data.len() && self.bit == self.last_bit {
            self.bit = 0;
            self.pos += 1;
        }
        if self.pos == self.data.len() {
            return None;
        }
        let b = (self.data[self.pos] >> self.bit) & 1;
        self.bit += 1;
        Some(b == 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn len() {
        let mut v = BitVec::new();
        assert_eq!(v.len(), 0);
        v.push(false);
        assert_eq!(v.len(), 1);
        for _ in 0..6 {
            v.push(false);
        }
        assert_eq!(v.len(), 7);
        v.push(false);
        assert_eq!(v.len(), 8);
        v.push(false);
        assert_eq!(v.len(), 9);
    }

    #[test]
    fn split_rest() {
        let v = BitVec::new();
        let (rest, last) = v.split_rest();
        assert_eq!(rest, BitVec::new());
        assert_eq!(last, BitVec::new());

        let v: BitVec = vec![true; 8].into();
        let (rest, last) = v.split_rest();
        assert_eq!(rest, vec![true; 8].into());
        assert_eq!(last, BitVec::new());

        let v: BitVec = vec![true; 9].into();
        let (rest, last) = v.split_rest();
        assert_eq!(rest, vec![true; 8].into());
        assert_eq!(last, vec![true].into());
    }

    #[test]
    fn iter() {
        use std::iter::FromIterator;
        let b: BitVec = vec![true; 7].into();
        assert_eq!(vec![true; 7], Vec::from_iter(b.iter()));
        let b: BitVec = vec![true; 8].into();
        assert_eq!(vec![true; 8], Vec::from_iter(b.iter()));
        let b: BitVec = vec![true; 9].into();
        assert_eq!(vec![true; 9], Vec::from_iter(b.iter()));
        let b: BitVec = vec![true, false, false, true].into();
        assert_eq!(vec![true, false, false, true], Vec::from_iter(b.iter()));
    }
}
