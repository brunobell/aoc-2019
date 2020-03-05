use std::collections::HashMap;
use std::error::Error;
use std::fs;

fn count_orbits(map: &HashMap<String, Vec<String>>, key: String, last: i32) -> i32 {
    if !map.contains_key(&key) {
        last
    } else {
        last + map
            .get(&key)
            .cloned()
            .unwrap()
            .iter()
            .map(|k| count_orbits(&map, String::from(k), last + 1))
            .sum::<i32>()
    }
}

fn walk_up_one_step(map: &HashMap<String, Vec<String>>, key: String) -> Option<String> {
    for (k, v) in map {
        if v.contains(&key) {
            return Some(k.clone());
        }
    }
    None
}

fn walk_up_to_root(map: &HashMap<String, Vec<String>>, from: String) -> Vec<String> {
    let mut from = from;
    let mut route = Vec::new();
    loop {
        let key = walk_up_one_step(map, from.clone());
        match key {
            Some(key) => {
                from = key.clone();
                route.push(key)
            }
            None => break,
        }
    }
    route
}

fn min_orbital_transfers(map: &HashMap<String, Vec<String>>, from: String, to: String) -> i32 {
    let route_a = walk_up_to_root(map, from);
    let route_b = walk_up_to_root(map, to);
    let mut result = 0i32;
    'outer: for (i, a) in route_a.iter().enumerate() {
        'inner: for (j, b) in route_b.iter().enumerate() {
            if *a == *b {
                result = (i + j) as i32;
                break 'outer;
            }
        }
    }
    result
}

fn lines_to_map(input: String) -> HashMap<String, Vec<String>> {
    input.lines().fold(HashMap::new(), |mut map, line| {
        let splits = line
            .trim()
            .split(')')
            .take(2)
            .collect::<Vec<&str>>()
            .to_vec();
        let (k, v) = (splits[0], splits[1]);
        if map.contains_key(k) {
            map.get_mut(k).unwrap().push(v.to_string());
        } else {
            map.insert(k.to_string(), vec![v.to_string()]);
        }
        map
    })
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input-part-1.txt")?;
    let map: HashMap<String, Vec<String>> = lines_to_map(input);
    println!(
        "the answer to part-1 is {}",
        count_orbits(&map, String::from("COM"), 0)
    );

    let input = fs::read_to_string("input-part-1.txt")?;
    let map: HashMap<String, Vec<String>> = lines_to_map(input);
    println!(
        "the answer to part-2 is {}",
        min_orbital_transfers(&map, String::from("YOU"), String::from("SAN"))
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{count_orbits, lines_to_map, min_orbital_transfers};

    #[test]
    fn test_count_orbits() {
        let input = String::from(
            "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L",
        );
        let map = lines_to_map(input);
        let counts = count_orbits(&map, String::from("COM"), 0);
        assert_eq!(counts, 42)
    }

    #[test]
    fn test_min_orbital_transfers() {
        let input = String::from(
            "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN",
        );
        let map = lines_to_map(input);
        let counts = min_orbital_transfers(&map, String::from("YOU"), String::from("SAN"));
        assert_eq!(counts, 4)
    }
}
