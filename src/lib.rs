#![no_std]
use core::ops::{Div, Rem, ShrAssign};

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

pub trait NumToAscii<const B: usize, const D: usize>:
    Copy + ShrAssign + Div + Rem + PartialEq + Sized
{
    fn bits(self) -> [u8; B];

    fn bit_width(self) -> usize {
        self.bits()
            .into_iter()
            .enumerate()
            .filter(|(_, bit)| *bit == 1)
            .map(|(idx, _)| idx + 1)
            .last()
            .unwrap_or_else(|| 0)
    }

    fn n_as_po2() -> usize {
        let mut n = D;
        let mut res = 0;
        while n > 2 {
            n /= 2;
            res += 1;
        }

        res
    }

    fn max_digit_count() -> usize {
        let n = Self::n_as_po2() + 1;
        let div = n / 10;
        let rem = n % 10;
        let coefr = match rem {
            r if r > 7 => 3,
            r if r > 4 => 2,
            r if r > 0 => 1,
            _ => 0,
        };

        // + 1 accounts for max digits when all bits are on
        div * 3 + coefr + 1
    }

    fn get_num_digits(self) -> ([u8; D], usize);

    fn asciify(self) -> ([u8; D], usize) {
        let (mut digits, size) = self.get_num_digits();
        digits.iter_mut().for_each(|d| {
            *d = byte_to_ascii(*d);
        });

        (digits, size)
    }
}

macro_rules! num_to_ascii {
    ($bits: expr, $digits: expr, $numty: ty) => {
        impl NumToAscii<$bits, { $digits }> for $numty {
            fn bits(mut self) -> [u8; $bits] {
                let mut bits = [0u8; $bits];
                for idx in 0..8 {
                    bits[idx] = self as u8 & 1;
                    self >>= 1;
                }

                bits
            }

            fn get_num_digits(mut self) -> ([u8; $digits], usize) {
                let mut nums = [0; _];
                let mut idx = 0;
                while idx < nums.len() {
                    let next = self / 10;
                    if next == 0 {
                        nums[idx] = self as u8;
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
    ($({ bits = $bits: expr , digits = $digits: expr, type = $numty: ty }),+) => {
        $(
            num_to_ascii!($bits, $digits, $numty);
        )+
    }
}

num_to_ascii_many!(
    { bits = 8, digits = digit_count(8) + 1, type = u8 },
    { bits = 16, digits = digit_count(16) + 1, type = u16 },
    { bits = 32, digits = digit_count(32) + 1, type = u32 },
    { bits = 64, digits = digit_count(64) + 1, type = u64 },
    { bits = 128, digits = digit_count(128) + 1, type = u128 }
);
