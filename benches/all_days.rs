use aoc2022::solutions::*;
use criterion::{criterion_group, criterion_main, Criterion};

pub fn benchmark_day1_part1(c: &mut Criterion) {
    c.bench_function("Day 01, Part 1", |b| b.iter(day01::part1));
}

pub fn benchmark_day1_part2(c: &mut Criterion) {
    c.bench_function("Day 01, Part 2", |b| b.iter(day01::part2));
}

criterion_group!(day1, benchmark_day1_part1, benchmark_day1_part2);

pub fn benchmark_day2_part1(c: &mut Criterion) {
    c.bench_function("Day 02, Part 1", |b| b.iter(day02::part1));
}

pub fn benchmark_day2_part2(c: &mut Criterion) {
    c.bench_function("Day 02, Part 2", |b| b.iter(day02::part2));
}

criterion_group!(day2, benchmark_day2_part1, benchmark_day2_part2);

pub fn benchmark_day3_part1(c: &mut Criterion) {
    c.bench_function("Day 03, Part 1", |b| b.iter(day03::part1));
}

pub fn benchmark_day3_part2(c: &mut Criterion) {
    c.bench_function("Day 03, Part 2", |b| b.iter(day03::part2));
}

criterion_group!(day3, benchmark_day3_part1, benchmark_day3_part2);

pub fn benchmark_day4_part1(c: &mut Criterion) {
    c.bench_function("Day 04, Part 1", |b| b.iter(day04::part1));
}

pub fn benchmark_day4_part2(c: &mut Criterion) {
    c.bench_function("Day 04, Part 2", |b| b.iter(day04::part2));
}

criterion_group!(day4, benchmark_day4_part1, benchmark_day4_part2);

pub fn benchmark_day5_part1(c: &mut Criterion) {
    c.bench_function("Day 05, Part 1", |b| b.iter(day05::part1));
}

pub fn benchmark_day5_part2(c: &mut Criterion) {
    c.bench_function("Day 05, Part 2", |b| b.iter(day05::part2));
}

criterion_group!(day5, benchmark_day5_part1, benchmark_day5_part2);

criterion_main!(day1, day2, day3, day4, day5);
