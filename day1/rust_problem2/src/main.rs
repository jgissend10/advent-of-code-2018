use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::string::String;
use std::collections::HashSet;


fn compute_freq(s: String) -> i32  {
    let mut freq = 0;
    let mut freq_set: HashSet<i32> = vec!(0i32).into_iter().collect();

    'outer: loop {
        'inner: for number in s.split_whitespace() {
            let my_int: i32 = number.parse().unwrap();
            freq += my_int;
            if freq_set.contains(&freq) {
                break 'outer;
            }
            freq_set.insert(freq);
        }
    }

    freq
}

fn process_file(filename: &str) -> i32 {
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
        Ok(_) => compute_freq(s),
    }

    // `file` goes out of scope, and the "hello.txt" file gets closed
}

fn main() {
    print!("The answer is: {}\n", process_file("/Users/james/PycharmProjects/advent-of-code-2018/day1/input.txt"));
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(process_file("/Users/james/PycharmProjects/advent-of-code-2018/day1/test_files/test4.txt"), 0);
    }

    #[test]
    fn test2() {
        assert_eq!(process_file("/Users/james/PycharmProjects/advent-of-code-2018/day1/test_files/test5.txt"), 10);
    }

    #[test]
    fn test3() {
        assert_eq!(process_file("/Users/james/PycharmProjects/advent-of-code-2018/day1/test_files/test6.txt"), 5);
    }

    #[test]
    fn test4() {
        assert_eq!(process_file("/Users/james/PycharmProjects/advent-of-code-2018/day1/test_files/test7.txt"), 14);
    }
}