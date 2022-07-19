use criterion::{criterion_group, criterion_main, Criterion};
use test_bin::{
    container_with_most_water::*, longest_substring_without_repeating_characters::*,
    palindrome_number::*, partition_labels::*, pascals_triangle::*,
    remove_duplicates_from_sorted_array::*, remove_element::*, roman_to_integer::*,
    string_to_integer::*,
};

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
fn is_palindrome_bench(c: &mut Criterion) {
    c.bench_function("121", |b| b.iter(|| is_palindrome(121)));
    c.bench_function("-121", |b| b.iter(|| is_palindrome(-121)));
    c.bench_function("123454321", |b| b.iter(|| is_palindrome(123454321)));
    c.bench_function("10", |b| b.iter(|| is_palindrome(10)));

    c.bench_function("11", |b| b.iter(|| is_palindrome(11)));
}
fn partition_labels_bench(c: &mut Criterion) {
    c.bench_function("ababcbacadefegdehijhklij", |b| {
        b.iter(|| partition_labels(String::from("ababcbacadefegdehijhklij")))
    });
    c.bench_function("eccbbbbdec", |b| {
        b.iter(|| partition_labels(String::from("eccbbbbdec")))
    });
}
fn container_with_most_water_bench(c: &mut Criterion) {
    c.bench_function("[1,1]", |b| b.iter(|| max_area(vec![1, 1])));
    c.bench_function("[1,8,6,2,5,4,8,3,7]", |b| {
        b.iter(|| max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]))
    });
}
fn roman_to_int_bench(c: &mut Criterion) {
    c.bench_function("III", |b| b.iter(|| roman_to_int(String::from("III"))));
    c.bench_function("LVIII", |b| b.iter(|| roman_to_int(String::from("LVIII"))));
    c.bench_function("MCMXCIV", |b| {
        b.iter(|| roman_to_int(String::from("MCMXCIV")))
    });
}
fn remove_duplicates_from_sorted_array_bench(c: &mut Criterion) {
    c.bench_function("[1,1,2]", |b| {
        b.iter_with_setup(|| vec![1, 1, 2], |mut vec| remove_duplicates(&mut vec))
    });
    c.bench_function("[0,0,1,1,1,2,2,3,3,4]", |b| {
        b.iter_with_setup(
            || vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4],
            |mut vec| remove_duplicates(&mut vec),
        )
    });
    c.bench_function("long", |b| {
        b.iter_with_setup(
            || {
                vec![
                    0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 4, 4, 4, 4,
                    4, 4, 4, 4, 4, 4, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 19, 20, 21,
                    22, 25, 27, 28, 29, 29, 29, 29, 29, 29, 29, 29, 29, 30,
                ]
            },
            |mut vec| remove_duplicates(&mut vec),
        )
    });
}
fn remove_element_bench(c: &mut Criterion) {
    c.bench_function("[3,2,2,3]", |b| {
        b.iter_with_setup(|| vec![3, 2, 2, 3], |mut vec| remove_element(&mut vec, 3))
    });
    c.bench_function("[0,1,2,2,3,0,4,2]", |b| {
        b.iter_with_setup(
            || vec![0, 1, 2, 2, 3, 0, 4, 2],
            |mut vec| remove_element(&mut vec, 2),
        )
    });
    c.bench_function("long", |b| {
        b.iter_with_setup(
            || {
                vec![
                    0, 1, 2, 2, 3, 0, 4, 2, 0, 1, 2, 2, 3, 0, 4, 2, 0, 1, 2, 2, 3, 0, 4, 2, 0, 1,
                    2, 2, 3, 0, 4, 2, 0, 1, 2, 2, 3, 0, 4, 2, 0, 1, 2, 2, 3, 0, 4, 2,
                ]
            },
            |mut vec| remove_element(&mut vec, 2),
        )
    });
}
fn pascals_triangle_bench(c: &mut Criterion) {
    c.bench_function("5", |b| b.iter(|| generate(5)));
    c.bench_function("10", |b| b.iter(|| generate(10)));
    c.bench_function("20", |b| b.iter(|| generate(20)));
}

criterion_group!(benches, pascals_triangle_bench);
// criterion_group!(benches, container_with_most_water_bench, length_of_longest_substring_bench, atoi_bench);
criterion_main!(benches);
