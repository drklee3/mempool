use criterion::{black_box, criterion_group, criterion_main, Criterion};
use mempool::model::Mempool;
use rand::Rng;

const POOL_SIZE: usize = 5000;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("mempool 5000 sequential", |b| b.iter(|| {
        let mut pool: Mempool<u64, ()> = Mempool::new_with_capacity(POOL_SIZE);

        for x in 0..10000 {
            pool.insert(black_box(x), ());
        }
    }
    ));
    c.bench_function("mempool 5000 random", |b| b.iter(|| {
        let mut pool: Mempool<u64, ()> = Mempool::new_with_capacity(POOL_SIZE);
        let mut rng = rand::thread_rng();

        for _ in 0..10000 {
            pool.insert(black_box(rng.gen()), ());
        }
    }
    ));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);