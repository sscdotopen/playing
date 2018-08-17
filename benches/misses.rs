#[macro_use]
extern crate bencher;
extern crate playing;

use bencher::Bencher;

benchmark_group!(benches,
    bench_colwise_summation,
    bench_rowwise_summation,
    bench_rowwise_simd_summation_sse41);

benchmark_main!(benches);


const NUM_REPETITIONS: usize = 3;

fn bench_colwise_summation(bench: &mut Bencher) {

    let matrix = playing::generate_random_matrix();

    bench.iter(|| {
        for _ in 0..NUM_REPETITIONS {
            bencher::black_box(playing::colwise_summation(&matrix));
        }
    })
}


fn bench_rowwise_summation(bench: &mut Bencher) {

    let matrix = playing::generate_random_matrix();

    bench.iter(|| {
        for _ in 0..NUM_REPETITIONS {
            bencher::black_box(playing::rowwise_summation(&matrix));
        }
    })
}


fn bench_rowwise_simd_summation_sse41(bench: &mut Bencher) {

    let matrix = playing::generate_random_matrix();

    bench.iter(|| {
        for _ in 0..NUM_REPETITIONS {
            unsafe {
                bencher::black_box(playing::rowwise_simd_summation_sse41(&matrix));
            }
        }
    })
}

