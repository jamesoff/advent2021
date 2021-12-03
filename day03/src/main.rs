mod data;

fn main() {
    println!("challenge 1 is {}", challange1());
    println!("challenge 2 is {}", challenge2());
}

fn gamma_rate(inputs: &Vec<&str>, size: usize) -> i32 {
    let mut gamma = 0;
    for position in 1..size + 1 {
        gamma = gamma << 1;
        if most_common(&inputs, position) {
            gamma |= 1;
        }
        // println!("position: {}, current gamma: {:b}", position, gamma);
    }
    gamma
}

fn epsilon_rate(inputs: &Vec<&str>, size: usize) -> i32 {
    let gamma = gamma_rate(&inputs, size);
    let epsilon = !gamma & ((1 << size) - 1);
    epsilon
}

fn str_to_bin(input: &str) -> i32 {
    let mut result = 0;
    for char in input.chars() {
        result = result << 1;
        if char == '1' {
            result |= 1
        }
    }
    result
}

fn bit_at_position(value: i32, size: usize, position: usize) -> i32 {
    value >> (size - position) & 1
}

fn most_common(inputs: &Vec<&str>, position: usize) -> bool {
    let mut ones = 0;
    for item in inputs {
        let size = item.len();
        let value = bit_at_position(str_to_bin(item), size, position);
        if value > 0 {
            ones += 1;
        }
    }
    ones > (inputs.len() / 2)
}

fn filter_entries(inputs: &Vec<&str>, filter_for: i32) -> i32 {
    let mut this_vec: Vec<&str> = inputs.clone();
    let mut position = 1;
    while this_vec.len() > 1 {
        let mut matches: Vec<&str> = Vec::new();
        let mut not_matches: Vec<&str> = Vec::new();

        for item in this_vec {
            let value = str_to_bin(item);
            if bit_at_position(value, item.len(), position) == 1 {
                matches.push(item)
            } else {
                not_matches.push(item)
            }
        }
        position += 1;
        if filter_for == 1 {
            if matches.len() >= not_matches.len() {
                this_vec = matches;
            } else {
                this_vec = not_matches;
            }
        } else {
            if matches.len() < not_matches.len() {
                this_vec = matches;
            } else {
                this_vec = not_matches;
            }
        }
    }
    str_to_bin(this_vec.iter().next().expect("bad value after filtering"))
}

fn challange1() -> i32 {
    gamma_rate(&data::input(), 12) * epsilon_rate(&data::input(), 12)
}

fn challenge2() -> i32 {
    let oxygen = filter_entries(&data::input(), 1);
    let co2 = filter_entries(&data::input(), 0);
    println!("oxygen: {}, co2: {}", oxygen, co2);
    oxygen * co2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_most_common() {
        assert_eq!(most_common(&vec!["00100", "11110", "10110"], 1), true);
        assert_eq!(most_common(&vec!["00100", "11110", "10110"], 2), false);
    }

    #[test]
    fn test_gamma() {
        assert_eq!(
            gamma_rate(
                &vec![
                    "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100",
                    "10000", "11001", "00010", "01010",
                ],
                5
            ),
            22
        );
    }

    #[test]
    fn test_epsilon() {
        assert_eq!(
            epsilon_rate(
                &vec![
                    "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100",
                    "10000", "11001", "00010", "01010",
                ],
                5
            ),
            9
        );
    }

    #[test]
    fn test_str_to_bin() {
        assert_eq!(str_to_bin("10101"), 21)
    }

    #[test]
    fn test_bit_at_position() {
        assert_eq!(bit_at_position(21, 5, 1), 1);
        assert_eq!(bit_at_position(21, 5, 2), 0);
    }

    #[test]
    fn test_filter_for() {
        assert_eq!(
            filter_entries(
                &vec![
                    "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100",
                    "10000", "11001", "00010", "01010",
                ],
                1
            ),
            23
        );
        assert_eq!(
            filter_entries(
                &vec![
                    "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100",
                    "10000", "11001", "00010", "01010",
                ],
                0
            ),
            10
        )
    }
}
