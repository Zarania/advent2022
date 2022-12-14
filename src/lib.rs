#![feature(iter_array_chunks)]
#![feature(array_chunks)]
#![feature(array_windows)]
#![feature(get_many_mut)]
#![feature(slice_group_by)]

use std::env;
use std::fs;
use std::ops::ControlFlow;

pub mod solutions;

pub fn read_file(folder: &str, day: u8) -> String {
    let cwd = env::current_dir().unwrap();

    let filepath = cwd
        .join("src")
        .join(folder)
        .join(format!("day{day:02}.txt"));

    let f = fs::read_to_string(filepath);
    f.expect("could not open input file")
}

//fold the bytes exactly creating the number
pub fn int_from_bytes_exact<T>(s: &[u8]) -> T
where
    T: From<u8> + std::ops::Mul<T, Output = T> + std::ops::Add<T, Output = T>,
{
    s.iter().fold(T::from(0), |n, c| {
        let r = match c {
            b'0' => Some(T::from(0)),
            b'1' => Some(T::from(1)),
            b'2' => Some(T::from(2)),
            b'3' => Some(T::from(3)),
            b'4' => Some(T::from(4)),
            b'5' => Some(T::from(5)),
            b'6' => Some(T::from(6)),
            b'7' => Some(T::from(7)),
            b'8' => Some(T::from(8)),
            b'9' => Some(T::from(9)),
            _ => None,
        };
        if let Some(r) = r {
            n * T::from(10) + r
        } else {
            n
        }
    })
}

pub fn int_from_bytes_signed<T>(s: &[u8]) -> T
where
    T: From<i8> + std::ops::Mul<T, Output = T> + std::ops::Add<T, Output = T>,
{
    T::from((s[0] != b'-') as i8 * 2 - 1)
        * s.iter().fold(T::from(0), |n, c| {
            let r = match c {
                b'0' => T::from(0),
                b'1' => T::from(1),
                b'2' => T::from(2),
                b'3' => T::from(3),
                b'4' => T::from(4),
                b'5' => T::from(5),
                b'6' => T::from(6),
                b'7' => T::from(7),
                b'8' => T::from(8),
                b'9' => T::from(9),
                _ => T::from(0),
            };
            n * T::from(10) + r
        })
}

fn int_from_bytes_greedy<T>(s: &[u8]) -> T
where
    T: From<u8> + std::ops::Mul<T, Output = T> + std::ops::Add<T, Output = T>,
{
    match s.iter().try_fold(T::from(0), |n, c| {
        let r = match c {
            b'0' => Some(T::from(0)),
            b'1' => Some(T::from(1)),
            b'2' => Some(T::from(2)),
            b'3' => Some(T::from(3)),
            b'4' => Some(T::from(4)),
            b'5' => Some(T::from(5)),
            b'6' => Some(T::from(6)),
            b'7' => Some(T::from(7)),
            b'8' => Some(T::from(8)),
            b'9' => Some(T::from(9)),
            _ => None,
        };
        if let Some(r) = r {
            ControlFlow::Continue(n * T::from(10) + r)
        } else {
            ControlFlow::Break(n)
        }
    }) {
        ControlFlow::Continue(n) => n,
        ControlFlow::Break(n) => n,
    }
}

fn int_from_bytes_greedy_signed<T>(s: &[u8]) -> (T, usize)
where
    T: From<i8> + std::ops::Mul<T, Output = T> + std::ops::Add<T, Output = T>,
{
    let mut size = 0;
    match s.iter().try_fold(T::from(0), |n, c| {
        let r = match c {
            b'0' => Some(T::from(0)),
            b'1' => Some(T::from(1)),
            b'2' => Some(T::from(2)),
            b'3' => Some(T::from(3)),
            b'4' => Some(T::from(4)),
            b'5' => Some(T::from(5)),
            b'6' => Some(T::from(6)),
            b'7' => Some(T::from(7)),
            b'8' => Some(T::from(8)),
            b'9' => Some(T::from(9)),
            b'-' => Some(T::from(0)),
            _ => None,
        };
        if let Some(r) = r {
            size += 1;
            ControlFlow::Continue(n * T::from(10) + r)
        } else {
            ControlFlow::Break(n)
        }
    }) {
        ControlFlow::Continue(n) => (T::from((s[0] != b'-') as i8 * 2 - 1) * n, size),
        ControlFlow::Break(n) => (T::from((s[0] != b'-') as i8 * 2 - 1) * n, size),
    }
}
