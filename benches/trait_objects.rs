/*
 * Copyright (c) Adrian Alic <contact@alic.dev>
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

#![feature(test)]

use std::hint::black_box;

use test::Bencher;

extern crate test;

#[bench]
fn iterate_noinline_monomorphized_32kb(b: &mut Bencher) {
    let arr: [u8; 1 << 20] = std::array::from_fn(|i| i as u8);
    b.iter(|| {
        iterate_mono(&mut arr.iter());
        black_box(arr);
    });
}
#[bench]
fn iterate_noinline_dyn_32kb(b: &mut Bencher) {
    let arr: [u8; 1 << 20] = std::array::from_fn(|i| i as u8);
    b.iter(|| {
        iterate_dyn_inner(&mut arr.iter());
        black_box(arr);
    });
}
#[inline(never)]
fn iterate_mono<'a, I: Iterator<Item = &'a u8>>(x: &mut I) {
    let mut result = 0;
    while let Some(i) = x.next() {
        result += i;
    }
    black_box(result);
}
#[inline(never)]
fn iterate_dyn_inner(x: &mut dyn Iterator<Item = &u8>) {
    let mut result = 0;
    while let Some(i) = x.next() {
        result += i;
    }
    black_box(result);
}
