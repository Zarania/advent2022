use criterion::{black_box, criterion_group, criterion_main, Criterion};
use advent2022::read_file;
use advent2022::solutions::*;

pub fn bench_day1_part1(c: &mut Criterion) {
    let input = read_file("inputs", 1);
    c.bench_function("day1 part1", |b| b.iter(|| day01::part_one(black_box(&input))));
}

pub fn bench_day1_part2(c: &mut Criterion) {
    let input = read_file("inputs", 1);
    c.bench_function("day1 part2", |b| b.iter(|| day01::part_two(black_box(&input))));
}

pub fn bench_day2_part1(c: &mut Criterion) {
    let input = read_file("inputs", 2);
    c.bench_function("day2 part1", |b| b.iter(|| day02::part_one(black_box(&input))));
}

pub fn bench_day2_part2(c: &mut Criterion) {
    let input = read_file("inputs", 2);
    c.bench_function("day2 part2", |b| b.iter(|| day02::part_two(black_box(&input))));
}

pub fn bench_day3_part1(c: &mut Criterion) {
    let input = read_file("inputs", 3);
    c.bench_function("day3 part1", |b| b.iter(|| day03::part_one(black_box(&input))));
}

pub fn bench_day3_part2(c: &mut Criterion) {
    let input = read_file("inputs", 3);
    c.bench_function("day3 part2", |b| b.iter(|| day03::part_two(black_box(&input))));
}

pub fn bench_day4_part1(c: &mut Criterion) {
    let input = read_file("inputs", 4);
    c.bench_function("day4 part1", |b| b.iter(|| day04::part_one(black_box(&input))));
}

pub fn bench_day4_part2(c: &mut Criterion) {
    let input = read_file("inputs", 4);
    c.bench_function("day4 part2", |b| b.iter(|| day04::part_two(black_box(&input))));
}

pub fn bench_day5_part1(c: &mut Criterion) {
    let input = read_file("inputs", 5);
    c.bench_function("day5 part1", |b| b.iter(|| day05::part_one(black_box(&input))));
}

pub fn bench_day5_part2(c: &mut Criterion) {
    let input = read_file("inputs", 5);
    c.bench_function("day5 part2", |b| b.iter(|| day05::part_two(black_box(&input))));
}

criterion_group!(day01, bench_day1_part1, bench_day1_part2);
criterion_group!(day02, bench_day2_part1, bench_day2_part2);
criterion_group!(day03, bench_day3_part1, bench_day3_part2);
criterion_group!(day04, bench_day4_part1, bench_day4_part2);
criterion_group!(day05, bench_day5_part1, bench_day5_part2);
criterion_main!(day03);