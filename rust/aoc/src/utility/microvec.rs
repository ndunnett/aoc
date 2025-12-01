use std::{
    mem::MaybeUninit,
    ops::{Index, IndexMut},
};

pub const trait Usizeable: Copy {
    fn as_usize(self) -> usize;
    fn increment(&mut self);
    fn decrement(&mut self);
    fn zero() -> Self;
    fn is_zero(&self) -> bool;
}

macro_rules! usizeable_impl {
    ($($t:ty),+) => { $(
        impl const Usizeable for $t {
            #[inline(always)]
            fn as_usize(self) -> usize { self as usize }
            #[inline(always)]
            fn increment(&mut self) { *self += 1; }
            #[inline(always)]
            fn decrement(&mut self) { *self -= 1; }
            #[inline(always)]
            fn zero() -> Self { 0 }
            #[inline(always)]
            fn is_zero(&self) -> bool { *self == 0 }
        }
    )+ };
}

usizeable_impl!(u8, u16, u32, u64, usize);

/// Work in progress minimal implementation of a high performance vector.
#[derive(Clone, Copy)]
pub struct MicroVec<T: Copy, const CAPACITY: usize, LenType: const Usizeable> {
    data: [MaybeUninit<T>; CAPACITY],
    len: LenType,
}

impl<T: Copy, const CAPACITY: usize, LenType: const Usizeable> MicroVec<T, CAPACITY, LenType> {
    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            data: [MaybeUninit::uninit(); CAPACITY],
            len: LenType::zero(),
        }
    }

    #[inline(always)]
    pub const fn as_ptr(&self) -> *const T {
        self.data.as_ptr() as *const T
    }

    #[inline(always)]
    pub const fn as_mut_ptr(&mut self) -> *mut T {
        self.data.as_mut_ptr() as *mut T
    }

    #[inline(always)]
    pub const fn as_slice(&self) -> &[T] {
        unsafe { std::slice::from_raw_parts(self.as_ptr(), self.len()) }
    }

    #[inline(always)]
    pub const fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe { std::slice::from_raw_parts_mut(self.as_mut_ptr(), self.len()) }
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
    pub const fn len(&self) -> usize {
        self.len.as_usize()
    }

    #[inline(always)]
    pub const fn capacity(&self) -> usize {
        CAPACITY
    }

    #[inline(always)]
    pub const fn is_empty(&self) -> bool {
        self.len.is_zero()
    }

    #[inline(always)]
    pub const fn is_full(&self) -> bool {
        self.len() == self.capacity()
    }

    #[inline(always)]
    pub const fn push(&mut self, value: T) {
        if self.is_full() {
            panic!("exceeded capacity")
        } else {
            unsafe {
                self.push_unchecked(value);
            }
        }
    }

    #[inline(always)]
    pub const fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            Some(unsafe { self.pop_unchecked() })
        }
    }

    #[inline(always)]
    pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.len() {
            Some(unsafe { self.get_unchecked(index) })
        } else {
            None
        }
    }

    #[inline(always)]
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index < self.len() {
            Some(unsafe { self.get_mut_unchecked(index) })
        } else {
            None
        }
    }

    /// # Safety
    /// no bounds check, can exceed capacity
    #[inline(always)]
    pub const unsafe fn ptr_to(&self, index: usize) -> *const T {
        unsafe { self.as_ptr().add(index) }
    }

    /// # Safety
    /// no bounds check, can exceed capacity
    #[inline(always)]
    pub const unsafe fn mut_ptr_to(&mut self, index: usize) -> *mut T {
        unsafe { self.as_mut_ptr().add(index) }
    }

    /// # Safety
    /// no bounds check, can exceed capacity
    #[inline(always)]
    pub const unsafe fn push_unchecked(&mut self, value: T) {
        unsafe {
            self.mut_ptr_to(self.len()).write(value);
        }

        self.len.increment();
    }

    /// # Safety
    /// no bounds check, can pop an empty vec
    #[inline(always)]
    pub const unsafe fn pop_unchecked(&mut self) -> T {
        self.len.decrement();
        unsafe { self.mut_ptr_to(self.len()).read() }
    }

    /// # Safety
    /// no bounds check, can overflow buffer
    #[inline(always)]
    pub const unsafe fn get_unchecked(&self, index: usize) -> &T {
        unsafe { self.ptr_to(index).as_ref_unchecked() }
    }

    /// # Safety
    /// no bounds check, can overflow buffer
    #[inline(always)]
    pub const unsafe fn get_mut_unchecked(&mut self, index: usize) -> &mut T {
        unsafe { self.mut_ptr_to(index).as_mut_unchecked() }
    }
}

impl<T: Copy + std::fmt::Debug, const CAPACITY: usize, LenType: const Usizeable> std::fmt::Debug
    for MicroVec<T, CAPACITY, LenType>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl<T: Copy, const CAPACITY: usize, LenType: const Usizeable> Default
    for MicroVec<T, CAPACITY, LenType>
{
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Copy, const CAPACITY: usize, LenType: const Usizeable> FromIterator<T>
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

impl<T: Copy, const CAPACITY: usize, LenType: const Usizeable> Index<usize>
    for MicroVec<T, CAPACITY, LenType>
{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.get(index).expect("index out of bounds")
    }
}

impl<T: Copy, const CAPACITY: usize, LenType: const Usizeable> IndexMut<usize>
    for MicroVec<T, CAPACITY, LenType>
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.get_mut(index).expect("index out of bounds")
    }
}

impl<T: Copy, const CAPACITY: usize, LenType: const Usizeable> Index<std::ops::Range<usize>>
    for MicroVec<T, CAPACITY, LenType>
{
    type Output = [T];

    fn index(&self, index: std::ops::Range<usize>) -> &Self::Output {
        if index.end < self.len() && index.start <= index.end {
            unsafe { std::slice::from_raw_parts(self.ptr_to(index.start), index.end - index.start) }
        } else {
            panic!("index out of bounds");
        }
    }
}

impl<T: Copy, const CAPACITY: usize, LenType: const Usizeable> IndexMut<std::ops::Range<usize>>
    for MicroVec<T, CAPACITY, LenType>
{
    fn index_mut(&mut self, index: std::ops::Range<usize>) -> &mut Self::Output {
        if index.end < self.len() && index.start <= index.end {
            unsafe {
                std::slice::from_raw_parts_mut(
                    self.mut_ptr_to(index.start),
                    index.end - index.start,
                )
            }
        } else {
            panic!("index out of bounds");
        }
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
