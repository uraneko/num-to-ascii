#![no_std]
use core::ops::{Div, Rem};

fn byte_to_ascii(byte: u8) -> u8 {
    match byte {
        0 => b'0',
        1 => b'1',
        2 => b'2',
        3 => b'3',
        4 => b'4',
        5 => b'5',
        6 => b'6',
        7 => b'7',
        8 => b'8',
        9 => b'9',
        _ => unreachable!("this fn is only called where the input is assured to be inside 0..=9"),
    }
}

pub const fn digit_count(n: usize) -> usize {
    let n = n + 1;
    let div = n / 10;
    let rem = n % 10;
    let coefr = match rem {
        r if r > 7 => 3,
        r if r > 4 => 2,
        r if r > 0 => 1,
        _ => 0,
    };

    div * 3 + coefr
}

pub trait NumToAscii<const D: usize>: Copy + Div + Rem + PartialEq + Sized {
    fn get_num_digits(self) -> ([u8; D], usize);

    fn ascii_bytes(self) -> ([u8; D], usize) {
        let (mut digits, size) = self.get_num_digits();
        let dgtcp = digits;
        for idx in 0..size {
            digits[idx] = byte_to_ascii(dgtcp[size - idx - 1]);
        }

        (digits, size)
    }
}

macro_rules! num_to_ascii {
    ($digits: expr, $numty: ty) => {
        impl NumToAscii<{ $digits }> for $numty {
            fn get_num_digits(mut self) -> ([u8; $digits], usize) {
                let mut nums = [0; _];
                let mut idx = 0;
                while idx < nums.len() {
                    extern crate std;
                    let next = self / 10;
                    if next == 0 {
                        nums[idx] = self as u8;
                        idx += 1;
                        break;
                    }

                    nums[idx] = (self % 10) as u8;
                    self = next;
                    idx += 1;
                }

                (nums, idx)
            }
        }
    };
}

macro_rules! num_to_ascii_many {
    ($({  digits = $digits: expr, type = $numty: ty }),+) => {
        $(
            num_to_ascii!($digits, $numty);
        )+
    }
}

num_to_ascii_many!(
    { digits = digit_count(8) + 1, type = u8 },
    { digits = digit_count(16) + 1, type = u16 },
    { digits = digit_count(32) + 1, type = u32 },
    { digits = digit_count(64) + 1, type = u64 },
    { digits = digit_count(64) + 1, type = usize },
    { digits = digit_count(128) + 1, type = u128 },
    { digits = digit_count(8) + 1, type = i8 },
    { digits = digit_count(16) + 1, type = i16 },
    { digits = digit_count(32) + 1, type = i32 },
    { digits = digit_count(64) + 1, type = i64 },
    { digits = digit_count(64) + 1, type = isize },
    { digits = digit_count(128) + 1, type = i128 }

);

#[cfg(test)]
mod tests {
    use super::NumToAscii;

    #[test]
    fn u8() {
        let res = b"168";
        let (ascii, size) = 168u8.ascii_bytes();

        assert_eq!(&ascii[..size], res.as_slice());
    }

    #[test]
    fn u16() {
        let res = b"1012";
        let (ascii, size) = 1012u16.ascii_bytes();

        assert_eq!(&ascii[..size], res.as_slice());
    }

    #[test]
    fn u32() {
        let res = b"3560470";
        let (ascii, size) = 3560470u32.ascii_bytes();

        assert_eq!(&ascii[..size], res.as_slice());
    }

    #[test]
    fn i8() {
        let res = b"70";
        let (ascii, size) = 70i8.ascii_bytes();

        assert_eq!(&ascii[..size], res.as_slice());
    }

    #[test]
    fn i16() {
        let res = b"26345";
        let (ascii, size) = 26345i16.ascii_bytes();

        assert_eq!(&ascii[..size], res.as_slice());
    }

    #[test]
    fn i640() {
        let res = b"0";
        let (ascii, size) = 0i64.ascii_bytes();

        assert_eq!(&ascii[..size], res.as_slice());
    }

    #[test]
    fn i64() {
        let res = b"706534263454";
        let (ascii, size) = 706534263454i64.ascii_bytes();

        assert_eq!(&ascii[..size], res.as_slice());
    }
}
