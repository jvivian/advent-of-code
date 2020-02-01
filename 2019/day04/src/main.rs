// However, they do remember a few key facts about the password:
//
// It is a six-digit number.
// The value is within the range given in your puzzle input.
// Two adjacent digits are the same (like 22 in 122345).
// Going from left to right, the digits never decrease; they only ever increase or stay the same (like 111123 or 135679).
// Other than the range rule, the following are true:
//
// 111111 meets these criteria (double 11, never decreases).
// 223450 does not meet these criteria (decreasing pair of digits 50).
// 123789 does not meet these criteria (no double).
// P1: How many different passwords within the range given in your puzzle input meet these criteria?
// Range is: 156218-652527

// P2: the two adjacent matching digits are not part of a larger group of matching digits.

// 112233 meets these criteria because the digits never decrease and all repeated digits are exactly two digits long.
// 123444 no longer meets the criteria (the repeated 44 is part of a larger group of 444).
// 111122 meets the criteria (even though 1 is repeated more than twice, it still contains a double 22).

fn main() {
    let start = 156218;
    let stop = 652527;
    let mut num_pass_1 = 0;
    let mut num_pass_2 = 0;

    // Steps
    for i in start..stop {
        // Convert to array of integers
        let arr: Vec<char> = i.clone().to_string().chars().collect();

        if decreases(&arr) {
            if has_adjacent(&arr) {
                num_pass_1 += 1
            }
            if has_adjacent_2(&arr) {
                num_pass_2 += 1
            }
        }
    }
    println!("NumPass1: {}\tNumPass2: {}", num_pass_1, num_pass_2);
}

// Returns a bool of whether every number decreases from left to right
fn decreases(arr: &Vec<char>) -> bool {
    for i in 0..arr.len() - 1 {
        if arr[i] <= arr[i + 1] {
            continue;
        } else {
            return false;
        }
    }
    return true;
}

fn has_adjacent(arr: &Vec<char>) -> bool {
    for i in 0..arr.len() - 1 {
        if arr[i] == arr[i + 1] {
            return true;
        } else {
            continue;
        }
    }
    return false;
}

fn has_adjacent_2(arr: &Vec<char>) -> bool {
    let l = arr.len();
    for i in 1..arr.len() - 2 {
        if arr[i] == arr[i + 1] {
            // check both sides of doublet for matches
            if (arr[i - 1] == arr[i]) || (arr[i + 2] == arr[i]) {
                continue;
            } else {
                return true;
            }
        }
    }
    // Check first and last triplet
    if (arr[0] == arr[1]) & (arr[0] != arr[2]) {
        return true;
    } else if (arr[l - 1] == arr[l - 2]) & (arr[l - 2] != arr[l - 3]) {
        return true;
    } else {
        return false;
    }
}
