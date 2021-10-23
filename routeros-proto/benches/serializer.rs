use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use routeros_proto::{command::{Command, CommandBuilder, CommandWord}, core::Encodable};

fn command_to_bytes(command: &Command) -> Vec<u8> {
    command.to_bytes_vec()
}

fn command_serialization(c: &mut Criterion) {
    let login_cmd = CommandBuilder::new(CommandWord::Login)
        .attribute("name", "foo")
        .build();
    let print_cmd = CommandBuilder::new_raw("/interface/print")
        .attribute("type", "ether")
        .attribute("type", "vlan")
        .query("?mtu=1500")
        .query("?#|!")
        .tag(Some("foobar"))
        .build();

    let mut group = c.benchmark_group("command_to_bytes");
    for (name, command) in [("login", login_cmd), ("print", print_cmd)].iter() {
        group.throughput(Throughput::Elements(1));
        group.bench_with_input(
            BenchmarkId::new("to_bytes_vec", name),
            command,
            |b, command| {
                b.iter_with_large_drop(|| command_to_bytes(command));
            },
        );
    }
    group.finish();
}

criterion_group!(benches, command_serialization);
criterion_main!(benches);
