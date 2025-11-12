use std::{marker::PhantomData, mem::MaybeUninit, ops::Index};

/// Work in progress minimal implementation of a high performance key-value collection. Very unsafe, no bounds checks.
pub struct KeyMap<K, V, const CAPACITY: usize>
where
    [(); CAPACITY.div_ceil(usize::BITS as usize)]: Sized,
{
    data: Box<[MaybeUninit<V>; CAPACITY]>,
    bitset: [usize; CAPACITY.div_ceil(usize::BITS as usize)],
    marker: PhantomData<K>,
}

macro_rules! keymap_impl {
    ($($t:ty),+) => { $(
        impl<V: Copy, const CAPACITY: usize> KeyMap<$t, V, CAPACITY>
        where
            [(); CAPACITY.div_ceil(usize::BITS as usize)]: Sized,
        {
            #[inline(always)]
            pub fn new() -> Self {
                Self {
                    data: Box::new([MaybeUninit::uninit(); _]),
                    bitset: [0; _],
                    marker: PhantomData,
                }
            }

            #[inline(always)]
            const fn select_bit(key: $t) -> (usize, $t) {
                let chunk = key as usize >> (usize::BITS.trailing_zeros() as usize);
                let bit = key & (usize::BITS as $t - 1);
                (chunk, bit)
            }

            #[inline(always)]
            pub fn insert(&mut self, key: $t, value: V) {
                unsafe { self.data.get_unchecked_mut(key as usize).write(value) };
                let (chunk, bit) = Self::select_bit(key);
                unsafe { *self.bitset.get_unchecked_mut(chunk) |= 1 << bit };
            }

            #[inline(always)]
            pub fn get(&self, key: $t) -> Option<&V> {
                if self.contains_key(key) {
                    Some(unsafe { self.data.get_unchecked(key as usize).assume_init_ref() })
                } else {
                    None
                }
            }

            #[inline(always)]
            pub fn contains_key(&self, key: $t) -> bool {
                let (chunk, bit) = Self::select_bit(key);
                unsafe { (self.bitset.get_unchecked(chunk) >> bit) & 1 == 1 }
            }
        }

        impl<V: Copy, const CAPACITY: usize> Default for KeyMap<$t, V, CAPACITY>
        where
            [(); CAPACITY.div_ceil(usize::BITS as usize)]: Sized,
        {
            fn default() -> Self {
                Self::new()
            }
        }

        impl<V: Copy, const CAPACITY: usize> Index<usize> for KeyMap<$t, V, CAPACITY>
        where
            [(); CAPACITY.div_ceil(usize::BITS as usize)]: Sized,
        {
            type Output = V;

            fn index(&self, index: usize) -> &Self::Output {
                unsafe { self.data[index].assume_init_ref() }
            }
        }
    )+ };
}

keymap_impl!(u8, u16, u32, u64, usize);
