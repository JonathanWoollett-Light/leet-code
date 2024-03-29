use criterion::{criterion_group, criterion_main, Criterion};
use test_bin::{
    best_time_to_buy_and_sell_stock::*, container_with_most_water::*,
    longest_substring_without_repeating_characters::*, palindrome_number::*, partition_labels::*,
    pascals_triangle::*, plus_one::*, remove_duplicates_from_sorted_array::*, remove_element::*,
    roman_to_integer::*, string_to_integer::*,
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
fn plus_one_bench(c: &mut Criterion) {
    c.bench_function("[1,2,3]", |b| {
        b.iter_with_setup(|| vec![1, 2, 3], |vec| plus_one(vec))
    });
    c.bench_function("[4,3,2,1]", |b| {
        b.iter_with_setup(|| vec![4, 3, 2, 1], |vec| plus_one(vec))
    });
    c.bench_function("[9]", |b| {
        b.iter_with_setup(|| vec![9], |vec| plus_one(vec))
    });
    c.bench_function("[1,2,3,1,2,3,etc.]", |b| {
        b.iter_with_setup(
            || {
                vec![
                    1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2,
                    3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1,
                    2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3,
                    1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2,
                    3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1,
                    2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3,
                    1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3,
                ]
            },
            |vec| plus_one(vec),
        )
    });
    c.bench_function("[9,9,9,etc.]", |b| {
        b.iter_with_setup(
            || {
                vec![
                    9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
                    9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
                    9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
                    9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
                    9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
                    9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
                    9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
                    9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
                    9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
                    9, 9, 9, 9, 9, 9,
                ]
            },
            |vec| plus_one(vec),
        )
    });
}
fn max_profit_bench(c: &mut Criterion) {
    c.bench_function("max_profit big", |b| {
        b.iter_with_setup(
            || {
                vec![
                    4, 7, 5, 8, 3, 9, 1, 8, 3, 8, 25, 6234, 234, 3, 6, 77, 86, 345, 578, 346, 5678,
                    23, 12, 648, 67, 36, 8, 34, 342, 0, 56, 123, 4, 578, 980, 345, 69, 23, 4, 7890,
                    5467, 679, 457, 56797, 80, 456, 6780, 457, 2536, 7689, 3425, 7980, 1235, 689,
                    780, 456, 9, 567, 6570, 346, 345,
                ]
            },
            |vec| max_profit(vec),
        )
    });
}

criterion_group!(
    benches,
    length_of_longest_substring_bench,
    atoi_bench,
    is_palindrome_bench,
    partition_labels_bench,
    container_with_most_water_bench,
    roman_to_int_bench,
    remove_duplicates_from_sorted_array_bench,
    remove_element_bench,
    pascals_triangle_bench,
    plus_one_bench,
    max_profit_bench
);
criterion_main!(benches);
