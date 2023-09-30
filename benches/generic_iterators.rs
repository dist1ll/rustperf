/*
 * Copyright (c) Adrian Alic <contact@alic.dev>
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

#![feature(test)]
use std::{hint::black_box, ops::ControlFlow};

use test::Bencher;

extern crate test;

pub fn eq_slower<T: Eq>(a: impl Iterator<Item = T>, mut b: impl Iterator<Item = T>) -> bool {
    for x in a {
        let Some(y) = b.next() else { return false };
        if x != y {
            return false;
        }
    }
    b.next().is_none()
}

#[inline]
fn call<T, R>(mut f: impl FnMut(T) -> R) -> impl FnMut(T) -> R {
    move |x| f(x)
}
pub fn eq_my<T: Eq>(mut a: impl Iterator<Item = T>, mut b: impl Iterator<Item = T>) -> bool {
    while let Some(ref x) = a.next() {
        let res = call(|_: &T| {
            let Some(ref y) = b.next() else {
                return ControlFlow::Break(());
            };
            if x != y {
                return ControlFlow::Break(());
            }
            ControlFlow::Continue(())
        })(x)
        .is_break();

        if res {
            return false;
        }
    }
    b.next().is_none()
}

pub fn eq_faster<T: Eq>(mut a: impl Iterator<Item = T>, mut b: impl Iterator<Item = T>) -> bool {
    a.try_for_each(|x| {
        let Some(y) = b.next() else {
            return ControlFlow::Break(());
        };
        if x != y {
            return ControlFlow::Break(());
        };
        ControlFlow::Continue(())
    })
    .is_continue()
        && b.next().is_none()
}

#[bench]
pub fn for_iterator(b: &mut Bencher) {
    b.iter(|| {
        let iter1 = (0..1_000_000)
            .flat_map(|i| [i, i + 1].into_iter())
            .chain([1, 2, 3].into_iter());
        let iter2 = (0..1_000_000)
            .flat_map(|i| [i, i + 1].into_iter())
            .chain([4, 5, 6].into_iter());
        let mut result = eq_slower(iter1, iter2);
        black_box(&mut result);
    })
}
#[bench]
pub fn try_fold_iterator(b: &mut Bencher) {
    b.iter(|| {
        let iter1 = (0..1_000_000)
            .flat_map(|i| [i, i + 1].into_iter())
            .chain([1, 2, 3].into_iter());
        let iter2 = (0..1_000_000)
            .flat_map(|i| [i, i + 1].into_iter())
            .chain([4, 5, 6].into_iter());
        let mut result = eq_faster(iter1, iter2);
        black_box(&mut result);
    })
}
#[bench]
pub fn my_iterator(b: &mut Bencher) {
    b.iter(|| {
        let iter1 = (0..1_000_000)
            .flat_map(|i| [i, i + 1].into_iter())
            .chain([1, 2, 3].into_iter());
        let iter2 = (0..1_000_000)
            .flat_map(|i| [i, i + 1].into_iter())
            .chain([4, 5, 6].into_iter());
        let mut result = eq_my(iter1, iter2);
        black_box(&mut result);
    })
}
