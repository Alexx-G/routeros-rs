use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use routeros_proto::{core::Decodable, reply::Reply};

fn parse_reply(input: &[u8]) -> Reply {
    Reply::from_bytes_slice(input).expect("Unexpected parsing error")
}

fn reply_parsing(c: &mut Criterion) {
    // ·!done·
    let done_reply = vec![0x05, 0x21, 0x64, 0x6f, 0x6e, 0x65, 0x00];
    // ·!re·=.id=*1 =name=ether1·=default-name=ether1·=type=ether =mtu=1500·=actual-mtu=1500·=l2mtu=1596·=max-l2mtu=2026·=mac-address=48:8F:5A:2F:54:06'=last-link-up-time=aug/07/2021 14:01:13 =link-downs=0·=rx-byte=67894474104·=tx-byte=3275888520·=rx-packet=52108758·=tx-packet=24342800·=tx-queue-drop=246·=fp-rx-byte=67684369188·=fp-tx-byte=3174435713·=fp-rx-packet=52506495·=fp-tx-packet=24342805 =running=true·=disabled=false·=comment=foobar ·
    let data_reply = vec![
        0x03, 0x21, 0x72, 0x65, 0x07, 0x3d, 0x2e, 0x69, 0x64, 0x3d, 0x2a, 0x31, 0x0c, 0x3d, 0x6e,
        0x61, 0x6d, 0x65, 0x3d, 0x65, 0x74, 0x68, 0x65, 0x72, 0x31, 0x14, 0x3d, 0x64, 0x65, 0x66,
        0x61, 0x75, 0x6c, 0x74, 0x2d, 0x6e, 0x61, 0x6d, 0x65, 0x3d, 0x65, 0x74, 0x68, 0x65, 0x72,
        0x31, 0x0b, 0x3d, 0x74, 0x79, 0x70, 0x65, 0x3d, 0x65, 0x74, 0x68, 0x65, 0x72, 0x09, 0x3d,
        0x6d, 0x74, 0x75, 0x3d, 0x31, 0x35, 0x30, 0x30, 0x10, 0x3d, 0x61, 0x63, 0x74, 0x75, 0x61,
        0x6c, 0x2d, 0x6d, 0x74, 0x75, 0x3d, 0x31, 0x35, 0x30, 0x30, 0x0b, 0x3d, 0x6c, 0x32, 0x6d,
        0x74, 0x75, 0x3d, 0x31, 0x35, 0x39, 0x36, 0x0f, 0x3d, 0x6d, 0x61, 0x78, 0x2d, 0x6c, 0x32,
        0x6d, 0x74, 0x75, 0x3d, 0x32, 0x30, 0x32, 0x36, 0x1e, 0x3d, 0x6d, 0x61, 0x63, 0x2d, 0x61,
        0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x3d, 0x34, 0x38, 0x3a, 0x38, 0x46, 0x3a, 0x35, 0x41,
        0x3a, 0x32, 0x46, 0x3a, 0x35, 0x34, 0x3a, 0x30, 0x36, 0x27, 0x3d, 0x6c, 0x61, 0x73, 0x74,
        0x2d, 0x6c, 0x69, 0x6e, 0x6b, 0x2d, 0x75, 0x70, 0x2d, 0x74, 0x69, 0x6d, 0x65, 0x3d, 0x61,
        0x75, 0x67, 0x2f, 0x30, 0x37, 0x2f, 0x32, 0x30, 0x32, 0x31, 0x20, 0x31, 0x34, 0x3a, 0x30,
        0x31, 0x3a, 0x31, 0x33, 0x0d, 0x3d, 0x6c, 0x69, 0x6e, 0x6b, 0x2d, 0x64, 0x6f, 0x77, 0x6e,
        0x73, 0x3d, 0x30, 0x14, 0x3d, 0x72, 0x78, 0x2d, 0x62, 0x79, 0x74, 0x65, 0x3d, 0x36, 0x37,
        0x38, 0x39, 0x34, 0x34, 0x37, 0x34, 0x31, 0x30, 0x34, 0x13, 0x3d, 0x74, 0x78, 0x2d, 0x62,
        0x79, 0x74, 0x65, 0x3d, 0x33, 0x32, 0x37, 0x35, 0x38, 0x38, 0x38, 0x35, 0x32, 0x30, 0x13,
        0x3d, 0x72, 0x78, 0x2d, 0x70, 0x61, 0x63, 0x6b, 0x65, 0x74, 0x3d, 0x35, 0x32, 0x31, 0x30,
        0x38, 0x37, 0x35, 0x38, 0x13, 0x3d, 0x74, 0x78, 0x2d, 0x70, 0x61, 0x63, 0x6b, 0x65, 0x74,
        0x3d, 0x32, 0x34, 0x33, 0x34, 0x32, 0x38, 0x30, 0x30, 0x12, 0x3d, 0x74, 0x78, 0x2d, 0x71,
        0x75, 0x65, 0x75, 0x65, 0x2d, 0x64, 0x72, 0x6f, 0x70, 0x3d, 0x32, 0x34, 0x36, 0x17, 0x3d,
        0x66, 0x70, 0x2d, 0x72, 0x78, 0x2d, 0x62, 0x79, 0x74, 0x65, 0x3d, 0x36, 0x37, 0x36, 0x38,
        0x34, 0x33, 0x36, 0x39, 0x31, 0x38, 0x38, 0x16, 0x3d, 0x66, 0x70, 0x2d, 0x74, 0x78, 0x2d,
        0x62, 0x79, 0x74, 0x65, 0x3d, 0x33, 0x31, 0x37, 0x34, 0x34, 0x33, 0x35, 0x37, 0x31, 0x33,
        0x16, 0x3d, 0x66, 0x70, 0x2d, 0x72, 0x78, 0x2d, 0x70, 0x61, 0x63, 0x6b, 0x65, 0x74, 0x3d,
        0x35, 0x32, 0x35, 0x30, 0x36, 0x34, 0x39, 0x35, 0x16, 0x3d, 0x66, 0x70, 0x2d, 0x74, 0x78,
        0x2d, 0x70, 0x61, 0x63, 0x6b, 0x65, 0x74, 0x3d, 0x32, 0x34, 0x33, 0x34, 0x32, 0x38, 0x30,
        0x35, 0x0d, 0x3d, 0x72, 0x75, 0x6e, 0x6e, 0x69, 0x6e, 0x67, 0x3d, 0x74, 0x72, 0x75, 0x65,
        0x0f, 0x3d, 0x64, 0x69, 0x73, 0x61, 0x62, 0x6c, 0x65, 0x64, 0x3d, 0x66, 0x61, 0x6c, 0x73,
        0x65, 0x0f, 0x3d, 0x63, 0x6f, 0x6d, 0x6d, 0x65, 0x6e, 0x74, 0x3d, 0x66, 0x6f, 0x6f, 0x62,
        0x61, 0x72, 0x00,
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
