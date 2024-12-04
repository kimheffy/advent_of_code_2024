use std::{
    collections::HashMap,
    fs::{self},
};

#[derive(Debug)]
struct PathedIoError {
    path: String,
    inner: std::io::Error,
}

fn read_strings() -> Result<String, PathedIoError> {
    let path = "src/input.txt";
    match fs::read_to_string(path) {
        Ok(s) => Ok(s),
        Err(e) => Err(PathedIoError {
            path: path.into(),
            inner: e,
        }),
    }
}

fn part_one(contents: &str) {
    let mut left_bucket: Vec<u32> = Vec::new();
    let mut right_bucket: Vec<u32> = Vec::new();

    let mut sum: Vec<u32> = Vec::new();

    for line in contents.lines() {
        let bucket = line.split_whitespace().collect::<Vec<&str>>();
        let left = bucket
            .get(0)
            .expect("Should get the 0th index (left) from bucket.");
        let right = bucket
            .get(1)
            .expect("Should get the 1st index (right) from bucket.");

        let left: u32 = left
            .parse()
            .expect("left should have been converted to u32.");
        let right: u32 = right
            .parse()
            .expect("right should have been converted to u32.");

        left_bucket.push(left);
        right_bucket.push(right);
    }

    left_bucket.sort();
    right_bucket.sort();

    assert!(left_bucket.len() == right_bucket.len());

    for i in 0..left_bucket.len() {
        let left = left_bucket
            .get(i)
            .expect("Getting the ith item in left_bucket.");
        let right = right_bucket
            .get(i)
            .expect("Getting the ith item in right bucket.");

        let diff = left.abs_diff(*right);

        sum.push(diff);
    }

    println!("sum: {:?}", sum);

    let result = sum
        .into_iter()
        .reduce(|acc, cur| acc + cur)
        .expect("iterator should not be empty!");
    println!("result: {result}");
}

fn part_two(content: &str) {
    let mut left_side: Vec<u32> = Vec::new();
    let mut right_frequency: HashMap<u32, u32> = HashMap::new();

    let mut results: Vec<u32> = Vec::new();

    for line in content.lines() {
        let split: Vec<&str> = line.split_whitespace().collect();
        if let [left, right] = &split[..] {
            let left: u32 = left
                .parse()
                .expect("left should be parsed to u32 from str slice.");
            let right: u32 = right
                .parse()
                .expect("right should be parsed to u32 from str slice.");

            left_side.push(left);
            right_frequency
                .entry(right)
                .and_modify(|n| *n += 1)
                .or_insert(1);
        }
    }

    println!("left: {:?}", left_side);
    println!("right-frequencies... {:?}", right_frequency);

    for num in left_side {
        match right_frequency.get(&num) {
            Some(&n) => results.push(n * num),
            None => results.push(0),
        }
    }

    let answer = results
        .iter()
        .copied()
        .reduce(|acc, n| acc + n)
        .expect("iterator should not be empty!");

    println!("results: {answer}");
}

fn main() {
    let lines = read_strings().unwrap();
    part_one(&lines);
    part_two(&lines);
}
