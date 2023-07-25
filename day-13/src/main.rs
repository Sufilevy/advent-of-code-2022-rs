mod packets;

use std::{ fs, cmp::Ordering };
use packets::*;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let input: Vec<&str> = input.split("\r\n\r\n").collect();

    println!("{}", puzzle_one(&input));
    println!("{}", puzzle_two(&input));
}

fn puzzle_one(input: &[&str]) -> usize {
    parse_packets(input)
        .into_iter()
        .enumerate()
        .map(|(i, p)| if compare_packet_pair(&p) == Ordering::Greater { 0 } else { i + 1 })
        .sum()
}

fn puzzle_two(input: &[&str]) -> usize {
    let mut packets: Vec<PacketValue> = input
        .iter()
        .flat_map(|pair| pair.split("\r\n"))
        .map(parse_packet)
        .collect();

    let two = PacketValue::list_list_int(2);
    let six = PacketValue::list_list_int(6);
    packets.push(two.clone());
    packets.push(six.clone());

    packets.sort_by(|left, right| compare_packet_pair(&(left.clone(), right.clone())));

    let two_index =
        packets
            .iter()
            .position(|packet| *packet == two)
            .unwrap() + 1;
    let six_index =
        packets
            .iter()
            .position(|packet| *packet == six)
            .unwrap() + 1;

    two_index * six_index
}
