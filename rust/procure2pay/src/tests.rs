
#[cfg(test)]
mod tests {
    use chrono::NaiveDateTime;
    use std::panic;
    use crate::activities;
    #[cfg(feature = "sequential")]
    use crate::sequential;
    #[cfg(feature = "parallel")]
    use crate::parallel;

    #[cfg(feature = "sequential")]
    fn process_cases(cases: Vec<(String, NaiveDateTime, String)>) -> Vec<(Vec<u8>, usize)>  {
        sequential::process_cases(cases)
    }

    #[cfg(feature = "parallel")]
    fn process_cases(cases: Vec<(String, NaiveDateTime, String)>) -> Vec<(Vec<u8>, usize)>  {
        parallel::process_cases(cases)
    }

    fn parse_date(date_str: &str) -> NaiveDateTime {
        let datetime_str = format!("{} 00:00:00", date_str);
        NaiveDateTime::parse_from_str(&datetime_str, "%Y-%m-%d %H:%M:%S").unwrap()
    }

    fn generate_test_cases_from_variants(variants: Vec<(Vec<u8>, usize)>) -> Vec<(String, NaiveDateTime, String)> {
        let mut cases = Vec::new();
        let mut case_id_counter = 1;

        for (sequence, count) in variants {
            for i in 0..count {
                let current_date = parse_date("2024-08-18") + chrono::Duration::days(i as i64);

                // Create activities for each case
                // For each case, generate activities
                for (i, &activity_num) in sequence.iter().enumerate() {
                    let activity_name = activities::num_to_str(activity_num as u8);
                    // let timestamp = start_date + Duration::days(i as i64);
                    let timestamp = current_date + chrono::Duration::days(i as i64);


                    cases.push((
                        case_id_counter.to_string(),
                        timestamp,
                        activity_name.to_string(),
                    ));
                }
                case_id_counter += 1;
            }
        }

        // Shuffle cases to randomize the order
        // cases.shuffle(&mut rng);
        cases
    }

    #[test]
    fn test_common_set() {
        let cases = vec![
            ("1".to_string(), parse_date("2024-08-17"), activities::num_to_str(0).to_string()),
            ("1".to_string(), parse_date("2024-08-18"), activities::num_to_str(1).to_string()),
            ("2".to_string(), parse_date("2024-08-17"), activities::num_to_str(0).to_string()),
            ("2".to_string(), parse_date("2024-08-18"), activities::num_to_str(1).to_string()),
        ];

        let result = process_cases(cases);
        let expected_variants = vec![(vec![0, 1], 2)];
        assert_eq!(result.len(), 1);
        assert_eq!(result, expected_variants);
    }

    #[test]
    fn test_one_valid_activity() {
        let cases = vec![
            ("1".to_string(), parse_date("2024-08-18"), "Clear open item".to_string()),
        ];

        let result = process_cases(cases);
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_invalid_activity() {
        let cases = vec![
            ("1".to_string(), parse_date("2024-08-18"), "Activity A".to_string()),
        ];

        let result = panic::catch_unwind(|| {
            process_cases(cases);
        });
        assert!(result.is_err());
    }

    #[test]
    fn test_no_input() {
        let cases: Vec<(String, NaiveDateTime, String)> = Vec::new();
        assert_eq!(process_cases(cases), Vec::new());
    }

    #[test]
    fn test_duplicate_activities_in_case() {
        let cases = vec![
            ("001".to_string(), parse_date("2024-08-17"), activities::num_to_str(0).to_string()),
            ("001".to_string(), parse_date("2024-08-18"), activities::num_to_str(0).to_string()),
            ("001".to_string(), parse_date("2024-08-17"), activities::num_to_str(0).to_string()),
            ("001".to_string(), parse_date("2024-08-18"), activities::num_to_str(0).to_string()),
        ];

        let result = process_cases(cases);
        let expected_variants = vec![(vec![0], 1)];
        assert_eq!(result.len(), 1);
        assert_eq!(result, expected_variants);
    }

    #[test]
    fn test_activities_with_the_same_timestamp() {
        let cases = vec![
            ("001".to_string(), parse_date("2024-08-17"), activities::num_to_str(0).to_string()),
            ("001".to_string(), parse_date("2024-08-17"), activities::num_to_str(1).to_string()),
            ("001".to_string(), parse_date("2024-08-17"), activities::num_to_str(0).to_string()),
            ("001".to_string(), parse_date("2024-08-17"), activities::num_to_str(1).to_string()),
        ];

        let result = process_cases(cases);
        let expected_variants = vec![(vec![1, 0], 1)];
        assert_eq!(result.len(), 1);
        assert_eq!(result, expected_variants);
    }

    #[test]
    fn test_more_cases_with_one_variants() {
        let cases = vec![
            ("1".to_string(), parse_date("2024-08-18"), activities::num_to_str(0).to_string()),
            ("2".to_string(), parse_date("2024-08-18"), activities::num_to_str(0).to_string()),
            ("3".to_string(), parse_date("2024-08-18"), activities::num_to_str(0).to_string()),
            ("4".to_string(), parse_date("2024-08-18"), activities::num_to_str(0).to_string()),
        ];

        let result = process_cases(cases);
        let expected_variants = vec![(vec![0], 4)];
        assert_eq!(result.len(), 1);
        assert_eq!(result, expected_variants);
    }

    #[test]
    fn test_long_variant() {
        let count = 20;
        let mut cases = Vec::with_capacity(count);
        for i in 1..=count {
            // Generate a date incrementing by days
            let date = parse_date("2024-08-18") + chrono::Duration::days(i as i64);

            // Assuming `activities::num_to_str` function returns a string representation for activity numbers
            let activity_name = activities::num_to_str(i as u8);

            cases.push(("1".to_string(), date, activity_name.to_string()));
        }
        let result = process_cases(cases);
        let expected_variants = vec![(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20], 1)];
        assert_eq!(result.len(), 1);
        assert_eq!(result, expected_variants);
    }

    #[test]
    fn test_generated_from_expected_variants() {
        let expected_variants = vec![
            (vec![1, 2, 3], 3),
            (vec![1, 2], 2),
            (vec![1], 1),
        ];
        let cases = generate_test_cases_from_variants(expected_variants.clone());
        let result = process_cases(cases);
        assert_eq!(result, expected_variants);
    }
}
