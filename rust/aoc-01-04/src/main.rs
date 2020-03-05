use std::error::Error;
use std::collections::HashMap;

fn get_repeat_info(n: i32) -> (bool, HashMap<char, i32>) {
    let (_, is_asc, _, map) = n.to_string().chars().fold(
        ('0', true, 0, HashMap::new()),
        |(digit, is_asc, repeat, mut map), c| {
            if map.contains_key(&c) {
                map.insert(c, map.get(&c).unwrap() + 1);
            } else {
                map.insert(c, 1);
            }
            if digit <= c {
                if digit == c {
                    (c, is_asc, repeat + 1, map)
                } else {
                    (c, is_asc, 1, map)
                }
            } else {
                (c, false, 1, map)
            }
        },
    );
    (is_asc, map)
}

fn is_password_part_1(n: i32, min: i32, max: i32) -> bool {
    let (is_asc, map) = get_repeat_info(n);
    let min_repeat = *map.values().filter(|&x| x > &1).min().unwrap_or(&1);
    let max_repeat = *map.values().max().unwrap();
    if is_asc && max_repeat <= max && min_repeat >= min {
        true
    } else {
        false
    }
}

fn is_password_part_2(n: i32, repeat: i32) -> bool {
    let (is_asc, map) = get_repeat_info(n);
    let valid = map.values().filter(|&x| x == &repeat).count() > 0;
    if is_asc && valid {
        true
    } else {
        false
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let start: i32 = 353096;
    let end: i32 = 843212;
    let passwords = (start..=end)
        .filter(|&n| is_password_part_1(n, 2, 6))
        .collect::<Vec<i32>>();
    println!("the answer to part-1 is {}", passwords.len());

    let passwords = (start..=end)
        .filter(|&n| is_password_part_2(n, 2))
        .collect::<Vec<i32>>();
    println!("the answer to part-2 is {}", passwords.len());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::is_password_part_1;
    use crate::is_password_part_2;

    #[test]
    fn test_is_password_part_1() {
        assert_eq!(is_password_part_1(111111, 2, 6), true);
        assert_eq!(is_password_part_1(223450, 2, 6), false);
        assert_eq!(is_password_part_1(123789, 2, 6), false);
    }

    #[test]
    fn test_is_password_part_2() {
        assert_eq!(is_password_part_2(112233, 2), true);
        assert_eq!(is_password_part_2(123444, 2), false);
        assert_eq!(is_password_part_2(111122, 2), true);
    }
}
