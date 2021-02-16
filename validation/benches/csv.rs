use criterion::{criterion_group, criterion_main, Criterion};
use std::path::PathBuf;
use validation::validate_data;

fn test_file(subpath: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("resources")
        .join(subpath)
}

fn can_handle_medium_dataset() -> Result<(), Box<dyn std::error::Error>> {
    validate_data(test_file("test_csv_100mb.csv"))?;
    Ok(())
}

fn benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Medium dataset group");
    
    group.sample_size(10);
    group.bench_function("can_handle_medium_dataset", |b| b.iter(|| can_handle_medium_dataset()));
    group.finish();
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
