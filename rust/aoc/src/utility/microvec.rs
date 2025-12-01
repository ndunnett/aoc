use std::{
    mem::MaybeUninit,
    ops::{Index, IndexMut},
};

pub trait Usizeable: Copy + Default {
    fn as_usize(self) -> usize;
    fn increment(&mut self);
    fn decrement(&mut self);
}

macro_rules! usizeable_impl {
    ($($t:ty),+) => { $(
        impl Usizeable for $t {
            #[inline(always)]
            fn as_usize(self) -> usize { self as usize }
            #[inline(always)]
            fn increment(&mut self) { *self += 1; }
            #[inline(always)]
            fn decrement(&mut self) { *self -= 1; }
        }
    )+ };
}

usizeable_impl!(u8, u16, u32, u64, usize);

/// Work in progress minimal implementation of a high performance vector.
#[derive(Clone, Copy)]
pub struct MicroVec<T: Copy, const CAPACITY: usize, LenType: Usizeable> {
    data: [MaybeUninit<T>; CAPACITY],
    len: LenType,
}

impl<T: Copy, const CAPACITY: usize, LenType: Usizeable> MicroVec<T, CAPACITY, LenType> {
    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            data: [MaybeUninit::uninit(); CAPACITY],
            len: unsafe { MaybeUninit::zeroed().assume_init() },
        }
    }

    #[inline(always)]
    pub fn as_slice(&self) -> &[T] {
        unsafe { std::slice::from_raw_parts(self.data.as_ptr() as *const T, self.len.as_usize()) }
    }

    #[inline(always)]
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe {
            std::slice::from_raw_parts_mut(self.data.as_mut_ptr() as *mut T, self.len.as_usize())
        }
    }

    #[inline(always)]
    pub fn push(&mut self, value: T) {
        let index = self.len.as_usize();
        unsafe { self.data.get_unchecked_mut(index).write(value) };
        self.len.increment();
    }

    #[inline(always)]
    pub fn pop(&mut self) -> Option<T> {
        if !self.is_empty() {
            self.len.decrement();
            Some(unsafe { self.data.get_unchecked(self.len.as_usize()).assume_init() })
        } else {
            None
        }
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.len.as_usize()
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline(always)]
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.as_slice().iter()
    }

    #[inline(always)]
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.as_mut_slice().iter_mut()
    }

    #[inline(always)]
    pub fn get(&mut self, index: usize) -> Option<&T> {
        if index < self.len.as_usize() {
            Some(&self[index])
        } else {
            None
        }
    }

    #[inline(always)]
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index < self.len.as_usize() {
            Some(&mut self[index])
        } else {
            None
        }
    }
}

impl<T: Copy + std::fmt::Debug, const CAPACITY: usize, LenType: Usizeable> std::fmt::Debug
    for MicroVec<T, CAPACITY, LenType>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl<T: Copy, const CAPACITY: usize, LenType: Usizeable> Default
    for MicroVec<T, CAPACITY, LenType>
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Copy, const CAPACITY: usize, LenType: Usizeable> FromIterator<T>
    for MicroVec<T, CAPACITY, LenType>
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut vec = Self::new();

        for value in iter {
            vec.push(value);
        }

        vec
    }
}

impl<T: Copy, const CAPACITY: usize, LenType: Usizeable> Index<usize>
    for MicroVec<T, CAPACITY, LenType>
{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        unsafe { self.data[index].assume_init_ref() }
    }
}

impl<T: Copy, const CAPACITY: usize, LenType: Usizeable> IndexMut<usize>
    for MicroVec<T, CAPACITY, LenType>
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { self.data[index].assume_init_mut() }
    }
}

impl<T: Copy, const CAPACITY: usize, LenType: Usizeable> Index<std::ops::Range<usize>>
    for MicroVec<T, CAPACITY, LenType>
{
    type Output = [T];

    fn index(&self, index: std::ops::Range<usize>) -> &Self::Output {
        &self.as_slice()[index]
    }
}

#[macro_export]
macro_rules! microvec {
    ( $t:expr => $( $x:expr ),* ) => {
        {
            let mut vec = $t;
            $(vec.push($x);)*
            vec
        }
    };
    ( $t:expr => $x:expr; $count:expr ) => {
        {
            let mut vec = $t;

            for _ in 0..$count {
                vec.push($x);
            }

            vec
        }
    };
}
