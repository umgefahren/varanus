use criterion::{Criterion, criterion_group, criterion_main};
use fast_version::version::{VersionNumber, serial_compare};
use rand::{distributions::uniform::SampleUniform, thread_rng, Rng};



fn generate_version_part<N: VersionNumber + SampleUniform>() -> N {
    let min = N::min();
    let max = N::max();
    
    let mut rng = thread_rng();

    let ret_num: N = rng.gen_range(min..=max);

    ret_num
}

fn perform_compare_serial<N: VersionNumber + SampleUniform>() {
    let major = generate_version_part::<N>();
    let minor = generate_version_part::<N>();
    let patch = generate_version_part::<N>();

    serial_compare(major, minor, patch);
}

#[cfg(nightly)]
fn perform_compare_simd<N: VersionNumber + SampleUniform>() {
    use fast_version::simd::fast_compare_simd;

    let major = generate_version_part::<N>();
    let minor = generate_version_part::<N>();
    let patch = generate_version_part::<N>();

    fast_compare_simd(major, minor, patch);
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut u8_group = c.benchmark_group("Compare u8");
    u8_group.bench_function("non-simd", |b| b.iter(|| perform_compare_serial::<u8>()));

    #[cfg(nightly)]
    u8_group.bench_function("simd", |b| b.iter(|| perform_compare_simd::<u8>()));

    u8_group.finish();

    let mut u16_group = c.benchmark_group("Compare u16");
    u16_group.bench_function("non-simd", |b| b.iter(|| perform_compare_serial::<u16>()));

    #[cfg(nightly)]
    u16_group.bench_function("simd", |b| b.iter(|| perform_compare_simd::<u16>()));

    u16_group.finish();


    let mut u32_group = c.benchmark_group("Compare u32");
    u32_group.bench_function("non-simd", |b| b.iter(|| perform_compare_serial::<u32>()));

    #[cfg(nightly)]
    u32_group.bench_function("simd", |b| b.iter(|| perform_compare_simd::<u32>()));

    u32_group.finish();


    let mut u64_group = c.benchmark_group("Compare u64");
    u64_group.bench_function("non-simd", |b| b.iter(|| perform_compare_serial::<u64>()));

    #[cfg(nightly)]
    u64_group.bench_function("simd", |b| b.iter(|| perform_compare_simd::<u64>()));

    u64_group.finish();


    let mut usize_group = c.benchmark_group("Compare usize");
    usize_group.bench_function("non-simd", |b| b.iter(|| perform_compare_serial::<usize>()));

    #[cfg(nightly)]
    usize_group.bench_function("simd", |b| b.iter(|| perform_compare_simd::<usize>()));

    usize_group.finish();

    let mut i8_group = c.benchmark_group("Compare i8");
    i8_group.bench_function("non-simd", |b| b.iter(|| perform_compare_serial::<i8>()));

    #[cfg(nightly)]
    i8_group.bench_function("simd", |b| b.iter(|| perform_compare_simd::<i8>()));

    i8_group.finish();

    let mut i16_group = c.benchmark_group("Compare i16");
    i16_group.bench_function("non-simd", |b| b.iter(|| perform_compare_serial::<i16>()));

    #[cfg(nightly)]
    i16_group.bench_function("simd", |b| b.iter(|| perform_compare_simd::<i16>()));

    i16_group.finish();


    let mut i32_group = c.benchmark_group("Compare i32");
    i32_group.bench_function("non-simd", |b| b.iter(|| perform_compare_serial::<i32>()));

    #[cfg(nightly)]
    i32_group.bench_function("simd", |b| b.iter(|| perform_compare_simd::<i32>()));

    i32_group.finish();


    let mut i64_group = c.benchmark_group("Compare i64");
    i64_group.bench_function("non-simd", |b| b.iter(|| perform_compare_serial::<i64>()));

    #[cfg(nightly)]
    i64_group.bench_function("simd", |b| b.iter(|| perform_compare_simd::<i64>()));

    i64_group.finish();


    let mut isize_group = c.benchmark_group("Compare isize");
    isize_group.bench_function("non-simd", |b| b.iter(|| perform_compare_serial::<isize>()));

    #[cfg(nightly)]
    isize_group.bench_function("simd", |b| b.iter(|| perform_compare_simd::<isize>()));

    isize_group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
