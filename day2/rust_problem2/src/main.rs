extern crate itertools;

use itertools::Itertools;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::string::String;
use std::collections::HashMap;
use std::collections::HashSet;



fn get_common_letters(stringa: &str, stringb: &str) -> String {
    let mut common_string: String = "".to_string();
    for (a, b) in stringa.chars().zip(stringb.chars()) {
        if a == b {
            common_string.push(a)
        }
    }
    common_string
}


fn compare_counters(counter_a: &HashMap<char, i32>, counter_b: &HashMap<char, i32>) -> bool {
    let a: HashSet<(&char, &i32)> = counter_a.into_iter().collect();
    let b: HashSet<(&char, &i32)> = counter_b.into_iter().collect();
    let diff = a.symmetric_difference(&b).collect::<Vec<&(&char,&i32)>>();
    if diff.len() == 2 || diff.len() == 3 {
        true
    } else {
        false
    }
}

fn find_common(s: String) -> String  {
    let mut common = "".to_string();
    let counters: Vec<(String, HashMap<char, i32>)> = s
        .split_whitespace()
        .into_iter()
        .map(|word| {
            let mut counter: HashMap<char, i32> = HashMap::new();
            for item in word.chars() {
                let entry = counter.entry(item).or_insert(0);
                *entry += 1;
            }
            (word.to_string(), counter).to_owned()}).collect();

    for combo in counters.iter().combinations(2){
        let a: &(String, HashMap<char, i32>) = &combo[0];
        let b: &(String, HashMap<char, i32>) = &combo[1];
        if a.0 == b.0 {
            continue;
        }
        if !compare_counters(&a.1, &b.1) {
            continue;
        }
        common = get_common_letters(&a.0, &b.0);
        if common.len() == a.0.len() - 1 {
            break
        }
    }

    common
}

fn process_file(filename: &str, func: fn(String) -> String) -> String {
    // Create a path to the desired file
    let path = Path::new(filename);
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", display,
                                                   why.description()),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display,
                                                   why.description()),
        Ok(_) => func(s),
    }

    // `file` goes out of scope, and the "hello.txt" file gets closed
}

fn main() {
    print!("The answer is: {}\n", process_file("/Users/james/PycharmProjects/advent-of-code-2018/day2/input.txt", find_common));
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(process_file("/Users/james/PycharmProjects/advent-of-code-2018/day2/test_files/test1.txt", find_common), "abcde");
    }

    #[test]
    fn test2() {
        assert_eq!(process_file("/Users/james/PycharmProjects/advent-of-code-2018/day2/test_files/test2.txt", find_common), "fgij");
    }
}