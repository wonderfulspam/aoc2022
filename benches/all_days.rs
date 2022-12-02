use aoc2022::solutions::*;
use criterion::{criterion_group, criterion_main, Criterion};

pub fn benchmark_day01_part1(c: &mut Criterion) {
    c.bench_function("Day 01, Part 1", |b| b.iter(day01::part1));
}

pub fn benchmark_day01_part2(c: &mut Criterion) {
    c.bench_function("Day 01, Part 2", |b| b.iter(day01::part2));
}

criterion_group!(day01, benchmark_day01_part1, benchmark_day01_part2);

pub fn benchmark_day02_part1(c: &mut Criterion) {
    c.bench_function("Day 02, Part 1", |b| b.iter(day02::part1));
}

pub fn benchmark_day02_part2(c: &mut Criterion) {
    c.bench_function("Day 02, Part 2", |b| b.iter(day02::part2));
}

criterion_group!(day02, benchmark_day02_part1, benchmark_day02_part2);

criterion_main!(day01, day02);
