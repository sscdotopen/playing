extern crate playing;

fn main() {

    let matrix = playing::generate_random_matrix();

    let sum: f32 = unsafe { playing::rowwise_simd_summation_sse41(&matrix) };

    println!("{}", sum);
}