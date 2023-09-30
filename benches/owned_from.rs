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

#[derive(Default, Clone)]
pub struct Simple1 {
    a: u128,
    b: Vec<u8>,
    c: Vec<u8>,
    d: Vec<u8>,
}
#[derive(Default, Clone)]
pub struct Simple2 {
    a: Vec<u8>,
    b: u128,
    c: Vec<u8>,
    d: u128,
    k: Vec<u8>,
}
#[derive(Default, Clone)]
pub struct Foo1 {
    f1: Simple1,
    f2: Simple1,
}
pub struct Foo2 {
    f3: Simple2,
    f4: Simple2,
}

#[inline(never)]
fn create_foo() -> Foo1 {
    let arr: [u8; 4096] = std::array::from_fn(|i| i as u8);
    let s1 = Simple1 {
        a: 0xf3102,
        b: arr.into_iter().collect(),
        c: arr.into_iter().collect(),
        d: arr.into_iter().collect(),
    };
    let arr: [u8; 4096] = std::array::from_fn(|i| (0usize.wrapping_sub(i)) as u8);
    let s2 = Simple1 {
        a: 0xf3102,
        b: arr.into_iter().collect(),
        c: arr.into_iter().collect(),
        d: arr.into_iter().collect(),
    };
    Foo1 { f1: s1, f2: s2 }
}

#[bench]
fn owned_from(b: &mut Bencher) {
    let foo = create_foo();
    b.iter(|| {
        let mut foo2 = convert_owned(foo.clone());
        black_box(&mut foo2);
    });
}
#[bench]
fn ref_from(b: &mut Bencher) {
    let foo = create_foo();
    b.iter(|| {
        let mut foo2 = convert_b(&foo);
        black_box(&mut foo2);
    });
}

#[inline(always)]
pub fn convert_owned(f: Foo1) -> Foo2 {
    Foo2 {
        f3: owned_1(f.f1),
        f4: owned_1(f.f2),
    }
}
#[inline(always)]
fn owned_1(f: Simple1) -> Simple2 {
    Simple2 {
        a: f.b.into_iter().map(|e| e.wrapping_add(123)).collect(),
        b: f.a,
        c: f.c.into_iter().map(|e| e.wrapping_add(123)).collect(),
        d: f.a,
        k: f.d.into_iter().map(|e| e.wrapping_add(123)).collect(),
    }
}

#[inline(always)]
pub fn convert_b(f: &Foo1) -> Foo2 {
    Foo2 {
        f3: owned_1b(&f.f1),
        f4: owned_1b(&f.f2),
    }
}
#[inline(always)]
fn owned_1b(f: &Simple1) -> Simple2 {
    Simple2 {
        a: f.b.iter().map(|e| e.wrapping_add(123)).collect(),
        b: f.a,
        c: f.c.iter().map(|e| e.wrapping_add(123)).collect(),
        d: f.a,
        k: f.d.iter().map(|e| e.wrapping_add(123)).collect(),
    }
}
