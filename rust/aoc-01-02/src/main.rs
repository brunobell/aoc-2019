use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input-part-1.txt")?;
    let mut integers: Vec<i32> = input
        .split(",")
        .map(|x| x.parse::<i32>().expect("invalid integer"))
        .collect::<Vec<i32>>();
    integers[1] = 12;
    integers[2] = 2;
    let answer = int_code(&mut integers);
    println!("answer to part-1 is {}", answer[0]);

    let input = fs::read_to_string("input-part-2.txt")?;
    let integers: Vec<i32> = input
        .split(",")
        .map(|x| x.parse::<i32>().expect("invalid integer"))
        .collect::<Vec<i32>>();
    let target = 19690720;
    'outer: for pos_a in 0..99 {
        'inner: for pos_b in 0..99 {
            let mut integers = integers.clone();
            integers[1] = pos_a as i32;
            integers[2] = pos_b as i32;
            if int_code(&mut integers)[0] == target {
                println!(
                    "noun is {}, verb is {}, answer to part-2 is {}",
                    pos_a,
                    pos_b,
                    pos_a * 100 + pos_b
                );
                break 'outer;
            }
        }
    }
    Ok(())
}

fn int_code(integers: &mut Vec<i32>) -> Vec<i32> {
    match integers.len() / 4 {
        0 => integers,
        n @ _ => {
            for i in 0..n {
                let start = 4 * i;
                let op_code = integers[start] as usize;
                let pos_a = integers[start + 1] as usize;
                let pos_b = integers[start + 2] as usize;
                let pos_c = integers[start + 3] as usize;
                match op_code {
                    1 => integers[pos_c] = integers[pos_a] + integers[pos_b],
                    2 => integers[pos_c] = integers[pos_a] * integers[pos_b],
                    99 => break,
                    _ => panic!("bad op_code"),
                }
            }
            integers
        }
    }
    .to_owned()
}

#[cfg(test)]
mod tests {
    use super::int_code;

    #[test]
    fn test_int_code_part_1() {
        assert_eq!(int_code(&mut vec![1, 0, 0, 0, 99]), vec![2, 0, 0, 0, 99]);
        assert_eq!(int_code(&mut vec![2, 3, 0, 3, 99]), vec![2, 3, 0, 6, 99]);
        assert_eq!(
            int_code(&mut vec![2, 4, 4, 5, 99, 0]),
            vec![2, 4, 4, 5, 99, 9801]
        );
        assert_eq!(
            int_code(&mut vec![1, 1, 1, 4, 99, 5, 6, 0, 99]),
            vec![30, 1, 1, 4, 2, 5, 6, 0, 99]
        );
    }
}
