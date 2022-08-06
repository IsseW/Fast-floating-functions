#![feature(const_fn_floating_point_arithmetic, test)]
///! Fast approximate floating point functions from https://www.youtube.com/watch?v=ReTetN51r7A
use std::mem;

extern crate test;

const fn as_f32(i: u32) -> f32 {
    unsafe { mem::transmute(i) }
}

const fn as_u32(f: f32) -> u32 {
    unsafe { mem::transmute(f) }
}

// const NEG_ZERO: u32 = 0x80000000;

const ONE: u32 = as_u32(1.0);
const SCALE_UP: f32 = 8388608.0;
const SCALE_DOWN: f32 = 1.0 / SCALE_UP;

pub const fn log2(f: f32) -> f32 {
    (as_u32(f) - ONE) as f32 * SCALE_DOWN
}

pub const fn exp2(f: f32) -> f32 {
    as_f32((f * SCALE_UP) as u32 + ONE)
}

pub const fn pow(a: f32, b: f32) -> f32 {
    as_f32((b * (as_u32(a) - ONE) as f32) as u32 + ONE)
}

pub const fn sqrt(f: f32) -> f32 {
    as_f32((as_u32(f) >> 1) + (ONE >> 1))
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{Bencher, black_box};
    
    #[bench]
    fn std_log2(b: &mut Bencher) {
        b.iter(|| {
            for _ in 0..1000 {
                black_box(black_box(1.0f32).log2());
            }
        });
    }
    
    #[bench]
    fn our_log2(b: &mut Bencher) {
        b.iter(|| {
            for _ in 0..1000 {
                black_box(log2(black_box(1.0)));
            }
        });
    }
    
    #[bench]
    fn std_exp2(b: &mut Bencher) {
        b.iter(|| {
            for _ in 0..1000 {
                black_box(black_box(69.69f32).exp2());
            }
        });
    }
    
    #[bench]
    fn our_exp2(b: &mut Bencher) {
        b.iter(|| {
            for _ in 0..1000 {
                black_box(exp2(black_box(69.69)));
            }
        });
    }

    
    #[bench]
    fn std_pow(b: &mut Bencher) {
        b.iter(|| {
            for _ in 0..1000 {
                black_box(black_box(69.69f32).powf(69.69f32));
            }
        });
    }
    
    #[bench]
    fn our_pow(b: &mut Bencher) {
        b.iter(|| {
            for _ in 0..1000 {
                black_box(exp2(pow(black_box(69.69), black_box(69.69))));
            }
        });
    }
    
    #[bench]
    fn std_sqrt(b: &mut Bencher) {
        b.iter(|| {
            for _ in 0..1000 {
                black_box(black_box(69.69f32).sqrt());
            }
        });
    }
    
    #[bench]
    fn our_sqrt(b: &mut Bencher) {
        b.iter(|| {
            for _ in 0..1000 {
                black_box(sqrt(black_box(69.69)));
            }
        });
    }
}