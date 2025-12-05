use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut contents = String::new();
    File::open("input.txt")?.read_to_string(&mut contents)?;
    let mut all_endpoints: Vec<Vec<u64>> = vec![];
    let mut all_numbers: u64 = 0;

    let ranges: Vec<&str> = contents
        .lines()
        .filter(|line| !line.trim().is_empty() && line.contains('-'))
        .collect();

    // PART 2
    for range in &ranges {
        let endpoints: Vec<&str> = range.split('-').collect();
        let mut new_range: Vec<u64> = endpoints
            .iter()
            .filter_map(|&endpoint| endpoint.parse::<u64>().ok())
            .collect();

        if new_range.len() == 2 {
            let mut merged_ranges = vec![];

            for enpoint in all_endpoints {
                if !(new_range[1] < enpoint[0] || new_range[0] > enpoint[1]) {
                    new_range[0] = new_range[0].min(enpoint[0]);
                    new_range[1] = new_range[1].max(enpoint[1]);
                } else {
                    merged_ranges.push(enpoint.clone());
                }
            }

            merged_ranges.push(new_range);
            all_endpoints = merged_ranges;
        }
    }

    println!("{:?}", all_endpoints);

    for endpoints in all_endpoints {
        all_numbers += (endpoints[1] + 1) - endpoints[0];
    }

    println!("{}", all_numbers);

    Ok(())
}

//PART 1
//for ingredient in ingredients {
//    for range in &ranges {
//        let endpoints: Vec<&str> = range.split("-").collect();
//        let ingredient_value: u64 = ingredient.parse().unwrap_or(0);
//        let endpoints_values: Vec<u64> = endpoints
//            .iter()
//            .filter_map(|&endpoint| endpoint.parse::<u64>().ok())
//            .collect();
//
//        if endpoints_values.len() == 2 {
//            let start = endpoints_values[0];
//            let end = endpoints_values[1];
//            if ingredient_value >= start && ingredient_value <= end {
//                let mut is_in_fresh_ingredients = false;
//                for fresh_ingredient in &fresh_ingredients {
//                    if *fresh_ingredient == ingredient_value {
//                        is_in_fresh_ingredients = true;
//                        break;
//                    }
//                }
//                if !is_in_fresh_ingredients {
//                    fresh_ingredients.push(ingredient_value);
//                }
//            }
//        }
//    }
//}
