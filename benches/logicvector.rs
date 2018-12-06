#![feature(test)]
extern crate test;

use logical::Ieee1164;
use logical::LogicVector;
use test::black_box as bb;
use test::Bencher;

const NITER: u128 = 10_000;

#[bench]
fn create_from_int(b: &mut Bencher) {
    b.iter(|| {
        for i in 0..NITER {
            bb(LogicVector::from_int(i, 128));
        }
    });
}

#[bench]
fn create_from_vec(b: &mut Bencher) {
    b.iter(|| {
        for i in 0..NITER {
            bb(LogicVector::from(vec![Ieee1164::_U; ((i % 127) + 1) as usize]));
        }
    })
}

#[bench]
fn create_from_str(b: &mut Bencher) {
    b.iter(|| {
        for i in 0..NITER {
            bb("U".repeat((i % 128) as usize));
        }
    })
}

#[bench]
fn create_width(b: &mut Bencher) {
    b.iter(|| {
        for i in 0..NITER {
            bb(LogicVector::with_width(((i % 128) + 1) as u8));
        }
    })
}

#[bench]
fn to_u128(b: &mut Bencher) {
    b.iter(|| {
        for i in 0..NITER {
            assert_eq!(Some(i), bb(LogicVector::from_int(i, 128)).unwrap().as_u128());
        }
    })
}
