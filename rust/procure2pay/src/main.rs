//
//  SPDX-License-Identifier: Apache-2.0
//
//  main: The main entry point for the application:
//          * handles CLI arguments.
//          * calls the extract function to get top-variants
//          * prints out the json
//
mod csv_parser;
mod sequential;
mod activities;
mod parallel;
mod tests;

use std::time::Instant;
use serde_json::json;
use clap::{Arg, Command};

fn main() {
    let matches = Command::new("ProcureToPay")
        .version("0.1.0")
        .about("Crunches cases from Procure to Pay to find most frequent variants of activities")
        .arg(
            Arg::new("file")
                .help("Path to the CSV file with cases")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("crunch-activities")
                .long("crunch-activities")
                .help("Goes over all activities and generates str2num conversions")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("with-names")
                .long("with-names")
                .help("Uses activity names (instead of numbers) in the resultant variant list")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("gold")
                .long("gold")
                .help("Uses original golden processing engine (not optimized)")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("no-time-eval")
                .long("no-time-eval")
                .help("Skips time/performance evaluation (used for integration tests)")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("decimate")
                .long("decimate")
                .help("Removes every 10th (arg) element of input vector (used for testing)")
                .value_name("FACTOR")
                .value_parser(clap::value_parser!(usize)),
        )
        .get_matches();
    let file_path = matches.get_one::<String>("file").expect("File name is required");

    // Read and parse the CSV
    let mut cases = csv_parser::parse_csv(file_path).expect("Failed to parse CSV");

    if matches.get_flag("crunch-activities") {
        activities::crunch_activities(cases);
        std::process::exit(0);
    }
    // Decimate input data if requested (only used for integration tests)
    if let Some(factor) = matches.get_one::<usize>("decimate") {
        if !matches.get_flag("no-time-eval") {
            println!("Decimate the input data by factor: {}", factor);
        }
        cases = decimate_vec(cases, *factor);
    }
    let begin = Instant::now();
    // Add the call to your solution here

    // Run the solution
    let top_variants;
    if matches.get_flag("gold") {   // Uses the golden sample
        top_variants = sequential::process_cases(cases);
    } else {
        top_variants = parallel::process_cases(cases);
    }

    let top_10 = top_variants.iter().take(10).collect::<Vec<_>>();

    // Prepare JSON output
    let mut json_output = json!(top_10);

    if matches.get_flag("with-names") {
        let top_10 = convert_variants_to_strings(top_10.clone());
        json_output = json!(top_10);
    }

    let end = Instant::now();

    let duration = end.duration_since(begin);
    if !matches.get_flag("no-time-eval") {
        println!("Duration: {} milliseconds", duration.as_millis());
    }
    println!("{}", json_output.to_string());
}

fn convert_variants_to_strings(variants: Vec<&(Vec<u8>, usize)>) -> Vec<(Vec<String>, usize)> {
    variants
        .into_iter()
        .map(|(activity_numbers, count)| {
            let activity_names = activity_numbers
                .iter()
                .map(|&num| activities::num_to_str(num).to_string()) // Convert &str to String
                .collect::<Vec<String>>();
            (activity_names, *count)
        })
        .collect()
}

fn decimate_vec<T>(vec: Vec<T>, n: usize) -> Vec<T> {
    if n == 0 {
        return vec;
    }
    vec.into_iter()
        .enumerate()
        .filter_map(|(index, value)| {
            if (index + 1) % n != 0 { // Keep elements that are not every n-th
                Some(value)
            } else {
                None
            }
        })
        .collect()                  // Collect filtered elements into a new Vec
}
