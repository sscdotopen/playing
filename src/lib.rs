extern crate rand;
extern crate simdeez;

use simdeez::Simd;
use simdeez::sse41::Sse41;
use rand::{Rng,SeedableRng,XorShiftRng};

const NUM_ROWS: usize = 1024;
const NUM_COLS: usize = 1024;

pub fn generate_random_matrix() -> Vec<Vec<f32>> {

    let mut rng = XorShiftRng::from_seed([1, 2, 3, 4]);

    (0..NUM_ROWS).map(|_| {
        (0..NUM_COLS).map(|_| rng.next_f32()).collect()
    })
    .collect()
}

pub fn colwise_summation(matrix: &Vec<Vec<f32>>) -> f32 {
    let mut sum: f32 = 0.0;

    for col in 0..NUM_COLS {
        for row in 0..NUM_ROWS {
            sum += matrix[row][col];
        }
    }

    sum
}


pub fn rowwise_summation(matrix: &Vec<Vec<f32>>) -> f32 {
    let mut sum: f32 = 0.0;

    for row in 0..NUM_ROWS {
        for col in 0..NUM_COLS {
            sum += matrix[row][col];
        }
    }

    sum
}

pub unsafe fn rowwise_simd_summation_sse41(matrix: &Vec<Vec<f32>>) -> f32 {

    let zeros: [f32; Sse41::VF32_WIDTH] = [0.0; Sse41::VF32_WIDTH];

    let mut sums = Sse41::loadu_ps(&zeros[0]);

    for row in 0..NUM_ROWS {

        let mut col: usize = 0;

        while col < NUM_COLS {
            let partial_row = Sse41::loadu_ps(&matrix[row][col]);
            sums += partial_row;
            col += Sse41::VF32_WIDTH;
        }
    }

    let mut result: Vec<f32> = Vec::with_capacity(Sse41::VF32_WIDTH);
    result.set_len(Sse41::VF32_WIDTH);
    Sse41::storeu_ps(&mut result[0], sums);

    result.into_iter().sum()
}