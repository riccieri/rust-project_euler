use num::Integer;
use std::num::{Int, Float, FromPrimitive, ToPrimitive};
use std::fmt::Debug;
use std::old_io::IoResult;

pub struct Digits<A, B> {
    remaining: A,
    remaining_digits: u32,
}

impl<A, B> Iterator for Digits<A, B>
    where A: Integer + FromPrimitive + ToPrimitive, B: FromPrimitive {

    type Item = B;

    fn next(&mut self) -> Option<B> {
        if self.remaining_digits == 0 {
            return None;
        }

        let divisor: A = FromPrimitive::from_u32(self.current_divisor()).unwrap();
        let (digit, remainder) = self.remaining.div_rem(&divisor);

        self.remaining = remainder;
        self.remaining_digits -= 1;

        digit.to_u8().and_then(FromPrimitive::from_u8)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match self.remaining.to_f64() {
            Some(as_float) => {
                let log = as_float.log10();
                (log.floor() as usize, Some(log.ceil() as usize))
            },

            None => (0, None)
        }
    }

}

impl<A, B> DoubleEndedIterator for Digits<A, B>
    where A: Integer + FromPrimitive + ToPrimitive, B: FromPrimitive {
    fn next_back(&mut self) -> Option<B> {
        if self.remaining_digits == 0 {
            return None;
        }

        let ten: A = FromPrimitive::from_u8(10).unwrap();
        let (remainder, digit) = self.remaining.div_rem(&ten);

        self.remaining = remainder;
        self.remaining_digits -= 1;

        digit.to_u8().and_then(FromPrimitive::from_u8)
    }
}

pub fn new<A, B>(number: A) -> Digits<A, B>
    where A: ToPrimitive + Debug {
    Digits {
        remaining_digits: number_of_digits(&number),
        remaining: number,
    }
}

impl<A, B> Digits<A, B>
    where A: Integer + FromPrimitive + ToPrimitive {

    fn current_divisor(&self) -> u32 {
        10.pow((self.remaining_digits - 1) as usize)
    }

    pub fn count(self) -> u32 {
        self.remaining_digits
    }
}

fn number_of_digits<A: ToPrimitive + Debug>(number: &A) -> u32 {
    let mut counter = DigitCounter { count: 0 };
    (write!(&mut counter, "{:?}", number)).unwrap();

    counter.count
}

struct DigitCounter {
    count: u32,
}

impl Writer for DigitCounter {
    fn write_all(&mut self, buf: &[u8]) -> IoResult<()> {
        self.count += buf.len() as u32;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::new;

    #[test]
    fn test_digits_in_order() {
        let digits = new(12345us).collect::<Vec<u32>>();
        assert_eq!(&digits, &[1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_digits_in_reverse() {
        let digits = new(12345us).rev().collect::<Vec<u32>>();
        assert_eq!(&digits, &[5, 4, 3, 2, 1]);
    }

    #[test]
    fn test_digits_in_order_with_zero() {
        let digits = new(123450us).collect::<Vec<u32>>();
        assert_eq!(&digits, &[1, 2, 3, 4, 5, 0]);
    }

    #[test]
    fn test_digits_in_reverse_with_zero() {
        let digits = new(123450us).rev().collect::<Vec<u32>>();
        assert_eq!(&digits, &[0, 5, 4, 3, 2, 1]);
    }
}
