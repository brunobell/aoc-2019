use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input-part-1.txt")?;
    let answer: i32 = input
        .lines()
        .map(|line| fuel_part_1(line.parse::<i32>().expect("invalid input")))
        .sum();
    println!("the answer to part-1 is {}", answer);

    let input = fs::read_to_string("input-part-2.txt")?;
    let answer: i32 = input
        .lines()
        .map(|line| fuel_part_2(line.parse::<i32>().expect("invalid input")))
        .sum();
    println!("the answer to part-2 is {}", answer);

    Ok(())
}

fn fuel_part_1(mass: i32) -> i32 {
    mass / 3 - 2
}

fn fuel_part_2(mass: i32) -> i32 {
    let mut fuel_vec: Vec<i32> = Vec::new();
    let mut mass = mass;
    loop {
        mass = fuel_part_1(mass);
        if mass > 0 {
            fuel_vec.push(mass);
        } else {
            break;
        }
    }
    fuel_vec.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::fuel_part_1;
    use crate::fuel_part_2;

    #[test]
    fn test_fuel_part_1() {
        assert_eq!(fuel_part_1(12), 2);
        assert_eq!(fuel_part_1(14), 2);
        assert_eq!(fuel_part_1(1969), 654);
        assert_eq!(fuel_part_1(100756), 33583);
    }

    #[test]
    fn test_fuel_part_2() {
        assert_eq!(fuel_part_2(14), 2);
        assert_eq!(fuel_part_2(1969), 966);
        assert_eq!(fuel_part_2(100756), 50346);
    }
}
