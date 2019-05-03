#[macro_use]
extern crate criterion;

use criterion::Criterion;
//use criterion::black_box;

//fn a(data: &[f64]) -> f64 {
//    let mut a = 0.0;
//    for i in data {
//        let b: f64 = data.len() as f64 * i;
//        a += b;
//    }
//    a
//}
//
//fn criterion_benchmark(c: &mut Criterion) {
//    let list = [1,2,3,4,5,6,7,8,9];
//    c.bench_function("fib 20", |b| b.iter(|| a(&list)));
//}
//
//criterion_group!(benches, criterion_benchmark);
//criterion_main!(benches);