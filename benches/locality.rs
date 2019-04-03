#![feature(test)]

extern crate free_list;
extern crate test;
extern crate rand;

use test::Bencher;
use free_list::FreeList;
use rand::{Rng, XorShiftRng};
use rand::distributions::{IndependentSample, Range};

const N: usize = 20000;
const NON_CONTIGUOUS_BLOCK_SIZE: usize = N;
const LARGE_INTERVAL: usize = N / 40;
const SMALL_INTERVAL: usize = 8;
const NO_INTERVAL: usize = 1;

fn make_block_non_contiguous_and_allocate(interval: usize) {
    let mut pool = FreeList::new(N);
    
    for i in 0..NON_CONTIGUOUS_BLOCK_SIZE {
        pool.allocate(i);
    }
    
    // Free all, but in intervals
    for j in 0..interval {
        for i in 0..NON_CONTIGUOUS_BLOCK_SIZE / interval {
            pool.free(interval * i + j);
        }
    }
    
    for i in 0..(N) {
        pool.allocate(i);
    }
}

#[bench]
fn non_contiguous_allocate_large_interval(b: &mut Bencher) {
    b.iter(|| {
        make_block_non_contiguous_and_allocate(LARGE_INTERVAL);
    });
}

#[bench]
fn non_contiguous_allocate_small_interval(b: &mut Bencher) {
    b.iter(|| {
        make_block_non_contiguous_and_allocate(SMALL_INTERVAL);
    });
}

#[bench]
fn non_contiguous_allocate_random_interval(b: &mut Bencher) {
    b.iter(|| {
        let mut pool = FreeList::new(N);
        
        for i in 0..NON_CONTIGUOUS_BLOCK_SIZE {
            pool.allocate(i);
        }
        
        let range = Range::new(0, NON_CONTIGUOUS_BLOCK_SIZE);
        let mut rng = XorShiftRng::new_unseeded();
        
        // Free all, but in intervals
        for i in 0..NON_CONTIGUOUS_BLOCK_SIZE {
            pool.free(range.ind_sample(&mut rng));
        }
        
        for i in 0..(N) {
            pool.allocate(i);
        }
    });
}

#[bench]
fn contiguous_allocate(b: &mut Bencher) {
    b.iter(|| {
        make_block_non_contiguous_and_allocate(NO_INTERVAL);
    });
}
