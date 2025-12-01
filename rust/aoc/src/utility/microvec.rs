use std::{
    marker::PhantomData,
    mem::MaybeUninit,
    ops::{Index, IndexMut},
    slice::SliceIndex,
};

use crate::utility::{Guard, True};

pub const trait Usizeable: Copy {
    fn from_usize(value: usize) -> Self;
    fn as_usize(self) -> usize;
    fn increment(&mut self);
    fn decrement(&mut self);
    fn zero() -> Self;
    fn is_zero(&self) -> bool;
    fn max_value() -> usize;
}

macro_rules! usizeable_impl {
    ($($t:ty),+) => { $(
        impl const Usizeable for $t {
            #[inline(always)]
            fn from_usize(value: usize) -> Self { value as Self }
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
            #[inline(always)]
            fn max_value() -> usize { Self::MAX as usize }
        }

    )+ };
}

usizeable_impl!(u8, u16, u32, u64, usize);

/// Ensures that `LenType` is large enough for a given `CAPACITY` at compile time.
pub struct LenTypeBigEnough<const CAPACITY: usize, LenType: const Usizeable> {
    marker: PhantomData<LenType>,
}

impl<const CAPACITY: usize, LenType: const Usizeable> True for LenTypeBigEnough<CAPACITY, LenType> where
    Guard<{ LenType::max_value() >= CAPACITY }>: True
{
}

/// Work in progress minimal implementation of a high performance, heapless, const-friendly vector.
///
/// Should not be used if you rely on elements being dropped, due to `MicroVec<T, ...>` implementing `Copy` where `T: Copy`,
/// and specialised `Drop` implementations being forbidden, `Drop` will never be called for `T`.
pub struct MicroVec<T, const CAPACITY: usize, LenType: const Usizeable>
where
    LenTypeBigEnough<CAPACITY, LenType>: True,
{
    data: [MaybeUninit<T>; CAPACITY],
    len: LenType,
}

impl<T, const CAPACITY: usize, LenType: const Usizeable> MicroVec<T, CAPACITY, LenType>
where
    LenTypeBigEnough<CAPACITY, LenType>: True,
{
    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            data: [const { MaybeUninit::uninit() }; CAPACITY],
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
    pub const fn clear(&mut self) {
        self.len = LenType::zero();
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
    pub const fn insert(&mut self, index: usize, value: T) {
        if self.is_full() || index > self.len() {
            panic!("index out of bounds")
        } else {
            unsafe {
                self.insert_unchecked(index, value);
            }
        }
    }

    #[inline(always)]
    pub const fn pop(&mut self) -> Option<T> {
        if !self.is_empty() {
            Some(unsafe { self.pop_unchecked() })
        } else {
            None
        }
    }

    #[inline(always)]
    pub const fn remove(&mut self, index: usize) -> T {
        self.try_remove(index).expect("no element to remove")
    }

    #[inline(always)]
    pub const fn try_remove(&mut self, index: usize) -> Option<T> {
        if !self.is_empty() && index < self.len() {
            Some(unsafe { self.remove_unchecked(index) })
        } else {
            None
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
        unsafe { self.paste(self.len(), value) };
        self.len.increment();
    }

    /// # Safety
    /// no bounds check, can exceed capacity
    #[inline(always)]
    pub const unsafe fn insert_unchecked(&mut self, index: usize, value: T) {
        unsafe {
            std::ptr::copy(
                self.ptr_to(index),
                self.mut_ptr_to(index).add(1),
                self.len() - index,
            );

            self.paste(index, value);
            self.len.increment();
        }
    }

    /// # Safety
    /// no bounds check, can pop an empty vec
    #[inline(always)]
    pub const unsafe fn pop_unchecked(&mut self) -> T {
        self.len.decrement();
        unsafe { self.yoink(self.len()) }
    }

    /// # Safety
    /// no bounds check, can remove from an empty vec
    #[inline(always)]
    pub const unsafe fn remove_unchecked(&mut self, index: usize) -> T {
        let item = unsafe { self.yoink(index) };

        unsafe {
            let src = self.ptr_to(index).add(1);
            let len = self.len() - index;
            let dst = self.mut_ptr_to(index);
            std::ptr::copy(src, dst, len);
            self.len.decrement();
        }

        item
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

    /// # Safety
    /// no bounds check, doesn't rearrange elements, doesn't manage length
    #[inline(always)]
    const unsafe fn yoink(&mut self, index: usize) -> T {
        unsafe { self.ptr_to(index).read() }
    }

    /// # Safety
    /// no bounds check, doesn't rearrange elements, doesn't manage length, will blindly overwrite data
    #[inline(always)]
    const unsafe fn paste(&mut self, index: usize, value: T) {
        unsafe { self.mut_ptr_to(index).write(value) }
    }
}

/// Ensures that the given `MicroVec` type is large enough for a given array at compile time.
pub struct VecBigEnough<const VEC_SIZE: usize, const ARRAY_SIZE: usize>;

impl<const VEC_SIZE: usize, const ARRAY_SIZE: usize> True for VecBigEnough<VEC_SIZE, ARRAY_SIZE> where
    Guard<{ VEC_SIZE >= ARRAY_SIZE }>: True
{
}

impl<T: Copy, const CAPACITY: usize, LenType: const Usizeable, const N: usize> From<[T; N]>
    for MicroVec<T, CAPACITY, LenType>
where
    VecBigEnough<CAPACITY, N>: True,
    LenTypeBigEnough<CAPACITY, LenType>: True,
{
    fn from(value: [T; N]) -> Self {
        let mut vec = Self::new();

        unsafe {
            std::ptr::copy_nonoverlapping(value.as_ptr(), vec.as_mut_ptr(), N);
        }

        vec.len = LenType::from_usize(N);
        vec
    }
}

impl<T, const CAPACITY: usize, LenType: const Usizeable> Default for MicroVec<T, CAPACITY, LenType>
where
    LenTypeBigEnough<CAPACITY, LenType>: True,
{
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

impl<T, const CAPACITY: usize, LenType: const Usizeable> FromIterator<T>
    for MicroVec<T, CAPACITY, LenType>
where
    LenTypeBigEnough<CAPACITY, LenType>: True,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut vec = Self::new();

        for value in iter {
            vec.push(value);
        }

        vec
    }
}

impl<T: Clone, const CAPACITY: usize, LenType: const Usizeable> Clone
    for MicroVec<T, CAPACITY, LenType>
where
    LenTypeBigEnough<CAPACITY, LenType>: True,
{
    fn clone(&self) -> Self {
        let mut cloned = Self::new();

        unsafe {
            if std::mem::needs_drop::<T>() {
                for i in 0..self.len() {
                    cloned.mut_ptr_to(i).write(self.get_unchecked(i).clone());
                }
            } else {
                std::ptr::copy_nonoverlapping(self.as_ptr(), cloned.as_mut_ptr(), self.len());
            }
        }

        cloned.len = self.len;
        cloned
    }
}

impl<T: Copy, const CAPACITY: usize, LenType: const Usizeable> Copy
    for MicroVec<T, CAPACITY, LenType>
where
    LenTypeBigEnough<CAPACITY, LenType>: True,
{
}

pub struct IntoIter<T, const CAPACITY: usize, LenType: const Usizeable>
where
    LenTypeBigEnough<CAPACITY, LenType>: True,
{
    vec: MicroVec<T, CAPACITY, LenType>,
    index: usize,
}

impl<T, const CAPACITY: usize, LenType: const Usizeable> IntoIterator
    for MicroVec<T, CAPACITY, LenType>
where
    LenTypeBigEnough<CAPACITY, LenType>: True,
{
    type Item = T;
    type IntoIter = IntoIter<T, CAPACITY, LenType>;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            vec: self,
            index: 0,
        }
    }
}

impl<T, const CAPACITY: usize, LenType: const Usizeable> Iterator for IntoIter<T, CAPACITY, LenType>
where
    LenTypeBigEnough<CAPACITY, LenType>: True,
{
    type Item = T;

    #[inline(always)]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let exact = self.vec.len();
        (exact, Some(exact))
    }

    #[inline(always)]
    fn count(self) -> usize
    where
        Self: Sized,
    {
        self.vec.len()
    }

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.vec.len() {
            let item = unsafe { self.vec.yoink(self.index) };
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }
}

impl<T, const CAPACITY: usize, LenType: const Usizeable> ExactSizeIterator
    for IntoIter<T, CAPACITY, LenType>
where
    LenTypeBigEnough<CAPACITY, LenType>: True,
{
    fn len(&self) -> usize {
        self.vec.len() - self.index
    }
}

impl<T, const CAPACITY: usize, LenType: const Usizeable> std::iter::FusedIterator
    for IntoIter<T, CAPACITY, LenType>
where
    LenTypeBigEnough<CAPACITY, LenType>: True,
{
}

// `Deref` into slices

impl<T, const CAPACITY: usize, LenType: const Usizeable> std::ops::Deref
    for MicroVec<T, CAPACITY, LenType>
where
    LenTypeBigEnough<CAPACITY, LenType>: True,
{
    type Target = [T];

    #[inline(always)]
    fn deref(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T, const CAPACITY: usize, LenType: const Usizeable> std::ops::DerefMut
    for MicroVec<T, CAPACITY, LenType>
where
    LenTypeBigEnough<CAPACITY, LenType>: True,
{
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut [T] {
        self.as_mut_slice()
    }
}

// Blanket trait implementations, defer actual implementation to std::slice by dereferencing

impl<T, const CAPACITY: usize, LenType: const Usizeable, I: SliceIndex<[T]>> Index<I>
    for MicroVec<T, CAPACITY, LenType>
where
    LenTypeBigEnough<CAPACITY, LenType>: True,
{
    type Output = I::Output;

    fn index(&self, index: I) -> &Self::Output {
        &(**self)[index]
    }
}

impl<T, const CAPACITY: usize, LenType: const Usizeable, I: SliceIndex<[T]>> IndexMut<I>
    for MicroVec<T, CAPACITY, LenType>
where
    LenTypeBigEnough<CAPACITY, LenType>: True,
{
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        &mut (**self)[index]
    }
}

impl<'a, T, const CAPACITY: usize, LenType: const Usizeable> IntoIterator
    for &'a MicroVec<T, CAPACITY, LenType>
where
    LenTypeBigEnough<CAPACITY, LenType>: True,
{
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T, const CAPACITY: usize, LenType: const Usizeable> IntoIterator
    for &'a mut MicroVec<T, CAPACITY, LenType>
where
    LenTypeBigEnough<CAPACITY, LenType>: True,
{
    type Item = &'a mut T;
    type IntoIter = std::slice::IterMut<'a, T>;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<T: std::fmt::Debug, const CAPACITY: usize, LenType: const Usizeable> std::fmt::Debug
    for MicroVec<T, CAPACITY, LenType>
where
    LenTypeBigEnough<CAPACITY, LenType>: True,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (**self).fmt(f)
    }
}

impl<T: std::hash::Hash, const CAPACITY: usize, LenType: const Usizeable> std::hash::Hash
    for MicroVec<T, CAPACITY, LenType>
where
    LenTypeBigEnough<CAPACITY, LenType>: True,
{
    #[inline(always)]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        (**self).hash(state)
    }
}

impl<T: PartialEq, const CAPACITY: usize, LenType: const Usizeable>
    PartialEq<MicroVec<T, CAPACITY, LenType>> for MicroVec<T, CAPACITY, LenType>
where
    LenTypeBigEnough<CAPACITY, LenType>: True,
{
    #[inline(always)]
    fn eq(&self, other: &MicroVec<T, CAPACITY, LenType>) -> bool {
        (**self).eq(&**other)
    }
}

impl<T: PartialOrd, const CAPACITY: usize, LenType: const Usizeable>
    PartialOrd<MicroVec<T, CAPACITY, LenType>> for MicroVec<T, CAPACITY, LenType>
where
    LenTypeBigEnough<CAPACITY, LenType>: True,
{
    #[inline(always)]
    fn partial_cmp(&self, other: &MicroVec<T, CAPACITY, LenType>) -> Option<std::cmp::Ordering> {
        (**self).partial_cmp(&**other)
    }
}

impl<T: Eq, const CAPACITY: usize, LenType: const Usizeable> Eq for MicroVec<T, CAPACITY, LenType> where
    LenTypeBigEnough<CAPACITY, LenType>: True
{
}

impl<T: Ord, const CAPACITY: usize, LenType: const Usizeable> Ord for MicroVec<T, CAPACITY, LenType>
where
    LenTypeBigEnough<CAPACITY, LenType>: True,
{
    #[inline(always)]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (**self).cmp(&**other)
    }
}

/// Shorthand for constructing a `MicroVec` from an array literal.
///
/// 1. Push-type (const friendly), which constructs a new `MicroVec` and pushes elements sequentially.
///    The first argument must be an expression wrapped in curly braces which evaluates to a valid `MicroVec`,
///    the second argument must be an array literal (although an array won't actually be constructed).
/// ```
/// let vec = microvec!({ MicroVec::<u8, 10, u8>::new() } => [0; 10]);
/// ```
///
/// 2. Copy-type (more performant), which constructs an array and copies it into a `MicroVec`.
///    The first argument must be a valid type specifier of a `MicroVec`, and the second argument must be
///    an array literal which will be constructed and then copied into a `MicroVec`. `T` must implement `Copy`.
/// ```
/// let vec = microvec!(MicroVec::<u8, 10, u8> => [1, 2, 3, 4, 5]);
/// ```
#[macro_export]
macro_rules! microvec {
    ( { $t:expr } => [$( $x:expr ),*] ) => {
        {
            let mut vec = $t;
            $(vec.push($x);)*
            vec
        }
    };
    ( { $t:expr } => [$x:expr; $count:expr] ) => {
        {
            let mut vec = $t;
            let mut i = 0;

            while i < $count {
                vec.push($x);
                i += 1;
            }

            vec
        }
    };
    ( $t:ty => [$( $x:expr ),*] ) => {
        {
            <$t>::from([$($x,)*].into())
        }
    };
    ( $t:ty => [$x:expr; $count:expr] ) => {
        {
            <$t>::from([$x; $count].into())
        }
    };
}
