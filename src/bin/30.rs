/* Problem 30: Digit fifth powers
 *
 * Surprisingly there are only three numbers that can be written as the sum of fourth powers of
 * their digits:
 *
 *     1634 = 1⁴ + 6⁴ + 3⁴ + 4⁴
 *     8208 = 8⁴ + 2⁴ + 0⁴ + 8⁴
 *     9474 = 9⁴ + 4⁴ + 7⁴ + 4⁴
 *
 * As 1 = 1⁴ is not a sum it is not included.
 *
 * The sum of these numbers is 1634 + 8208 + 9474 = 19316.
 *
 * Find the sum of all the numbers that can be written as the sum of fifth powers of their digits */

const POWER: u32 = 5;

fn main() {
    let one_digit_max = 9u32.pow(POWER);

    let max_digits = (2..)
        .find(|&digits| {
            let min_with_digits = 10u32.pow(digits);
            one_digit_max * digits < min_with_digits
        })
        .unwrap();

    let result = (2..10u32.pow(max_digits))
        .filter_map(|num| {
            let sum = sum_of_digits_to_power(num, POWER);

            if sum == num {
                Some(sum)
            } else {
                None
            }
        })
        .fold(0, |acc, item| acc + item);

    println!("{}", result);
}

fn sum_of_digits_to_power(num: u32, power: u32) -> u32 {
    let mut result = 0;
    let mut rest = num;

    while rest > 0 {
        let digit = rest % 10;
        result += digit.pow(power);
        rest /= 10;
    }

    result
}
