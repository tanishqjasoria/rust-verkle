use criterion::criterion_main;

mod benchmarks;
criterion_main! {
    benchmarks::insert::benches,
}

// #[cfg(not(feature = "async_futures"))]
// pub mod async_measurement_overhead {
//     use criterion::{criterion_group, Criterion};
//     fn some_benchmark(_c: &mut Criterion) {}

//     criterion_group!(benches, some_benchmark);
// }