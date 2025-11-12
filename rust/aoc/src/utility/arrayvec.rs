use std::mem::MaybeUninit;

/// Work in progress minimal implementation of a high performance vector. Very unsafe, no bounds checks.
#[derive(Clone, Copy)]
pub struct ArrayVec<T: Copy, const N: usize> {
    data: [MaybeUninit<T>; N],
    len: u8,
}

impl<T: Copy, const N: usize> ArrayVec<T, N> {
    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            data: [const { MaybeUninit::uninit() }; N],
            len: 0,
        }
    }

    #[inline(always)]
    pub fn as_slice(&self) -> &[T] {
        unsafe { std::mem::transmute(self.data.get_unchecked(..self.len as usize)) }
    }

    #[inline(always)]
    pub fn push(&mut self, value: T) {
        unsafe { self.data.get_unchecked_mut(self.len as usize).write(value) };
        self.len += 1;
    }

    #[inline(always)]
    pub fn pop(&mut self) -> Option<T> {
        if self.len > 0 {
            self.len -= 1;
            Some(unsafe { self.data.get_unchecked(self.len as usize).assume_init() })
        } else {
            None
        }
    }
}

impl<T: Copy, const N: usize> Default for ArrayVec<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Copy, const N: usize> FromIterator<T> for ArrayVec<T, N> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut vec = Self::new();

        for value in iter {
            vec.push(value);
        }

        vec
    }
}
