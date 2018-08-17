extern crate playing;

fn main() {

    let matrix = playing::generate_random_matrix();

    let sum: f32 = playing::colwise_summation(&matrix);

    println!("{}", sum);
}