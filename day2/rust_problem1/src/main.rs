use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::string::String;
use std::collections::HashMap;


fn find_checksum(s: String) -> i32  {
    let mut count_2 = 0;
    let mut count_3 = 0;
    for word in s.split_whitespace() {
        let mut counter: HashMap<char, i32> = HashMap::new();
        for item in word.chars() {
            let entry = counter.entry(item).or_insert(0);
            *entry += 1;
        }
        if counter.values().any(|&x| x == 2) {
            count_2 +=1 ;
        }
        if counter.values().any(|&x| x == 3) {
            count_3 +=1 ;
        }
    }
    count_2 * count_3
}

fn process_file(filename: &str, func: fn(String) -> i32) -> i32 {
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
    print!("The answer is: {}\n", process_file("/Users/james/PycharmProjects/advent-of-code-2018/day2/input.txt", find_checksum));
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(process_file("/Users/james/PycharmProjects/advent-of-code-2018/day2/test_files/test1.txt", find_checksum), 12);
    }
}