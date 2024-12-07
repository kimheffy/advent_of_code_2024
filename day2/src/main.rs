use std::fs;
use std::io;

#[derive(Debug)]
struct PathIoError {
    path: String,
    inner: io::Error,
}

fn read_strings() -> Result<String, PathIoError> {
    let path = "src/input.txt";
    match fs::read_to_string(path) {
        Ok(contents) => Ok(contents),
        Err(e) => Err(PathIoError {
            path: path.to_string(),
            inner: e.into(),
        }),
    }
}

fn is_all_increasing(v: &Vec<u32>) -> bool {
    let mut all_is_increasing = false;
    let mut left = v.get(0).unwrap();

    for i in 1..v.len() {
        let right = v.get(i).unwrap();

        if left < right {
            all_is_increasing = true;
        } else {
            return false;
        }

        left = right;
    }

    all_is_increasing
}

fn is_all_decreasing(v: &Vec<u32>) -> bool {
    let mut all_is_decreasing = false;
    let mut left = v.get(0).unwrap();

    for i in 1..v.len() {
        let right = v.get(i).unwrap();

        if left > right {
            all_is_decreasing = true;
        } else {
            return false;
        }

        left = right;
    }

    all_is_decreasing
}

fn is_minor_diff(v: &Vec<u32>) -> bool {
    let mut left = v.get(0).expect("should get the 0th element.");

    for i in 1..v.len() {
        let right = v.get(i).expect("should get the ith element.");

        let diff = left.abs_diff(*right);
        if !(diff >= 1 && diff <= 3) {
            return false;
        }

        left = right;
    }

    true
}

fn part_one(contents: &str) {
    let mut report_vec: Vec<u32> = Vec::new();
    let mut counter = 0;

    for report in contents.lines() {
        // TODO: I believe this can written with a 'map'
        for level in report.split_whitespace() {
            let num: u32 = level.parse().expect("should have parsed str to u32");
            report_vec.push(num);
        }

        let delta: Option<String> = if is_all_increasing(&report_vec) == true {
            Some(String::from("increasing"))
        } else if is_all_decreasing(&report_vec) == true {
            Some(String::from("decreasing"))
        } else {
            None
        };

        let minor_diff = if is_minor_diff(&report_vec) {
            Some(String::from("minor"))
        } else {
            None
        };

        report_vec.clear();

        println!("delta is {:?}", delta);

        if delta == None || minor_diff.is_none() {
            println!("Vec was neither all increasing or all decreasing.");
            continue;
        }

        counter += 1;
    }

    println!("counter is {counter}");
}

fn main() {
    let contents = read_strings().expect("should have read the contents of input.txt");

    part_one(&contents);
}
