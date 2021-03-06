/* Problem 12: Highly divisible triangular number
 *
 * The sequence of triangle numbers is generated by adding the natural numbers. So the 7th triangle
 * number would be 1 + 2 + 3 + 4 + 5 + 6 + 7 = 28. The first ten terms would be:
 *
 * 1, 3, 6, 10, 15, 21, 28, 36, 45, 55, ...
 *
 * Let us list the factors of the first seven triangle numbers:
 *
 *    1: 1
 *    3: 1,3
 *    6: 1,2,3,6
 *   10: 1,2,5,10
 *   15: 1,3,5,15
 *   21: 1,3,7,21
 *   28: 1,2,4,7,14,28
 *
 * We can see that 28 is the first triangle number to have over five divisors.
 *
 * What is the value of the first triangle number to have over five hundred divisors? */

const NUMBER_OF_DIVISORS: u64 = 500;

fn main() {
    let result = triangular_numbers::new().find(|&num| {
        let divisor_count = (1..(num as f64).sqrt() as u64).fold(0, |sum, candidate| {
            if num % candidate == 0 {
                sum + 2
            } else {
                sum
            }
        });

        divisor_count > NUMBER_OF_DIVISORS
    });

    println!("{}", result.unwrap());
}

mod triangular_numbers {
    pub struct TriangularNumbers {
        last_number: u64,
        sum: u64,
    }

    impl Iterator for TriangularNumbers {
        type Item = u64;

        fn next(&mut self) -> Option<u64> {
            self.last_number += 1;
            self.sum += self.last_number;

            Some(self.sum)
        }
    }

    pub fn new() -> TriangularNumbers {
        TriangularNumbers {
            last_number: 0,
            sum: 0,
        }
    }
}
