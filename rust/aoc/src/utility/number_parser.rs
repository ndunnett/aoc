use std::ops::{Add, Neg, Shl};

/// Parses all numbers in a string to an arbitrary type, ignoring sign.
pub struct NumberParser<'a, T> {
    bytes: std::iter::Peekable<std::str::Bytes<'a>>,
    marker: std::marker::PhantomData<T>,
}

impl<'a, T> From<&'a str> for NumberParser<'a, T> {
    fn from(s: &'a str) -> Self {
        Self {
            bytes: s.bytes().peekable(),
            marker: std::marker::PhantomData,
        }
    }
}

impl<T> Iterator for NumberParser<'_, T>
where
    T: From<u8> + Clone + Copy + Shl + Add,
    <T as Shl>::Output: Add,
    <<T as Shl>::Output as Add>::Output: Add<T>,
    T: From<<<<T as Shl>::Output as Add>::Output as Add<T>>::Output>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(b) = self.bytes.next() {
            if b.is_ascii_digit() {
                let mut num = T::from(b - b'0');

                while let Some(b) = self.bytes.next_if(|b| b.is_ascii_digit()) {
                    num = T::from((num << T::from(1)) + (num << T::from(3)) + T::from(b - b'0'));
                }

                return Some(num);
            }
        }

        None
    }
}

/// Parses all numbers in a string to an arbitrary type, including sign.
pub struct NumberParserSigned<'a, T> {
    bytes: std::iter::Peekable<std::str::Bytes<'a>>,
    marker: std::marker::PhantomData<T>,
}

impl<'a, T> From<&'a str> for NumberParserSigned<'a, T> {
    fn from(s: &'a str) -> Self {
        Self {
            bytes: s.bytes().peekable(),
            marker: std::marker::PhantomData,
        }
    }
}

impl<T> Iterator for NumberParserSigned<'_, T>
where
    T: From<u8> + Clone + Copy + Shl + Add + Neg<Output = T>,
    <T as Shl>::Output: Add,
    <<T as Shl>::Output as Add>::Output: Add<T>,
    T: From<<<<T as Shl>::Output as Add>::Output as Add<T>>::Output>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(mut b) = self.bytes.next() {
            let mut negative = false;

            if b == b'-' {
                negative = true;
                b = self.bytes.next()?;
            }

            if b.is_ascii_digit() {
                let mut num = T::from(b - b'0');

                while let Some(b) = self.bytes.next_if(|b| b.is_ascii_digit()) {
                    num = T::from((num << T::from(1)) + (num << T::from(3)) + T::from(b - b'0'));
                }

                if negative {
                    return Some(-num);
                } else {
                    return Some(num);
                }
            }
        }

        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT_SMALL: &str = r"1 2 3, -56 asdf89: =-67";
    const TEST_INPUT_MEDIUM: &str = r"1 2 3, -456 asdf7890: =-4567";
    const TEST_INPUT_LARGE: &str = r"91928374971983984230 -12345562345345145134";

    #[test]
    fn i16_unsigned() {
        assert_eq!(
            NumberParser::from(TEST_INPUT_MEDIUM).collect::<Vec<i16>>(),
            vec![1, 2, 3, 456, 7890, 4567]
        );
    }

    #[test]
    fn i16_signed() {
        assert_eq!(
            NumberParserSigned::from(TEST_INPUT_MEDIUM).collect::<Vec<i16>>(),
            vec![1, 2, 3, -456, 7890, -4567]
        );
    }

    #[test]
    fn i32_unsigned() {
        assert_eq!(
            NumberParser::from(TEST_INPUT_MEDIUM).collect::<Vec<i32>>(),
            vec![1, 2, 3, 456, 7890, 4567]
        );
    }

    #[test]
    fn i32_signed() {
        assert_eq!(
            NumberParserSigned::from(TEST_INPUT_MEDIUM).collect::<Vec<i32>>(),
            vec![1, 2, 3, -456, 7890, -4567]
        );
    }

    #[test]
    fn i64_unsigned() {
        assert_eq!(
            NumberParser::from(TEST_INPUT_MEDIUM).collect::<Vec<i64>>(),
            vec![1, 2, 3, 456, 7890, 4567]
        );
    }

    #[test]
    fn i64_signed() {
        assert_eq!(
            NumberParserSigned::from(TEST_INPUT_MEDIUM).collect::<Vec<i64>>(),
            vec![1, 2, 3, -456, 7890, -4567]
        );
    }

    #[test]
    fn i128_unsigned() {
        assert_eq!(
            NumberParser::from(TEST_INPUT_MEDIUM).collect::<Vec<i128>>(),
            vec![1, 2, 3, 456, 7890, 4567]
        );
    }

    #[test]
    fn i128_signed() {
        assert_eq!(
            NumberParserSigned::from(TEST_INPUT_MEDIUM).collect::<Vec<i128>>(),
            vec![1, 2, 3, -456, 7890, -4567]
        );
    }

    #[test]
    fn isize_unsigned() {
        assert_eq!(
            NumberParser::from(TEST_INPUT_MEDIUM).collect::<Vec<isize>>(),
            vec![1, 2, 3, 456, 7890, 4567]
        );
    }

    #[test]
    fn isize_signed() {
        assert_eq!(
            NumberParserSigned::from(TEST_INPUT_MEDIUM).collect::<Vec<isize>>(),
            vec![1, 2, 3, -456, 7890, -4567]
        );
    }

    #[test]
    fn u8_unsigned() {
        assert_eq!(
            NumberParser::from(TEST_INPUT_SMALL).collect::<Vec<u8>>(),
            vec![1, 2, 3, 56, 89, 67]
        );
    }

    #[test]
    fn u16_unsigned() {
        assert_eq!(
            NumberParser::from(TEST_INPUT_MEDIUM).collect::<Vec<u16>>(),
            vec![1, 2, 3, 456, 7890, 4567]
        );
    }

    #[test]
    fn u32_unsigned() {
        assert_eq!(
            NumberParser::from(TEST_INPUT_MEDIUM).collect::<Vec<u32>>(),
            vec![1, 2, 3, 456, 7890, 4567]
        );
    }

    #[test]
    fn u64_unsigned() {
        assert_eq!(
            NumberParser::from(TEST_INPUT_MEDIUM).collect::<Vec<u64>>(),
            vec![1, 2, 3, 456, 7890, 4567]
        );
    }

    #[test]
    fn u128_unsigned() {
        assert_eq!(
            NumberParser::from(TEST_INPUT_MEDIUM).collect::<Vec<u128>>(),
            vec![1, 2, 3, 456, 7890, 4567]
        );
    }

    #[test]
    fn usize_unsigned() {
        assert_eq!(
            NumberParser::from(TEST_INPUT_MEDIUM).collect::<Vec<usize>>(),
            vec![1, 2, 3, 456, 7890, 4567]
        );
    }

    #[test]
    fn i128_unsigned_large() {
        assert_eq!(
            NumberParser::from(TEST_INPUT_LARGE).collect::<Vec<i128>>(),
            vec![91928374971983984230, 12345562345345145134]
        );
    }

    #[test]
    fn i128_signed_large() {
        assert_eq!(
            NumberParserSigned::from(TEST_INPUT_LARGE).collect::<Vec<i128>>(),
            vec![91928374971983984230, -12345562345345145134]
        );
    }

    #[test]
    fn u128_unsigned_large() {
        assert_eq!(
            NumberParser::from(TEST_INPUT_LARGE).collect::<Vec<u128>>(),
            vec![91928374971983984230, 12345562345345145134]
        );
    }
}
