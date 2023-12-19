use clap::{command, value_parser, Arg, ArgMatches, Command};
use rand::prelude::*;
use std::fs;
use std::path;

const INPUT_VAR_NAME: &str = "input";
const OUTPUT_VAR_NAME: &str = "output";
const ALGORITHM_VAR_NAME: &str = "algorithm";
const UNIQUE_VAR_NAME: &str = "unique";
const CASE_INSENSITIVE: &str = "case_insensitive";
const MERGESORT: &str = "mergesort";
const QUICKSORT: &str = "quicksort";
const HEAPSORT: &str = "heapsort";
const RADIXSORT: &str = "radixsort";
const COUNTINGSORT: &str = "countingsort";
const RANDOMSORT: &str = "randomsort";
const ALGORITHMS: [&str; 3] = [MERGESORT, QUICKSORT, RANDOMSORT];
const SPECIAL_CHARACTERS: [char; 10] = ['(', ')', ',', '\"', '.', ';', ':', '\'', '!', '?'];

// Coding challenge: https://codingchallenges.fyi/challenges/challenge-sort/
fn main() {
    let args: ArgMatches = build_args().get_matches();

    let mut preprocessed_args: PreProcessedArgs = preprocess_args(args);

    // dbg!(&preprocessed_args.input_text);
    println!("Output path: {}", preprocessed_args.output_path.display());
    println!("Algorithm: {}", preprocessed_args.algorithm);
    println!("Case-insensitive: {}", preprocessed_args.case_insensitive);
    println!("Unique: {}", preprocessed_args.unique);

    let mut sort_result: Vec<String> = match preprocessed_args.algorithm.as_str() {
        MERGESORT => merge_sort(&mut preprocessed_args.input_text),
        QUICKSORT => quick_sort(preprocessed_args.input_text),
        HEAPSORT => heap_sort(),
        RADIXSORT => radix_sort(),
        COUNTINGSORT => counting_sort(),
        RANDOMSORT => random_sort(preprocessed_args.input_text),
        _ => panic!("Algorithm selection failed, which should not be possible."),
    };

    if preprocessed_args.unique {
        sort_result.dedup();
    }

    fs::write(preprocessed_args.output_path, sort_result.join("\n"))
        .expect("Writing to output file did not work.");
}

/// Builds the arguments and their configurations as expected and accepted by the application.
///
/// * Return: a Comment object that holds the argument configuration of the application.
fn build_args() -> Command {
    command!()
    .name(env!("CARGO_PKG_NAME"))
    .about(env!("CARGO_PKG_DESCRIPTION"))
    .author(env!("CARGO_PKG_AUTHORS"))
    .version(env!("CARGO_PKG_VERSION"))
    .arg(
        Arg::new(INPUT_VAR_NAME)
            .help("Path to the file containing words to be sorted")
            .required(true)
            .value_parser(value_parser!(path::PathBuf))
    )
    .arg(
        Arg::new(OUTPUT_VAR_NAME)
            .help("Path to the file to write sorted output to")
            .required(true)
            .value_parser(value_parser!(path::PathBuf))
    )
    .arg(
        Arg::new(ALGORITHM_VAR_NAME)
            .short('a')
            .long(ALGORITHM_VAR_NAME)
            .help("Sorting algorithm to use")
            .value_parser(ALGORITHMS)
            .default_value(MERGESORT)
    )
    .arg(
        Arg::new(UNIQUE_VAR_NAME)
            .short('u')
            .long(UNIQUE_VAR_NAME)
            .help("Keep only unique words in output")
            .num_args(0)
    )
    .arg(
        Arg::new(CASE_INSENSITIVE)
            .short('c')
            .long(CASE_INSENSITIVE)
            .help("Sort words case-insensitively (converts all words to lower-case)")
            .num_args(0)
    )
}

/// Holds the preprocessing result of the arguments given to the application (see preprocess_args(ArgMatches)).
struct PreProcessedArgs {
    input_text: Vec<String>,
    output_path: path::PathBuf,
    algorithm: String,
    case_insensitive: bool,
    unique: bool,
}

/// Constructor of PreProcessedArgs.
impl PreProcessedArgs {
    fn new(
        input_text: Vec<String>,
        output_path: path::PathBuf,
        algorithm: String,
        case_insensitive: bool,
        unique: bool,
    ) -> Self {
        Self {
            input_text,
            output_path,
            algorithm,
            case_insensitive,
            unique,
        }
    }
}

/// Preprocesses the arguments given to the application: extracting text from input file, splitting text to words, applying case sensitivity rule to words (if required).
///
/// Return: A PreProcessedArgs object that holds the proprocessing results of the arguments.
fn preprocess_args(args: ArgMatches) -> PreProcessedArgs {
    let input_path: &path::PathBuf = args.get_one::<path::PathBuf>(INPUT_VAR_NAME).unwrap();
    let mut input_words: Vec<String> = fs::read_to_string(input_path)
        .unwrap()
        .replace(&SPECIAL_CHARACTERS[..], "")
        .split_whitespace()
        .map(str::to_string)
        .collect();

    let output_path: path::PathBuf = args
        .get_one::<path::PathBuf>(OUTPUT_VAR_NAME)
        .unwrap()
        .to_owned();

    let algorithm: String = args
        .get_one::<String>(ALGORITHM_VAR_NAME)
        .unwrap()
        .to_string();

    let unique: bool = *(args.get_one::<bool>(UNIQUE_VAR_NAME).unwrap());

    let case_insensitive: bool = *(args.get_one::<bool>(CASE_INSENSITIVE).unwrap());
    if case_insensitive {
        for word in input_words.iter_mut() {
            *word = word.to_lowercase();
        }
    }

    return PreProcessedArgs::new(
        input_words,
        output_path,
        algorithm,
        case_insensitive,
        unique,
    );
}

/// Merge sort algorithm for data types that satisfy the PartialOrd trait.
/// Adapted from https://applied-math-coding.medium.com/implementing-standard-algorithms-in-rust-merge-sort-quick-sort-bb4cfc640abc
///
/// * words: a vector containing the words to be sorted
/// * Return: a vector containing the given words in sorted order
fn merge_sort<T: PartialOrd>(words: &mut Vec<T>) -> Vec<T> {
    if words.len() == 1 {
        return vec![words.pop().unwrap()];
    }
    let n: usize = f64::ceil(words.len() as f64 / 2.0) as usize;
    let mut left: Vec<T> = merge_sort(&mut words.drain(0..n).collect::<Vec<T>>());
    let mut right: Vec<T> = merge_sort(&mut words.drain(..).collect::<Vec<T>>());
    let mut merged: Vec<T> = vec![];
    while left.len() > 0 && right.len() > 0 {
        if left[0] <= right[0] {
            merged.push(left.remove(0));
        } else {
            merged.push(right.remove(0));
        }
    }
    merged.append(if left.len() > 0 {
        &mut left
    } else {
        &mut right
    });
    return merged;
}

/// Quicksort algorithm for data types that satisfy the PartialOrd trait.
/// Adapted from https://preciselab.io/quicksort-implementation-in-rust-typescript-and-go/
///
/// * words: a vector containing the words to be sorted
/// * Return: a vector containing the given words in sorted order
fn quick_sort<T: PartialOrd>(mut words: Vec<T>) -> Vec<T> {
    if words.len() <= 1 {
        return words;
    }

    let pivot: T = words.remove(0);
    let mut left: Vec<T> = vec![];
    let mut right: Vec<T> = vec![];

    for item in words {
        if item <= pivot {
            left.push(item);
        } else {
            right.push(item);
        }
    }

    let mut sorted_left: Vec<T> = quick_sort(left);
    let mut sorted_right: Vec<T> = quick_sort(right);

    sorted_left.push(pivot);
    sorted_left.append(&mut sorted_right);

    sorted_left
}

fn heap_sort<T: PartialOrd>() -> Vec<T> {
    todo!();
}

fn radix_sort<T: PartialOrd>() -> Vec<T> {
    todo!();
}

fn counting_sort<T: PartialOrd>() -> Vec<T> {
    todo!();
}

/// Returns a random permutation of the given words (i.e., words non-deterministically).
///
/// * words: a vector containing the words to be permuted
/// * Return: a vector containing a random permutation of the given words
fn random_sort<T: PartialOrd>(mut words: Vec<T>) -> Vec<T> {
    words.shuffle(&mut thread_rng());
    words
}
