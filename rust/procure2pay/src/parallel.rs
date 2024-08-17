//
//  SPDX-License-Identifier: Apache-2.0
//
//  parallel: Implementation of parallel solution
//          * prepares for parallel analysis (sort and split)
//          * counts unique variant in each worker
//          * collects and merges the results
//
use chrono::NaiveDateTime;
use rayon::prelude::*;
use std::collections::HashMap;
use crate::activities;

pub fn process_cases(raw_cases: Vec<(String, NaiveDateTime, String)>) -> Vec<(Vec<u8>, usize)>  {
    let mut cases = raw_cases;
    // Handles special cases
    if cases.len() == 0 {
        return Vec::new();
    }

    // Needs to sort by
    // * case_id (so we could split the work)
    // * by timestamp so the subsequences are already ready for composing variants
    // * by activity too (see the explanation in sequential.rs:26-29) due to duplications/auto-transitions
    cases.par_sort_unstable_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1)).then_with(|| a.2.cmp(&b.2)));

    let mut num_threads = rayon::current_num_threads().max(1);
    // Considers some "sane" value to split the work between workers
    if cases.len() < 256 {
        num_threads = 1;
    }

    // Splits the work per thead:
    // * finds split points where case_id "just alters"
    // * this way we process each subset separately with no overlapping case_id (so merging is trivial, and cheap)
    let chunk_size = cases.len() / num_threads;
    let mut splits = Vec::with_capacity(num_threads + 1);

    for i in 0..num_threads {
        let mut split_point = i * chunk_size;
        if split_point > 0 {
            let current_case_id = &cases[split_point].0;
            let prev_case_id = &cases[split_point - 1].0;

            while split_point < cases.len() && &cases[split_point].0 == prev_case_id {
                split_point += 1;
            }

            while split_point > 0 && &cases[split_point].0 == current_case_id {
                split_point -= 1;
            }
        }
        splits.push(split_point);
    }
    splits.push(cases.len());

    // Processes each chunk in parallel
    let partial_variants: Vec<HashMap<Vec<u8>, usize>> = splits
        .windows(2)
        .collect::<Vec<_>>()
        .par_iter()
        .map(|window| {
            let chunk = &cases[window[0]..window[1]];
            let mut variants: HashMap<Vec<u8>, usize> = HashMap::new();
            let mut current_case_id = &chunk[0].0;
            let mut current_variant: Vec<u8> = Vec::new();

            for (case_id, _, activity_name) in chunk {
                if case_id != current_case_id {     // finding next case_id area
                    if !current_variant.is_empty() {
                        *variants.entry(current_variant.clone()).or_insert(0) += 1;
                    }
                    current_case_id = case_id;
                    current_variant.clear();
                }
                if current_variant.last() != Some(&activities::str_to_num(activity_name)) {
                    current_variant.push(activities::str_to_num(activity_name));
                }
            }

            // and counts this variant
            if !current_variant.is_empty() {
                *variants.entry(current_variant).or_insert(0) += 1;
            }

            variants
        })
        .collect();

    // Combines the results (a bit expensive as the variants overlap)
    let mut final_variants: HashMap<Vec<u8>, usize> = HashMap::new();
    for partial in partial_variants {
        for (variant, count) in partial {
            *final_variants.entry(variant).or_insert(0) += count;
        }
    }

    // Collect and sort the variants by their count in descending order
    let mut sorted_variants: Vec<_> = final_variants.into_iter().collect();
    sorted_variants.sort_by(|a, b| b.1.cmp(&a.1));

    sorted_variants
}
