#[macro_use]
extern crate criterion;

use criterion::black_box;
use criterion::Criterion;

mod datasets;

fn encode_array(values: &[u64], result: &mut [u64]) {
    let mut count = 0;
    let mut i = 0;
    while count < values.len() {
        count += simple8b::pack(&values[count..], &mut result[i]).unwrap();
        i += 1;
    }
}

fn benches_simple8b_encode(c: &mut Criterion) {
    c.bench_function("encode_zeros", |b| {
        let values = [0u64; 1024];
        let mut result = [0u64; 128];
        b.iter(|| encode_array(black_box(&values[..]), &mut result));
    });
    c.bench_function("encode_8", |b| {
        let mut result = [0u64; 256];
        b.iter(|| encode_array(black_box(&datasets::VALUES_8[..]), &mut result));
    });
    c.bench_function("encode_16", |b| {
        let mut result = [0u64; 512];
        b.iter(|| encode_array(black_box(&datasets::VALUES_16[..]), &mut result));
    });
}

fn decode_array(input: &[u64], output: &mut [u64]) -> usize {
    let mut count = 0;
    for &v in input {
        count += simple8b::unpack(v, &mut output[count..]);
        if count == output.len() {
            break;
        }
    }
    count
}

fn benches_simple8b_decode(c: &mut Criterion) {
    c.bench_function("decode_zeros", |b| {
        let values = [0u64; 1024];
        let mut encoded = [0u64; 128];
        encode_array(&values[..], &mut encoded);

        let mut decoded = [0u64; 1024];
        b.iter(|| decode_array(black_box(&encoded), &mut decoded));

        assert_eq!(&values[..], &decoded[..]);
    });
    c.bench_function("decode_8", |b| {
        let mut encoded = [0u64; 256];
        encode_array(&datasets::VALUES_8[..], &mut encoded);

        let mut decoded = [0u64; 1024];
        b.iter(|| decode_array(black_box(&encoded), &mut decoded));

        assert_eq!(&datasets::VALUES_8[..], &decoded[..]);
    });
    c.bench_function("decode_16", |b| {
        let mut encoded = [0u64; 512];
        encode_array(&datasets::VALUES_16[..], &mut encoded);

        let mut decoded = [0u64; 1024];
        b.iter(|| decode_array(black_box(&encoded), &mut decoded));

        assert_eq!(&datasets::VALUES_16[..], &decoded[..]);
    });
}

criterion_group!(benches, benches_simple8b_encode, benches_simple8b_decode);
criterion_main!(benches);
