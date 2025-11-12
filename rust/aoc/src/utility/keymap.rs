use std::mem::MaybeUninit;

/// Work in progress minimal implementation of a high performance key-value collection. Very unsafe, no bounds checks.
pub struct KeyMap<T, const N: usize, const B: usize = { N.div_ceil(usize::BITS as usize) }> {
    data: Box<[MaybeUninit<T>; N]>,
    bitset: [usize; B],
}

impl<T: Copy, const N: usize, const B: usize> KeyMap<T, N, B> {
    #[inline(always)]
    pub fn new() -> Self {
        Self {
            data: Box::new([const { MaybeUninit::uninit() }; _]),
            bitset: [0; _],
        }
    }

    #[inline(always)]
    const fn select_bit(key: u16) -> (usize, u16) {
        let chunk = key as usize >> (usize::BITS.trailing_zeros() as usize);
        let bit = key & (usize::BITS as u16 - 1);
        (chunk, bit)
    }

    #[inline(always)]
    pub fn insert(&mut self, key: u16, value: T) {
        unsafe { self.data.get_unchecked_mut(key as usize).write(value) };
        let (chunk, bit) = Self::select_bit(key);
        unsafe { *self.bitset.get_unchecked_mut(chunk) |= 1 << bit };
    }

    #[inline(always)]
    pub fn get(&self, key: u16) -> Option<&T> {
        if self.contains_key(key) {
            Some(unsafe { self.data.get_unchecked(key as usize).assume_init_ref() })
        } else {
            None
        }
    }

    #[inline(always)]
    pub fn contains_key(&self, key: u16) -> bool {
        let (chunk, bit) = Self::select_bit(key);
        unsafe { (self.bitset.get_unchecked(chunk) >> bit) & 1 == 1 }
    }
}

impl<T: Copy, const N: usize, const B: usize> Default for KeyMap<T, N, B> {
    fn default() -> Self {
        Self::new()
    }
}
