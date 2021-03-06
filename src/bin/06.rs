/* Problem 6: Sum square difference
 *
 * The sum of the squares of the first ten natural numbers is,
 * 1² + 2² + ... + 10² = 385
 *
 * The square of the sum of the first ten natural numbers is,
 * (1 + 2 + ... + 10)² = 55² = 3025
 *
 * Hence the difference between the sum of the squares of the first ten natural numbers and the
 * square of the sum is 3025 - 385 = 2640.
 *
 * Find the difference between the sum of the squares of the first one hundred natural numbers and
 * the square of the sum. */

use shared::combinations;

// (a + b + c + ...)² = a² + b² + c² + ... + 2ab + 2ac + 2ad + 2bc...
// (a + b + c + ...)² - (a² + b² + c² + ...) = 2ab + 2ac + 2ad + 2bc...
fn main() {
    let first_natural_numbers: Vec<u32> = (1..101).collect();

    let terms = combinations::new(first_natural_numbers)
        .filter(|&(a, b)| a != b) // No 2 * a * a
        .filter(|&(a, b)| a < b) // No 2 * a * b + 2 * b * a
        .map(|(a, b)| 2 * a * b);

    let result = terms.fold(0, |result, term| result + term);
    println!("{}", result);
}
