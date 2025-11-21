use std::{
    mem::MaybeUninit,
    ops::{Index, IndexMut},
};

/// Work in progress minimal implementation of a high performance vector. Very unsafe, no bounds checks.
#[derive(Debug, Clone, Copy)]
pub struct ArrayVec<T: Copy, const CAPACITY: usize, LenType: Copy> {
    data: [MaybeUninit<T>; CAPACITY],
    len: LenType,
}

macro_rules! arrayvec_impl {
    ($($t:ty),+) => { $(
        impl<T: Copy, const CAPACITY: usize> ArrayVec<T, CAPACITY, $t> {
            #[inline(always)]
            pub const fn new() -> Self {
                Self {
                    data: [MaybeUninit::uninit(); CAPACITY],
                    len: 0,
                }
            }

            #[inline(always)]
            pub fn as_slice(&self) -> &[T] {
                unsafe { std::slice::from_raw_parts(self.data.as_ptr() as *const T, self.len as usize) }
            }

            #[inline(always)]
            pub fn as_mut_slice(&mut self) -> &mut [T] {
                unsafe {
                    std::slice::from_raw_parts_mut(self.data.as_mut_ptr() as *mut T, self.len as usize)
                }
            }

            #[inline(always)]
            pub fn push(&mut self, value: T) {
                let index = self.len as usize;
                unsafe { self.data.get_unchecked_mut(index).write(value) };
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

            #[inline(always)]
            pub const fn len(&self) -> usize {
                self.len as usize
            }

            #[inline(always)]
            pub const fn is_empty(&self) -> bool {
                self.len == 0
            }

            #[inline(always)]
            pub fn iter(&self) -> impl Iterator<Item = &T> {
                self.as_slice().iter()
            }

            #[inline(always)]
            pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
                self.as_mut_slice().iter_mut()
            }
        }

        impl<T: Copy, const CAPACITY: usize> Default for ArrayVec<T, CAPACITY, $t> {
            fn default() -> Self {
                Self::new()
            }
        }

        impl<T: Copy, const CAPACITY: usize> FromIterator<T> for ArrayVec<T, CAPACITY, $t> {
            fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
                let mut vec = Self::new();

                for value in iter {
                    vec.push(value);
                }

                vec
            }
        }

        impl<T: Copy, const CAPACITY: usize> Index<usize> for ArrayVec<T, CAPACITY, $t> {
            type Output = T;

            fn index(&self, index: usize) -> &Self::Output {
                unsafe { self.data[index].assume_init_ref() }
            }
        }

        impl<T: Copy, const CAPACITY: usize> IndexMut<usize> for ArrayVec<T, CAPACITY, $t> {
            fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                unsafe { self.data[index].assume_init_mut() }
            }
        }
    )+ };
}

arrayvec_impl!(u8, u16, u32, u64, usize);
