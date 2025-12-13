use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

/// Work in progress implementation of a generic point on an N-dimensional coordinate space
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[repr(C)]
pub struct GenericPoint<const N: usize, T> {
    coordinates: [T; N],
}

impl<T: std::fmt::Debug, const N: usize> std::fmt::Debug for GenericPoint<N, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.coordinates.fmt(f)
    }
}

macro_rules! access_methods {
    ($index:expr, $fn:ident, $fn_mut:ident) => {
        #[inline(always)]
        pub const fn $fn(&self) -> T {
            unsafe { self.get_unchecked($index) }
        }

        #[inline(always)]
        pub const fn $fn_mut(&mut self) -> &mut T {
            unsafe { self.get_unchecked_mut($index) }
        }
    };
}

impl<T: Copy> GenericPoint<1, T> {
    #[inline(always)]
    pub const fn new(x: T) -> Self {
        Self { coordinates: [x] }
    }

    access_methods!(0, x, x_mut);
}

impl<T: Copy> GenericPoint<2, T> {
    #[inline(always)]
    pub const fn new(x: T, y: T) -> Self {
        Self {
            coordinates: [x, y],
        }
    }

    access_methods!(0, x, x_mut);
    access_methods!(1, y, y_mut);
}

impl<T: Copy> GenericPoint<3, T> {
    #[inline(always)]
    pub const fn new(x: T, y: T, z: T) -> Self {
        Self {
            coordinates: [x, y, z],
        }
    }

    access_methods!(0, x, x_mut);
    access_methods!(1, y, y_mut);
    access_methods!(2, z, z_mut);
}

impl<const N: usize, T: Copy> GenericPoint<N, T> {
    #[inline(always)]
    const unsafe fn get_unchecked(&self, index: usize) -> T {
        unsafe { self.coordinates.as_ptr().add(index).read() }
    }

    #[inline(always)]
    const unsafe fn get_unchecked_mut(&mut self, index: usize) -> &mut T {
        unsafe { self.coordinates.as_mut_ptr().add(index).as_mut_unchecked() }
    }

    #[inline(always)]
    fn _add(&mut self, rhs: Self)
    where
        T: AddAssign,
    {
        unsafe {
            match N {
                2 => {
                    *self.get_unchecked_mut(0) += rhs.get_unchecked(0);
                    *self.get_unchecked_mut(1) += rhs.get_unchecked(1);
                }
                3 => {
                    *self.get_unchecked_mut(0) += rhs.get_unchecked(0);
                    *self.get_unchecked_mut(1) += rhs.get_unchecked(1);
                    *self.get_unchecked_mut(2) += rhs.get_unchecked(2);
                }
                _ => {
                    for i in 0..N {
                        *self.get_unchecked_mut(i) += rhs.get_unchecked(i);
                    }
                }
            }
        }
    }

    #[inline(always)]
    fn _sub(&mut self, rhs: Self)
    where
        T: SubAssign,
    {
        unsafe {
            match N {
                2 => {
                    *self.get_unchecked_mut(0) -= rhs.get_unchecked(0);
                    *self.get_unchecked_mut(1) -= rhs.get_unchecked(1);
                }
                3 => {
                    *self.get_unchecked_mut(0) -= rhs.get_unchecked(0);
                    *self.get_unchecked_mut(1) -= rhs.get_unchecked(1);
                    *self.get_unchecked_mut(2) -= rhs.get_unchecked(2);
                }
                _ => {
                    for i in 0..N {
                        *self.get_unchecked_mut(i) -= rhs.get_unchecked(i);
                    }
                }
            }
        }
    }

    #[inline(always)]
    fn _mul(&mut self, rhs: Self)
    where
        T: MulAssign,
    {
        unsafe {
            match N {
                2 => {
                    *self.get_unchecked_mut(0) *= rhs.get_unchecked(0);
                    *self.get_unchecked_mut(1) *= rhs.get_unchecked(1);
                }
                3 => {
                    *self.get_unchecked_mut(0) *= rhs.get_unchecked(0);
                    *self.get_unchecked_mut(1) *= rhs.get_unchecked(1);
                    *self.get_unchecked_mut(2) *= rhs.get_unchecked(2);
                }
                _ => {
                    for i in 0..N {
                        *self.get_unchecked_mut(i) *= rhs.get_unchecked(i);
                    }
                }
            }
        }
    }

    #[inline(always)]
    fn _div(&mut self, rhs: Self)
    where
        T: DivAssign,
    {
        unsafe {
            match N {
                2 => {
                    *self.get_unchecked_mut(0) /= rhs.get_unchecked(0);
                    *self.get_unchecked_mut(1) /= rhs.get_unchecked(1);
                }
                3 => {
                    *self.get_unchecked_mut(0) /= rhs.get_unchecked(0);
                    *self.get_unchecked_mut(1) /= rhs.get_unchecked(1);
                    *self.get_unchecked_mut(2) /= rhs.get_unchecked(2);
                }
                _ => {
                    for i in 0..N {
                        *self.get_unchecked_mut(i) /= rhs.get_unchecked(i);
                    }
                }
            }
        }
    }

    #[inline(always)]
    pub fn euclidian2<U>(self, rhs: Self) -> U
    where
        U: Copy
            + PartialOrd
            + From<i8>
            + From<T>
            + Add<Output = U>
            + Sub<Output = U>
            + Mul<Output = U>,
    {
        match N {
            2 => {
                let mut ax = U::from(unsafe { self.get_unchecked(0) });
                let mut ay = U::from(unsafe { self.get_unchecked(1) });
                let mut bx = U::from(unsafe { rhs.get_unchecked(0) });
                let mut by = U::from(unsafe { rhs.get_unchecked(1) });

                if bx > ax {
                    std::mem::swap(&mut ax, &mut bx);
                }

                if by > ay {
                    std::mem::swap(&mut ay, &mut by);
                }

                let dx = ax - bx;
                let dy = ay - by;

                dx * dx + dy * dy
            }
            3 => {
                let mut ax = U::from(unsafe { self.get_unchecked(0) });
                let mut ay = U::from(unsafe { self.get_unchecked(1) });
                let mut az = U::from(unsafe { self.get_unchecked(2) });
                let mut bx = U::from(unsafe { rhs.get_unchecked(0) });
                let mut by = U::from(unsafe { rhs.get_unchecked(1) });
                let mut bz = U::from(unsafe { rhs.get_unchecked(2) });

                if bx > ax {
                    std::mem::swap(&mut ax, &mut bx);
                }

                if by > ay {
                    std::mem::swap(&mut ay, &mut by);
                }

                if bz > az {
                    std::mem::swap(&mut az, &mut bz);
                }

                let dx = ax - bx;
                let dy = ay - by;
                let dz = az - bz;

                dx * dx + dy * dy + dz * dz
            }
            _ => {
                let mut result = U::from(0);

                for i in 0..N {
                    let mut a = U::from(unsafe { self.get_unchecked(i) });
                    let mut b = U::from(unsafe { rhs.get_unchecked(i) });

                    if b > a {
                        std::mem::swap(&mut a, &mut b);
                    }

                    let d = a - b;
                    result = result + d * d;
                }

                result
            }
        }
    }

    #[inline(always)]
    pub fn scalar(mut self, rhs: T) -> Self
    where
        T: MulAssign,
    {
        unsafe {
            match N {
                2 => {
                    *self.get_unchecked_mut(0) *= rhs;
                    *self.get_unchecked_mut(1) *= rhs;
                }
                3 => {
                    *self.get_unchecked_mut(0) *= rhs;
                    *self.get_unchecked_mut(1) *= rhs;
                    *self.get_unchecked_mut(2) *= rhs;
                }
                _ => {
                    for i in 0..N {
                        *self.get_unchecked_mut(i) *= rhs;
                    }
                }
            }
        }

        self
    }
}

impl<const N: usize, T: Copy + AddAssign> Add for GenericPoint<N, T> {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self._add(rhs);
        self
    }
}

impl<const N: usize, T: Copy + AddAssign> AddAssign for GenericPoint<N, T> {
    fn add_assign(&mut self, rhs: Self) {
        self._add(rhs);
    }
}

impl<const N: usize, T: Copy + SubAssign> Sub for GenericPoint<N, T> {
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output {
        self._sub(rhs);
        self
    }
}

impl<const N: usize, T: Copy + SubAssign> SubAssign for GenericPoint<N, T> {
    fn sub_assign(&mut self, rhs: Self) {
        self._sub(rhs);
    }
}

impl<const N: usize, T: Copy + MulAssign> Mul for GenericPoint<N, T> {
    type Output = Self;

    fn mul(mut self, rhs: Self) -> Self::Output {
        self._mul(rhs);
        self
    }
}

impl<const N: usize, T: Copy + MulAssign> MulAssign for GenericPoint<N, T> {
    fn mul_assign(&mut self, rhs: Self) {
        self._mul(rhs);
    }
}

impl<const N: usize, T: Copy + DivAssign> Div for GenericPoint<N, T> {
    type Output = Self;

    fn div(mut self, rhs: Self) -> Self::Output {
        self._div(rhs);
        self
    }
}

impl<const N: usize, T: Copy + DivAssign> DivAssign for GenericPoint<N, T> {
    fn div_assign(&mut self, rhs: Self) {
        self._div(rhs);
    }
}

impl<T> From<(T, T)> for GenericPoint<2, T> {
    fn from(value: (T, T)) -> Self {
        Self {
            coordinates: [value.0, value.1],
        }
    }
}

impl<T> From<(T, T, T)> for GenericPoint<3, T> {
    fn from(value: (T, T, T)) -> Self {
        Self {
            coordinates: [value.0, value.1, value.2],
        }
    }
}
