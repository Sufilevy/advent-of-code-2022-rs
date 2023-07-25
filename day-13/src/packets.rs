use std::{ cmp::Ordering, slice::Iter };

#[derive(Clone, Debug)]
pub enum PacketValue {
    Int(u32),
    List(Vec<PacketValue>),
}

impl PacketValue {
    pub fn list(val: u32) -> Self {
        Self::List(vec![Self::Int(val)])
    }
}

type PacketPair = (PacketValue, PacketValue);

pub fn parse_packets(input: &[&str]) -> Vec<PacketPair> {
    input
        .iter()
        .map(|pair| {
            let pair: Vec<&str> = pair.split("\r\n").collect();
            (parse_packet(pair[0]), parse_packet(pair[1]))
        })
        .collect()
}

fn parse_packet(input: &str) -> PacketValue {
    if input.is_empty() {
        PacketValue::List(Vec::new())
    } else if input.starts_with('[') {
        let data = &input[1..input.len() - 1];
        let list = split_values_list(data)
            .into_iter()
            .map(|s| parse_packet(&s))
            .collect();
        PacketValue::List(list)
    } else {
        PacketValue::Int(
            input
                .parse()
                .unwrap_or_else(|_| panic!("failed to parse packet value '{input}' to int"))
        )
    }
}

fn split_values_list(input: &str) -> Vec<String> {
    if !input.contains(',') {
        return vec![input.to_owned()];
    }

    let mut values = Vec::new();

    let mut bracket_count = 0;
    let mut current_value = String::new();
    for c in input.chars() {
        current_value.push(c);
        match c {
            '[' => {
                bracket_count += 1;
            }
            ']' => {
                bracket_count -= 1;
            }
            ',' if bracket_count == 0 => {
                values.push(current_value.trim_end_matches(',').to_owned());
                current_value = String::new();
            }
            _ => {}
        }
    }

    let final_value = if bracket_count == 0 {
        current_value
    } else {
        current_value.trim_end_matches(']').to_owned()
    };
    values.push(final_value);

    values
}

pub fn compare_packet_pair(pair: &PacketPair) -> Ordering {
    match pair {
        (PacketValue::Int(left), PacketValue::Int(right)) => left.cmp(right),
        (PacketValue::Int(left), PacketValue::List(_)) => {
            let pair = (PacketValue::list(*left), pair.1.clone());
            compare_packet_pair(&pair)
        }
        (PacketValue::List(_), PacketValue::Int(right)) => {
            let pair = (pair.0.clone(), PacketValue::list(*right));
            compare_packet_pair(&pair)
        }
        (PacketValue::List(left), PacketValue::List(right)) =>
            compare_packet_pair_list((left.iter(), right.iter())),
    }
}

fn compare_packet_pair_list(mut pair: (Iter<PacketValue>, Iter<PacketValue>)) -> Ordering {
    let left = pair.0.next();
    let right = pair.1.next();

    match (left, right) {
        (None, None) => Ordering::Equal,
        (None, Some(_)) => Ordering::Less,
        (Some(_), None) => Ordering::Greater,
        (Some(left), Some(right)) => {
            // There are items in both lists
            let result = compare_packet_pair(&(left.clone(), right.clone()));
            if result == Ordering::Equal {
                compare_packet_pair_list(pair)
            } else {
                result
            }
        }
    }
}
