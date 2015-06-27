#![feature(test)]

extern crate test;

pub fn sequential_fold(iterations: i32) -> i32 {
    (0..iterations).filter(|&item| item % 2 == 0)
        .map(|item| item * 2)
        .fold(0, |accumulator, item| accumulator + item)
}

pub fn all_in_fold(iterations: i32) -> i32 {
    (0..iterations).fold(0, |acc, item| {
        if item % 2 == 0 {
            acc + (item*2)
        } else {
            acc
        }
    })
}

pub fn raw_loop(iterations: i32) -> i32 {
    let mut sum = 0;
    for x in (0..iterations) {
        if x % 2 == 0 {
            sum = sum + (x*2)
        }
    };
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_sequential_fold(b: &mut Bencher) {
        b.iter(|| sequential_fold(1000000));
    }
    #[bench]
    fn bench_all_fold(b: &mut Bencher) {
        b.iter(|| all_in_fold(1000000));
    }
    #[bench]
    fn bench_raw_loop(b: &mut Bencher) {
        b.iter(|| raw_loop(1000000));
    }
}
