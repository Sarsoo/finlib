use criterion::{
    criterion_group, criterion_main, AxisScale, BatchSize, BenchmarkId, Criterion,
    PlotConfiguration, Throughput,
};
use finlib::portfolio::{Portfolio, PortfolioAsset};
use rand::Rng;

pub fn bench_apply_rates_of_change_values(c: &mut Criterion) {
    let mut group = c.benchmark_group("Portfolio::apply_rates_of_change/values");
    let mut rng = rand::thread_rng();

    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    group.plot_config(plot_config);

    for i in [10, 100, 1000, 10_000, 100_000, 1_000_000].into_iter() {
        let portfolio = Portfolio::from(vec![
            PortfolioAsset::new(
                // 0.1,
                "a".to_string(),
                4.0,
                (0..i).map(|_| rng.gen::<f64>()).collect(),
            ),
            PortfolioAsset::new(
                // 0.1,
                "a".to_string(),
                4.0,
                (0..i).map(|_| rng.gen::<f64>()).collect(),
            ),
            PortfolioAsset::new(
                // 0.1,
                "a".to_string(),
                4.0,
                (0..i).map(|_| rng.gen::<f64>()).collect(),
            ),
            PortfolioAsset::new(
                // 0.1,
                "a".to_string(),
                4.0,
                (0..i).map(|_| rng.gen::<f64>()).collect(),
            ),
            PortfolioAsset::new(
                // 0.1,
                "a".to_string(),
                4.0,
                (0..i).map(|_| rng.gen::<f64>()).collect(),
            ),
            PortfolioAsset::new(
                // 0.1,
                "a".to_string(),
                4.0,
                (0..i).map(|_| rng.gen::<f64>()).collect(),
            ),
            PortfolioAsset::new(
                // 0.1,
                "a".to_string(),
                4.0,
                (0..i).map(|_| rng.gen::<f64>()).collect(),
            ),
            PortfolioAsset::new(
                // 0.1,
                "a".to_string(),
                4.0,
                (0..i).map(|_| rng.gen::<f64>()).collect(),
            ),
            PortfolioAsset::new(
                // 0.1,
                "a".to_string(),
                4.0,
                (0..i).map(|_| rng.gen::<f64>()).collect(),
            ),
            PortfolioAsset::new(
                // 0.1,
                "a".to_string(),
                4.0,
                (0..i).map(|_| rng.gen::<f64>()).collect(),
            ),
        ]);

        group.throughput(Throughput::Elements((i * 10) as u64));
        group.bench_function(BenchmarkId::new("Sequential", i), |b| {
            b.iter_batched(
                || portfolio.clone(),
                |mut p| {
                    p.apply_rates_of_change();
                },
                BatchSize::SmallInput,
            )
        });
        group.bench_function(BenchmarkId::new("Parallel", i), |b| {
            b.iter_batched(
                || portfolio.clone(),
                |mut p| {
                    p.par_apply_rates_of_change();
                },
                BatchSize::SmallInput,
            )
        });
    }
    group.finish();
}

pub fn bench_apply_rates_of_change_assets(c: &mut Criterion) {
    let mut group = c.benchmark_group("Portfolio::apply_rates_of_change/assets");
    let mut rng = rand::thread_rng();

    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    group.plot_config(plot_config);

    for i in [10, 100, 1000, 10_000].into_iter() {
        let portfolio = Portfolio::from(
            (0..i)
                .map(|_| {
                    PortfolioAsset::new(
                        // 0.1,
                        "a".to_string(),
                        4.0,
                        (0..10000).map(|_| rng.gen::<f64>()).collect(),
                    )
                })
                .collect(),
        );

        group.throughput(Throughput::Elements(i as u64));
        group.bench_function(BenchmarkId::new("Sequential", i), |b| {
            b.iter_batched(
                || portfolio.clone(),
                |mut p| {
                    p.apply_rates_of_change();
                },
                BatchSize::SmallInput,
            )
        });
        group.bench_function(BenchmarkId::new("Parallel", i), |b| {
            b.iter_batched(
                || portfolio.clone(),
                |mut p| {
                    p.par_apply_rates_of_change();
                },
                BatchSize::SmallInput,
            )
        });
    }
    group.finish();
}

criterion_group!(
    benches,
    bench_apply_rates_of_change_values,
    bench_apply_rates_of_change_assets
);
criterion_main!(benches);
