/* Problem 61: Cyclical figurate numbers
 *
 * Triangle, square, pentagonal, hexagonal, heptagonal, and octagonal numbers are
 * all figurate (polygonal) numbers and are generated by the following formulae:
 *
 * Triangle    P₃ₙ = n(n+1)/2   1, 3, 6, 10, 15, ...
 * Square      P₄ₙ = n²         1, 4, 9, 16, 25, ...
 * Pentagonal  P₅ₙ = n(3n−1)/2  1, 5, 12, 22, 35, ...
 * Hexagonal   P₆ₙ = n(2n−1)    1, 6, 15, 28, 45, ...
 * Heptagonal  P₇ₙ = n(5n−3)/2  1, 7, 18, 34, 55, ...
 * Octagonal   P₈ₙ = n(3n−2)    1, 8, 21, 40, 65, ...
 *
 * The ordered set of three 4-digit numbers: 8128, 2882, 8281, has three interesting properties.
 *    1. The set is cyclic, in that the last two digits of each number is the first two digits of
 *       the next number (including the last number with the first).
 *    2. Each polygonal type: triangle (P3,127=8128), square (P4,91=8281), and pentagonal
 *       (P5,44=2882), is represented by a different number in the set.
 *    3. This is the only set of 4-digit numbers with this property.
 *
 * Find the sum of the only ordered set of six cyclic 4-digit numbers for which each polygonal type:
 * triangle, square, pentagonal, hexagonal, heptagonal, and octagonal, is represented by a different
 * number in the set. */

#![feature(macro_rules)]

extern crate shared;
use shared::digits;

use std::mem;
use std::iter::AdditiveIterator;
use std::collections::TreeMap;
use std::collections::enum_set::{EnumSet, CLike};

struct NumberInfo {
    value: u32,
    first_digits: [u8, ..2],
    last_digits: [u8, ..2],
    classifications: EnumSet<PolygonalClassification>,
}

impl Clone for NumberInfo {
    fn clone(&self) -> NumberInfo {
        NumberInfo {
            value: self.value,
            first_digits: self.first_digits,
            last_digits: self.last_digits,
            classifications: self.classifications.clone()
        }
    }
}

fn main() {
    let candidates = build_candidates();
    let set = find_set(candidates);

    println!("{}", set.iter().map(|&n| n).sum());
}

fn build_candidates() -> Vec<NumberInfo> {
    let iterators = vec![
        PolygonalIterator::triangle_numbers(),
        PolygonalIterator::square_numbers(),
        PolygonalIterator::pentagonal_numbers(),
        PolygonalIterator::hexagonal_numbers(),
        PolygonalIterator::heptagonal_numbers(),
        PolygonalIterator::octagonal_numbers(),
    ];

    let mut map: TreeMap<u32, NumberInfo> = TreeMap::new();

    for iter in iterators.into_iter() {
        let iter_classification = iter.classification;
        let mut iter_candidates = iter
            .skip_while(|&n| n < 1_000)
            .take_while(|&n| n < 10_000);

        for candidate in iter_candidates {
            if !map.contains_key(& candidate) {
                let mut value_digits = digits::new::<_, u8>(candidate);

                let first_digits = [
                    value_digits.next().unwrap(),
                    value_digits.next().unwrap(),
                ];

                let last_digits  = [
                    value_digits.next().unwrap(),
                    value_digits.next().unwrap(),
                ];

                let classifications = EnumSet::new();

                map.insert(candidate, NumberInfo {
                    value: candidate,
                    classifications: classifications,
                    first_digits: first_digits,
                    last_digits:  last_digits
                });
            }

            map[candidate].classifications.insert(iter_classification);
        }
    }

    map.values().map(|info| info.clone()).collect()
}

fn find_set(candidates: Vec<NumberInfo>) -> Vec<u32> {
    struct Context<'a> {
        stack: Vec<NumberInfo>,
        used_types: EnumSet<PolygonalClassification>,
        candidates: &'a [NumberInfo],
    }

    return recurse(&mut Context {
        stack: Vec::new(),
        used_types: EnumSet::new(),
        candidates: candidates.as_slice()
    }).unwrap();

    fn recurse(context: &mut Context) -> Option<Vec<u32>> {
        if context.stack.len() == 6 {
            let result = context.stack.iter().map(|info| info.value).collect();
            return Some(result);
        }

        for candidate in context.candidates.iter() {
            if !context.used_types.is_disjoint(&candidate.classifications) {
                continue;
            }

            let fits_on_set = match context.stack.len() {
                0         => true,
                n @ 1...4 => context.stack[n - 1].last_digits == candidate.first_digits,
                5         => {
                    context.stack[4].last_digits == candidate.first_digits &&
                    candidate.last_digits == context.stack[0].first_digits
                },

                _ => unreachable!(),
            };

            if !fits_on_set { continue; }

            context.stack.push(candidate.clone());

            for classification in candidate.classifications.iter() {
                context.used_types.insert(classification);

                match recurse(context) {
                    None => {
                        context.used_types.remove(&classification);
                    },

                    result @ Some(_) => {
                        return result;
                    }
                }
            }

            context.stack.pop();
        }

        return None;
    }
}

struct PolygonalIterator {
    current_index: u32,
    formula: fn(u32) -> u32,
    classification: PolygonalClassification,
}

impl Iterator<u32> for PolygonalIterator {
    fn next(&mut self) -> Option<u32> {
        let value = (self.formula)(self.current_index);
        self.current_index += 1;

        Some(value)
    }
}

macro_rules! polygonal_formulas(
    ( $($name:ident, $fname:ident: $p:pat => $f:expr),+ ) => {
        impl PolygonalIterator {
            $(
                fn $fname() -> PolygonalIterator {
                    fn formula(index: u32) -> u32 {
                        match index {
                            $p => $f
                        }
                    }

                    PolygonalIterator {
                        current_index: 1,
                        formula: formula,
                        classification: PolygonalClassification::$name,
                    }
                }
            )+
        }

        #[deriving(Show, Clone, Copy)]
        #[repr(uint)]
        enum PolygonalClassification {
            $($name),+
        }

        impl CLike for PolygonalClassification {
            fn to_uint(&self) -> uint {
                *self as uint
            }

            fn from_uint(v: uint) -> PolygonalClassification {
                unsafe { mem::transmute(v) }
            }
        }
    }
)

polygonal_formulas!(
    Triangle,   triangle_numbers:    n => n * (n + 1) / 2,
    Square,     square_numbers:      n => n * n,
    Pentagonal, pentagonal_numbers:  n => n * (3*n - 1) / 2,
    Hexagonal,  hexagonal_numbers:   n => n * (2*n - 1),
    Heptagonal, heptagonal_numbers:  n => n * (5*n - 3) / 2,
    Octagonal,  octagonal_numbers:   n => n * (3*n - 2)
)
