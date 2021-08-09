use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use routeros_proto::reply::Reply;

fn parse_reply(input: &[u8]) -> Reply {
    Reply::from_bytes(input)
        .expect("Unexpected parsing error")
        .1
}

fn reply_parsing(c: &mut Criterion) {
    let done_reply = vec![0x05, 0x21, 0x64, 0x6f, 0x6e, 0x65, 0x00];
    let data_reply = vec![
        0x03, 0x21, 0x72, 0x65, 0x09, 0x3d, 0x6e, 0x61, 0x6d, 0x65, 0x3d, 0x66, 0x6f, 0x6f, 0x0e,
        0x3d, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x66, 0x61, 0x63, 0x65, 0x3d, 0x62, 0x61, 0x72, 0x00,
    ];
    let replies = vec![("done_reply", done_reply), ("data_reply", data_reply)];
    let mut bench_group = c.benchmark_group("reply_parser");
    for (name, data) in replies.into_iter() {
        bench_group.throughput(Throughput::Bytes(data.len() as u64));
        bench_group.bench_with_input(
            BenchmarkId::new(name, data.len()),
            data.as_slice(),
            |b, data| {
                b.iter_with_large_drop(|| parse_reply(data));
            },
        );
    }
    bench_group.finish();
}

criterion_group!(benches, reply_parsing);
criterion_main!(benches);
