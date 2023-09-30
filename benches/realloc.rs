/*
 * Copyright (c) Adrian Alic <contact@alic.dev>
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

#![feature(test)]
#![feature(generic_const_exprs)]
#![feature(adt_const_params)]
#![feature(const_trait_impl)]
#![feature(const_mut_refs)]
#![feature(const_refs_to_cell)]
#![feature(trait_alias)]
#![feature(return_position_impl_trait_in_trait)]
#![feature(pointer_is_aligned)]
#![feature(inline_const)]

use std::hint::black_box;

use test::Bencher;

extern crate test;

trait Bar {
    type K: Ord;
    fn xyz(&self) -> Self::K;
}
struct BarImpl;

impl Bar for BarImpl { 
    fn xyz(&self) -> Self::K { 0 }
    type K = u32;
}

use std::marker::ConstParamTy;

#[derive(PartialEq, Eq, ConstParamTy)]
struct Vector {
    x: usize,
}
const fn proj(v: Vector) -> usize {
    v.x
}
struct Foo<const V: Vector>
where
    [(); proj(V)]:,
{
    x: [u8; proj(V)],
}

fn foo() {
    const v_example: Vector = Vector { x: 12 };
    let x = Foo::<v_example> { x: [0u8; 12] };
}

#[bench]
pub fn reserve_4gib(b: &mut Bencher) {
    let mut v = Vec::<u8>::with_capacity(4 * 1024 * 1024 * 1024);
    b.iter(|| {
        v.clear();
        for _ in 0..(4 * 1024 * 1024 * 64) {
            v.push(0xff);
        }
        black_box(&mut v);
    })
}

#[bench]
pub fn reserve_16kib(b: &mut Bencher) {
    let mut v = Vec::<u8>::with_capacity(4 * 1024 * 16);
    b.iter(|| {
        v.clear();
        for _ in 0..(4 * 1024 * 1024 * 64) {
            v.push(0xff);
        }
        black_box(&mut v);
    })
}
