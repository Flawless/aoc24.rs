use crate::util::fetch;

pub trait Solution {
    fn part1(&self, input: &str) -> String;
    fn part2(&self, input: &str) -> String;
    fn day(&self) -> u8;

    fn solution(&self) -> String {
        let input = fetch::input(self.day()).unwrap();
        let part1 = self.part1(&input);
        let part2 = self.part2(&input);
        format!("Part 1: {}, Part 2: {}", part1, part2)
    }
}

// Simple macro for day setup
#[macro_export]
macro_rules! aoc_solution {
    ($day_num:expr, $part1_fn:ident, $part2_fn:ident) => {
        pub struct Sol;

        impl crate::util::aoc::Solution for Sol {
            fn day(&self) -> u8 {
                $day_num
            }

            fn part1(&self, input: &str) -> String {
                $part1_fn(input).to_string()
            }

            fn part2(&self, input: &str) -> String {
                $part2_fn(input).to_string()
            }
        }

        pub fn solution() -> String {
            Sol.solution()
        }
    };
}
