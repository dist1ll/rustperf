/*
 * Copyright (c) Adrian Alic <contact@alic.dev>
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

#![feature(test)]

use std::{collections::BTreeMap, hint::black_box};

use test::Bencher;

extern crate test;

fn get_map() -> BTreeMap<u64, Vec<u8>> {
    let mut res = BTreeMap::new();
    for i in 0..100_000 {
        res.insert(i, Vec::new());
    }
    res
}
#[bench]
pub fn remove_in_order(b: &mut Bencher) {
    b.iter(|| {
        let mut m = get_map();
        for i in 0..100_000 {
            m.remove(&i);
        }
        assert_eq!(m.len(), 0);
        black_box(&mut m);
    })
}

#[bench]
pub fn remove_reverse_order(b: &mut Bencher) {
    b.iter(|| {
        let mut m = get_map();
        for i in (0..100_000).rev() {
            m.remove(&i);
        }
        assert_eq!(m.len(), 0);
        black_box(&mut m);
    })
}
