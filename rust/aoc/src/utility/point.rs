use std::ops::{Add, AddAssign, Mul, MulAssign};

/// Work in progress implementation of a generic point on an N-dimensional coordinate space
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[repr(C)]
pub struct GenericPoint<const N: usize, T> {
    coordinates: [T; N],
}

impl<T: Copy> GenericPoint<2, T> {
    #[inline(always)]
    pub const fn new(x: T, y: T) -> Self {
        Self {
            coordinates: [x, y],
        }
    }
}

impl<T: Copy> GenericPoint<3, T> {
    #[inline(always)]
    pub const fn new(x: T, y: T, z: T) -> Self {
        Self {
            coordinates: [x, y, z],
        }
    }
}

impl<const N: usize, T: Copy> GenericPoint<N, T>
where
    [(); (N > 0) as usize]: Sized,
{
    #[inline(always)]
    pub const fn x(&self) -> T {
        self.coordinates[0]
    }

    #[inline(always)]
    pub const fn x_mut(&mut self) -> &mut T {
        &mut self.coordinates[0]
    }
}

impl<const N: usize, T: Copy> GenericPoint<N, T>
where
    [(); (N > 1) as usize]: Sized,
{
    #[inline(always)]
    pub const fn y(&self) -> T {
        self.coordinates[1]
    }

    #[inline(always)]
    pub const fn y_mut(&mut self) -> &mut T {
        &mut self.coordinates[1]
    }
}

impl<const N: usize, T: Copy> GenericPoint<N, T>
where
    [(); (N > 2) as usize]: Sized,
{
    #[inline(always)]
    pub const fn z(&self) -> T {
        self.coordinates[2]
    }

    #[inline(always)]
    pub const fn z_mut(&mut self) -> &mut T {
        &mut self.coordinates[2]
    }
}

impl<const N: usize, T: Copy> GenericPoint<N, T>
where
    [(); (N > 0) as usize]: Sized,
{
    #[inline(always)]
    fn _add(&mut self, rhs: Self)
    where
        T: AddAssign,
    {
        match N {
            2 => {
                self.coordinates[0] += rhs.coordinates[0];
                self.coordinates[1] += rhs.coordinates[1];
            }
            3 => {
                self.coordinates[0] += rhs.coordinates[0];
                self.coordinates[1] += rhs.coordinates[1];
                self.coordinates[2] += rhs.coordinates[2];
            }
            _ => {
                for i in 0..N {
                    self.coordinates[i] += rhs.coordinates[i];
                }
            }
        }
    }

    #[inline(always)]
    fn _mul(&mut self, rhs: Self)
    where
        T: MulAssign,
    {
        match N {
            2 => {
                self.coordinates[0] *= rhs.coordinates[0];
                self.coordinates[1] *= rhs.coordinates[1];
            }
            3 => {
                self.coordinates[0] *= rhs.coordinates[0];
                self.coordinates[1] *= rhs.coordinates[1];
                self.coordinates[2] *= rhs.coordinates[2];
            }
            _ => {
                for i in 0..N {
                    self.coordinates[i] *= rhs.coordinates[i];
                }
            }
        }
    }

    #[inline(always)]
    pub fn scalar(mut self, rhs: T) -> Self
    where
        T: MulAssign,
    {
        match N {
            2 => {
                self.coordinates[0] *= rhs;
                self.coordinates[1] *= rhs;
            }
            3 => {
                self.coordinates[0] *= rhs;
                self.coordinates[1] *= rhs;
                self.coordinates[2] *= rhs;
            }
            _ => {
                for i in 0..N {
                    self.coordinates[i] *= rhs;
                }
            }
        }

        self
    }
}

impl<const N: usize, T> Add for GenericPoint<N, T>
where
    T: Copy + AddAssign,
    [(); (N > 0) as usize]: Sized,
{
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self._add(rhs);
        self
    }
}

impl<const N: usize, T> AddAssign for GenericPoint<N, T>
where
    T: Copy + AddAssign,
    [(); (N > 0) as usize]: Sized,
{
    fn add_assign(&mut self, rhs: Self) {
        self._add(rhs);
    }
}

impl<const N: usize, T> Mul for GenericPoint<N, T>
where
    T: Copy + MulAssign,
    [(); (N > 0) as usize]: Sized,
{
    type Output = Self;

    fn mul(mut self, rhs: Self) -> Self::Output {
        self._mul(rhs);
        self
    }
}

impl<const N: usize, T> MulAssign for GenericPoint<N, T>
where
    T: Copy + MulAssign,
    [(); (N > 0) as usize]: Sized,
{
    fn mul_assign(&mut self, rhs: Self) {
        self._mul(rhs);
    }
}
