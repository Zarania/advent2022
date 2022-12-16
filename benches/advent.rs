use advent2022::read_file;
use advent2022::solutions::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn bench_day1_part1(c: &mut Criterion) {
    let input = read_file("inputs", 1);
    c.bench_function("day1 part1", |b| {
        b.iter(|| day01::part_one(black_box(&input)))
    });
}

pub fn bench_day1_part2(c: &mut Criterion) {
    let input = read_file("inputs", 1);
    c.bench_function("day1 part2", |b| {
        b.iter(|| day01::part_two(black_box(&input)))
    });
}

pub fn bench_day2_part1(c: &mut Criterion) {
    let input = read_file("inputs", 2);
    c.bench_function("day2 part1", |b| {
        b.iter(|| day02::part_one(black_box(&input)))
    });
}

pub fn bench_day2_part2(c: &mut Criterion) {
    let input = read_file("inputs", 2);
    c.bench_function("day2 part2", |b| {
        b.iter(|| day02::part_two(black_box(&input)))
    });
}

pub fn bench_day3_part1(c: &mut Criterion) {
    let input = read_file("inputs", 3);
    c.bench_function("day3 part1", |b| {
        b.iter(|| day03::part_one(black_box(&input)))
    });
}

pub fn bench_day3_part2(c: &mut Criterion) {
    let input = read_file("inputs", 3);
    c.bench_function("day3 part2", |b| {
        b.iter(|| day03::part_two(black_box(&input)))
    });
}

pub fn bench_day4_part1(c: &mut Criterion) {
    let input = read_file("inputs", 4);
    c.bench_function("day4 part1", |b| {
        b.iter(|| day04::part_one(black_box(&input)))
    });
}

pub fn bench_day4_part2(c: &mut Criterion) {
    let input = read_file("inputs", 4);
    c.bench_function("day4 part2", |b| {
        b.iter(|| day04::part_two(black_box(&input)))
    });
}

pub fn bench_day5_part1(c: &mut Criterion) {
    let input = read_file("inputs", 5);
    c.bench_function("day5 part1", |b| {
        b.iter(|| day05::part_one(black_box(&input)))
    });
}

pub fn bench_day5_part2(c: &mut Criterion) {
    let input = read_file("inputs", 5);
    c.bench_function("day5 part2", |b| {
        b.iter(|| day05::part_two(black_box(&input)))
    });
}

pub fn bench_day6_part1(c: &mut Criterion) {
    let input = read_file("inputs", 6);
    c.bench_function("day6 part1", |b| {
        b.iter(|| day06::part_one(black_box(&input)))
    });
}

pub fn bench_day6_part2(c: &mut Criterion) {
    let input = read_file("inputs", 6);
    c.bench_function("day6 part2", |b| {
        b.iter(|| day06::part_two(black_box(&input)))
    });
}

pub fn bench_day7_part1(c: &mut Criterion) {
    let input = read_file("inputs", 7);
    c.bench_function("day7 part1", |b| {
        b.iter(|| day07::part_one(black_box(&input)))
    });
}

pub fn bench_day7_part2(c: &mut Criterion) {
    let input = read_file("inputs", 7);
    c.bench_function("day7 part2", |b| {
        b.iter(|| day07::part_two(black_box(&input)))
    });
}

pub fn bench_day8_part1(c: &mut Criterion) {
    let input = read_file("inputs", 8);
    c.bench_function("day8 part1", |b| {
        b.iter(|| day08::part_one(black_box(&input)))
    });
}

pub fn bench_day8_part2(c: &mut Criterion) {
    let input = read_file("inputs", 8);
    c.bench_function("day8 part2", |b| {
        b.iter(|| day08::part_two(black_box(&input)))
    });
}

pub fn bench_day9_part1(c: &mut Criterion) {
    let input = read_file("inputs", 9);
    c.bench_function("day9 part1", |b| {
        b.iter(|| day09::part_one(black_box(&input)))
    });
}

pub fn bench_day9_part2(c: &mut Criterion) {
    let input = read_file("inputs", 9);
    c.bench_function("day9 part2", |b| {
        b.iter(|| day09::part_two(black_box(&input)))
    });
}

pub fn bench_day10_part1(c: &mut Criterion) {
    let input = read_file("inputs", 10);
    c.bench_function("day10 part1", |b| {
        b.iter(|| day10::part_one(black_box(&input)))
    });
}

pub fn bench_day10_part2(c: &mut Criterion) {
    let input = read_file("inputs", 10);
    c.bench_function("day10 part2", |b| {
        b.iter(|| day10::part_two(black_box(&input)))
    });
}

pub fn bench_day11_part1(c: &mut Criterion) {
    let input = read_file("inputs", 11);
    c.bench_function("day11 part1", |b| {
        b.iter(|| day11::part_one(black_box(&input)))
    });
}

pub fn bench_day11_part2(c: &mut Criterion) {
    let input = read_file("inputs", 11);
    c.bench_function("day11 part2", |b| {
        b.iter(|| day11::part_two(black_box(&input)))
    });
}

pub fn bench_day12_part1(c: &mut Criterion) {
    let input = read_file("inputs", 12);
    c.bench_function("day12 part1", |b| {
        b.iter(|| day12::part_one(black_box(&input)))
    });
}

pub fn bench_day12_part2(c: &mut Criterion) {
    let input = read_file("inputs", 12);
    c.bench_function("day12 part2", |b| {
        b.iter(|| day12::part_two(black_box(&input)))
    });
}

pub fn bench_day13_part1(c: &mut Criterion) {
    let input = read_file("inputs", 13);
    c.bench_function("day13 part1", |b| {
        b.iter(|| day13::part_one(black_box(&input)))
    });
}

pub fn bench_day13_part2(c: &mut Criterion) {
    let input = read_file("inputs", 13);
    c.bench_function("day13 part2", |b| {
        b.iter(|| day13::part_two(black_box(&input)))
    });
}

pub fn bench_day14_part1(c: &mut Criterion) {
    let input = read_file("inputs", 14);
    c.bench_function("day14 part1", |b| {
        b.iter(|| day14::part_one(black_box(&input)))
    });
}

pub fn bench_day14_part2(c: &mut Criterion) {
    let input = read_file("inputs", 14);
    c.bench_function("day14 part2", |b| {
        b.iter(|| day14::part_two(black_box(&input)))
    });
}

pub fn bench_day15_part1(c: &mut Criterion) {
    let input = read_file("inputs", 15);
    c.bench_function("day15 part1", |b| {
        b.iter(|| day15::part_one(black_box(&input)))
    });
}

pub fn bench_day15_part2(c: &mut Criterion) {
    let input = read_file("inputs", 15);
    c.bench_function("day15 part2", |b| {
        b.iter(|| day15::part_two(black_box(&input)))
    });
}

criterion_group!(day01, bench_day1_part1, bench_day1_part2);
criterion_group!(day02, bench_day2_part1, bench_day2_part2);
criterion_group!(day03, bench_day3_part1, bench_day3_part2);
criterion_group!(day04, bench_day4_part1, bench_day4_part2);
criterion_group!(day05, bench_day5_part1, bench_day5_part2);
criterion_group!(day06, bench_day6_part1, bench_day6_part2);
criterion_group!(day07, bench_day7_part1, bench_day7_part2);
criterion_group!(day08, bench_day8_part1, bench_day8_part2);
criterion_group!(day09, bench_day9_part1, bench_day9_part2);
criterion_group!(day10, bench_day10_part1, bench_day10_part2);
criterion_group!(day11, bench_day11_part1, bench_day11_part2);
criterion_group!(day12, bench_day12_part1, bench_day12_part2);
criterion_group!(day13, bench_day13_part1, bench_day13_part2);
criterion_group!(day14, bench_day14_part1, bench_day14_part2);
criterion_group!(day15, bench_day15_part1, bench_day15_part2);
criterion_main!(day15);
