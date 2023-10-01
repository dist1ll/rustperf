/*
 * Copyright (c) Adrian Alic <contact@alic.dev>
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::{collections::HashSet, time::Duration};

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rustc_hash::FxHashSet;

fn early_return(a: &[u32]) -> bool {
    for x in a {
        if *x == 0 {
            return true;
        }
    }
    false
}
fn full_scan(a: &[u32]) -> bool {
    let mut result = false;
    for x in a {
        if *x == 0 {
            result = true;
        }
    }
    result
}
fn hybrid(a: &[u32]) -> bool {
    for x in 0..(a.len() / 16) {
        let mut result = false;
        for i in 0..16 {
            if a[x * 16 + i] == 0 {
                result = true;
            }
        }
        if result {
            return true;
        }
    }
    false
}
fn contains(a: &[u32]) -> bool {
    a.contains(&0)
}
fn contains_hash(m: &FxHashSet<u32>) -> bool {
    m.contains(&0)
}

fn criterion_benchmark(c: &mut Criterion) {
    let count = 1024;
    let mut arr = Vec::with_capacity(count);
    for i in 0..count {
        if i == 20 {
            arr.push(0);
        } else {
            arr.push(i as u32 + 1);
        }
    }
    let hset: FxHashSet<u32> = arr.iter().cloned().collect();

    c.bench_function("early_return", |b| {
        b.iter(|| {
            black_box(early_return(arr.as_slice()));
        })
    });
    c.bench_function("full_scan", |b| {
        b.iter(|| {
            black_box(full_scan(arr.as_slice()));
        })
    });
    c.bench_function("stdlib_contains", |b| {
        b.iter(|| {
            black_box(contains(arr.as_slice()));
        })
    });
    c.bench_function("contains_set", |b| {
        b.iter(|| {
            black_box(contains_hash(&hset));
        })
    });
    c.bench_function("hybrid", |b| {
        b.iter(|| {
            black_box(hybrid(arr.as_slice()));
        })
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default()
        .warm_up_time(Duration::from_millis(400))
        .sample_size(50);
    targets = criterion_benchmark
}
criterion_main!(benches);

// #[bench]
// fn full_scan(b: &mut Bencher) {
//     #[inline(never)]
//     fn _impl(a: &[u32]) -> bool {
//         let mut result = false;
//         for x in a {
//             if *x == 0xfff {
//                 result = true;
//             }
//         }
//         result
//     }
//     let arr: [u32; 0xffff] = std::array::from_fn(|i| i as u32);
//     b.iter(|| {
//         let mut result = _impl(&arr);
//         black_box(&mut result);
//     });
// }
