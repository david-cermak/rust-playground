//
//  SPDX-License-Identifier: Apache-2.0
//
//  sequential: The naive solution (sequential, single-threaded solution).
//
use crate::activities;

use chrono::NaiveDateTime;
use std::collections::{HashMap};

pub fn process_cases(cases: Vec<(String, NaiveDateTime, String)>) -> Vec<(Vec<u8>, usize)> {

    // Creates a map on case_id, with list of all activities (with timestamps)
    let mut case_activities: HashMap<String, Vec<(NaiveDateTime, String)>> = HashMap::new();
    for (case_id, timestamp, activity_name) in cases {
        case_activities.entry(case_id)
            .or_insert_with(Vec::new)
            .push((timestamp, activity_name));
    }

    // Now we create variants for each case and count them
    let mut variant_count: HashMap<Vec<u8>, usize> = HashMap::new();

    for activities in case_activities.values_mut() {
        // Need to sort the activities chronologically
        // ...and then by activity name (note: this is a "naive" approach, and if we see various
        // activities with the same timestamp, putting them in alphabetic order solve potential
        // issues; more rigorous approach would mean to go over all these auto-transitions and
        // take them into consideration)
        activities.sort_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1)));

        // De-duplicate activities (again probably due to auto-transitions), so we can
        // simply count the unique variants
        let mut deduped_activities: Vec<u8> = Vec::new();
        for (_, activity_name) in activities {
            let activity_number = activities::str_to_num(&activity_name);
            // Deduplicate the activity sequence
            if deduped_activities.last().map(|&last| last != activity_number).unwrap_or(true) {
                deduped_activities.push(activity_number);
            }
        }

        // Counts the unique activity sequence (variant)
        *variant_count.entry(deduped_activities).or_insert(0) += 1;
    }

    // Sorts the variants by their count to get the "top" variants (note: don't need to sort the
    // entire collection if we're interested in top 10 variants only)
    let mut sorted_variants: Vec<_> = variant_count.into_iter().collect();
    sorted_variants.sort_by(|a, b| b.1.cmp(&a.1));

    sorted_variants
}
