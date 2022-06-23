
use std::collections::HashMap;

use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    let result = two_sum(vec![3,3], 6);

    println!("{:?}", result);
}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut sorted = nums.clone();
    sorted.sort();
    let mut start_index:usize = 0;
    let mut end_index:usize = nums.len() - 1;

    while start_index < end_index {
        let sum = sorted[start_index] + sorted[end_index];
        if sum == target {
            return vec![start_index as i32, end_index as i32];
        } else if sum < target {
            start_index += 1;
        } else {
            end_index -= 1;
        }
    }

    vec![]
    }
