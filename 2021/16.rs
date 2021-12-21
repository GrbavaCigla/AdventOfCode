use std::u128;

fn convert_to_binary_from_hex(hex: &str) -> String {
    hex.chars().map(to_binary).collect()
}

fn to_binary(c: char) -> &'static str {
    match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => "",
    }
}

fn versum_packet(packet: &str) -> (u32, usize, u64) {
    let mut version = u32::from_str_radix(&packet[0..3], 2).unwrap();
    let typeid = u16::from_str_radix(&packet[3..6], 2).unwrap();
    let mut counter = 6;

    let ans;
    let mut anss = vec![];

    if typeid == 4 {
        let mut i = 0;
        let mut values = String::new();
        loop {
            let value = &packet[i * 5 + counter..(i + 1) * 5 + counter];
            values.push_str(&packet[i * 5 + counter + 1..(i + 1) * 5 + counter]);
            i += 1;
            if &value[0..1] == "0" {
                break;
            }
        }
        ans = u64::from_str_radix(&values, 2).unwrap();

        counter += i * 5;
    } else {
        let length_type = &packet[counter..counter + 1];
        counter += 1;
        if length_type == "0" {
            let bits_length = usize::from_str_radix(&packet[counter..counter+15], 2).unwrap();
            counter+=15;
            
            let mut tmp_counter = 0;
            
            loop {
                if tmp_counter >= bits_length {
                    break;
                }
                let (new_ver, new_cur, val) = versum_packet(&packet[counter+tmp_counter..]);
                anss.push(val);
                // println!("str: {}, new_cur: {}, left: {}", &packet[counter+tmp_counter..counter+tmp_counter+new_cur], new_cur, &packet[counter+tmp_counter+new_cur..]);
                tmp_counter += new_cur;
                version += new_ver;

            }
            counter += tmp_counter;
        } else {
            let subpacket_count = usize::from_str_radix(&packet[counter..counter+11], 2).unwrap();
            counter+=11;

            let mut tmp_counter = 0;
            for i in 0..subpacket_count {
                let (new_ver, new_cur, val) = versum_packet(&packet[counter+tmp_counter..]);
                anss.push(val);
                tmp_counter += new_cur;
                version += new_ver;
            }
            counter += tmp_counter;
        }

        ans = match typeid {
            0 => anss.iter().sum(),
            1 => anss.iter().product(),
            2 => *anss.iter().min().unwrap(),
            3 => *anss.iter().max().unwrap(),
            5 => (anss[0] > anss[1]).into(),
            6 => (anss[0] < anss[1]).into(),
            7 => (anss[0] == anss[1]).into(),
            _ => 0
        }
    }

    (version, counter, ans)
}

fn main() {
    let input = include_str!("inputs/16");

    let binary_input = convert_to_binary_from_hex(input);

    let result = versum_packet(&binary_input);

    println!("First part result: {}", result.0);
    println!("Second part result: {}", result.2);
}