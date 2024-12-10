use std::fs::read_to_string;

fn main() {
    let blob = read_to_string("src/input.txt").expect("should read input.txt file.");

    let mut sum = 0;

    for potential_instruction in blob.split("mul(") {
        let trimmed_potential_instruction = potential_instruction.trim();

        if !trimmed_potential_instruction.contains(")") {
            continue;
        }

        for instruction in trimmed_potential_instruction.split(")") {
            if instruction.len() < 3 || instruction.len() > 8 {
                break;
            }

            let instruct: Vec<String> = instruction
                .split(",")
                .map(|input| input.to_string())
                .collect();

            if let [left, right] = &instruct[..] {
                let left: u32 = match left.parse() {
                    Ok(left) => left,
                    Err(e) => continue,
                };
                let right: u32 = match right.parse() {
                    Ok(right) => right,
                    Err(e) => continue,
                };

                sum += (left * right);
            }
        }
    }

    println!("sum is {sum}");
}
