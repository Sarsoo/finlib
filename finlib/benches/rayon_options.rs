use criterion::{
    criterion_group, criterion_main, AxisScale, BatchSize, BenchmarkId, Criterion,
    PlotConfiguration, Throughput,
};
use finlib::derivatives::options::blackscholes::option_surface::OptionSurfaceParameters;

pub fn bench_generate_options(c: &mut Criterion) {
    let mut group = c.benchmark_group("Options::generate_options");

    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    group.plot_config(plot_config);

    for i in [1, 10, 100, 1000].into_iter() {
        let surface = OptionSurfaceParameters::from(
            0..10,
            (100., 200.),
            0..i,
            (100., 200.),
            0..5,
            (0.25, 0.50),
            0..10,
            (0.05, 0.08),
            0..1,
            (0.01, 0.02),
            0..10,
            (30. / 365.25, 30. / 365.25),
        );

        let variables = surface.walk().unwrap();

        group.throughput(Throughput::Elements(variables.len() as u64));
        group.bench_function(BenchmarkId::new("Sequential", variables.len()), |b| {
            b.iter_batched(
                || variables.clone(),
                |mut p| {
                    p.generate();
                },
                BatchSize::SmallInput,
            )
        });
        group.bench_function(BenchmarkId::new("Parallel", variables.len()), |b| {
            b.iter_batched(
                || variables.clone(),
                |mut p| {
                    p.par_generate();
                },
                BatchSize::SmallInput,
            )
        });
    }
    group.finish();
}

criterion_group!(benches, bench_generate_options);
criterion_main!(benches);
