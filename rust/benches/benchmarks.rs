use criterion::{criterion_group, criterion_main, Criterion};
use test_bin::{longest_substring_without_repeating_characters::*, string_to_integer::*};

fn length_of_longest_substring_bench(c: &mut Criterion) {
    c.bench_function("abba", |b| {
        b.iter(|| length_of_longest_substring(String::from("abba")))
    });
    c.bench_function("abcabcbb", |b| {
        b.iter(|| length_of_longest_substring(String::from("abcabcbb")))
    });
    c.bench_function("bbbbb", |b| {
        b.iter(|| length_of_longest_substring(String::from("bbbbb")))
    });
    c.bench_function("pwwkew", |b| {
        b.iter(|| length_of_longest_substring(String::from("pwwkew")))
    });
    c.bench_function("abcdef", |b| {
        b.iter(|| length_of_longest_substring(String::from("abcdef")))
    });
}
fn atoi_bench(c: &mut Criterion) {
    c.bench_function("42", |b| b.iter(|| my_atoi(String::from("42"))));
    c.bench_function("  000123asd2", |b| {
        b.iter(|| my_atoi(String::from("  000123asd2")))
    });
    c.bench_function("   -42", |b| b.iter(|| my_atoi(String::from("   -42"))));
    c.bench_function("words and 987", |b| {
        b.iter(|| my_atoi(String::from("words and 987")))
    });
    c.bench_function("-91283472332", |b| {
        b.iter(|| my_atoi(String::from("-91283472332")))
    });
    c.bench_function("+-12", |b| b.iter(|| my_atoi(String::from("+-12"))));
}

criterion_group!(benches, length_of_longest_substring_bench, atoi_bench);
criterion_main!(benches);
