use criterion::{black_box, criterion_group, criterion_main, Criterion};
use psutil::process::{Process, ProcessCpuTimes, MemoryInfo};
use sagent::metrics::{ProcessMetrics, SystemMetrics};

fn process_metrics_cpu_percent_benchmark(c: &mut Criterion) {
    let pid = std::process::id() as u32;
    let mut process_metrics = ProcessMetrics::new(pid).unwrap();
    c.bench_function("ProcessMetrics cpu_percent", |b| b.iter(|| process_metrics.cpu_percent()));
}

fn process_metrics_physical_memory_benchmark(c: &mut Criterion) {
    let pid = std::process::id() as u32;
    let process_metrics = ProcessMetrics::new(pid).unwrap();
    c.bench_function("ProcessMetrics physical_memory", |b| b.iter(|| process_metrics.physical_memory()));
}

criterion_group!(
    name = process_metrics_benchmarks;
    config = Criterion::default().sample_size(10);
    targets =
        process_metrics_cpu_percent_benchmark,
        process_metrics_physical_memory_benchmark
);

criterion_main!(process_metrics_benchmarks);
