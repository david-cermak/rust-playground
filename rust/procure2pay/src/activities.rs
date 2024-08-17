//
//  SPDX-License-Identifier: Apache-2.0
//
//  activities: Utility to convert activity name to enum/number.
//  warning: This is module is generated, use `procure2pay ... --crunch-activities` to regenerate
//
use chrono::NaiveDateTime;
use std::collections::HashMap;

pub fn crunch_activities(cases: Vec<(String, NaiveDateTime, String)>) {
    let activities = preprocess_activities(cases);
    generate_function_prototypes(activities.0, activities.1);
}

pub fn str_to_num(activity: &str) -> u8 {
    match activity {
        "Reduce purchase order item quantity" => 10,
        "Change cash discount days 2" => 24,
        "Change purchase order (currency)" => 27,
        "Unblock purchase order item" => 22,
        "Change terms of payment key" => 20,
        "Change item text" => 25,
        "Reduce purchase order item price" => 13,
        "Increase purchase order item quantity" => 11,
        "Create purchase order item" => 0,
        "Create MM invoice by vendor" => 1,
        "Change purchase order item" => 3,
        "Change payment method" => 18,
        "Increase purchase order item net value" => 16,
        "Change cash discount percentage 1" => 21,
        "Change purchase order (other)" => 23,
        "Create FI invoice by vendor" => 9,
        "Enter goods receipt" => 6,
        "Post invoice in MM" => 8,
        "Set final delivery indicator" => 17,
        "Set payment block" => 26,
        "Change baseline date for payment" => 29,
        "Post invoice in FI" => 7,
        "Record order confirmation" => 12,
        "Change purchase order (purchasing group)" => 19,
        "Increase purchase order item price" => 14,
        "Block purchase order item" => 28,
        "Remove payment block" => 5,
        "Clear open item" => 4,
        "Change cash discount days 1" => 15,
        "Reduce purchase order item net value" => 2,
        _ => panic!("Unknown activity"),
    }
}

pub fn num_to_str(num: u8) -> &'static str {
    match num {
        1 => "Create MM invoice by vendor",
        12 => "Record order confirmation",
        14 => "Increase purchase order item price",
        20 => "Change terms of payment key",
        5 => "Remove payment block",
        6 => "Enter goods receipt",
        13 => "Reduce purchase order item price",
        9 => "Create FI invoice by vendor",
        25 => "Change item text",
        15 => "Change cash discount days 1",
        22 => "Unblock purchase order item",
        8 => "Post invoice in MM",
        18 => "Change payment method",
        10 => "Reduce purchase order item quantity",
        2 => "Reduce purchase order item net value",
        7 => "Post invoice in FI",
        26 => "Set payment block",
        4 => "Clear open item",
        0 => "Create purchase order item",
        21 => "Change cash discount percentage 1",
        19 => "Change purchase order (purchasing group)",
        24 => "Change cash discount days 2",
        16 => "Increase purchase order item net value",
        29 => "Change baseline date for payment",
        11 => "Increase purchase order item quantity",
        28 => "Block purchase order item",
        3 => "Change purchase order item",
        17 => "Set final delivery indicator",
        23 => "Change purchase order (other)",
        27 => "Change purchase order (currency)",
        _ => panic!("Unknown activity number"),
    }
}


fn preprocess_activities(cases: Vec<(String, NaiveDateTime, String)>) -> (HashMap<String, u8>, HashMap<u8, String>) {
    let mut activity_map: HashMap<String, u8> = HashMap::new();
    let mut reverse_map: HashMap<u8, String> = HashMap::new();
    let mut current_num: u8 = 0;

    for (_, _, activity) in &cases {
        if !activity_map.contains_key(activity) {
            activity_map.insert(activity.clone(), current_num);
            reverse_map.insert(current_num, activity.clone());
            current_num += 1;
        }
    }

    (activity_map, reverse_map)
}

fn generate_function_prototypes(activity_map: HashMap<String, u8>, reverse_map: HashMap<u8, String>) {
    println!("fn str_to_num(activity: &str) -> u8 {{");
    println!("    match activity {{");

    for (activity, num) in &activity_map {
        println!("        \"{}\" => {},", activity, num);
    }

    println!("        _ => panic!(\"Unknown activity\"),");
    println!("    }}");
    println!("}}");

    println!("\nfn num_to_str(num: u8) -> &'static str {{");
    println!("    match num {{");

    for (num, activity) in &reverse_map {
        println!("        {} => \"{}\",", num, activity);
    }

    println!("        _ => panic!(\"Unknown activity number\"),");
    println!("    }}");
    println!("}}");
}
