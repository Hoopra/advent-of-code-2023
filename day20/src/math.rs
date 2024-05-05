use num::integer::lcm;

pub fn lowest_common_multiple(numbers: Vec<u64>) -> u64 {
    numbers
        .iter()
        .skip(1)
        .fold(*numbers.get(0).unwrap(), |previous, next| {
            lcm::<u64>(previous, *next)
        })
}
