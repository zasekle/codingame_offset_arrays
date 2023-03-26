use std::io;
use std::collections::HashMap;
use std::str;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

struct Container {
    nums: Vec<i64>,
    offset: i64
}

impl Container {
    fn get_number(&self, index: i64) -> i64 {
        self.nums[(index - self.offset) as usize]
    }
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, i32);

    let mut array_map: HashMap<String, Container> = HashMap::new();
    for _ in 0..n as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let assignment = input_line.trim_matches('\n').to_string().as_bytes().to_vec();

        let start_of_array_pos = assignment.iter().position(|&s| s == b'[').expect("[ not found");
        let first_dot_pos = assignment.iter().position(|&s| s == b'.').expect(". not found");
        let equal_sign_pos = assignment.iter().position(|&s| s == b'=').expect("= not found");

        let array_name = str::from_utf8(&assignment[..start_of_array_pos]).expect("array_name no utf8 encoding");
        let start_range = str::from_utf8(&assignment[start_of_array_pos+1..first_dot_pos]).expect("start_range no utf8 encoding").parse::<i64>().unwrap();

        let mut container = Container{
            nums: Vec::new(),
            offset: start_range
        };

        let number_list =  str::from_utf8(&assignment[equal_sign_pos+2..])
            .expect("number_list no utf8 encoding").split(' ');

        for num in number_list {
            container.nums.push(num.parse::<i64>().unwrap());
        }

        array_map.insert(array_name.to_string(), container);
    }
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let input_element = input_line.trim_matches('\n').to_string().as_bytes().to_vec();

    let mut prev = Vec::<String>::new();
    let mut i = 0;
    loop {
        if b'A' <= input_element[i] && input_element[i] <= b'Z' { //letter
            let mut identifier = String::new();
            while input_element[i] != b'[' {
                identifier.push(input_element[i] as char);
                i += 1;
            }
            prev.push(identifier);
        } else { //number
            let mut number_str = String::new();
            while input_element[i] != b']' {
                number_str.push(input_element[i] as char);
                i += 1;
            }
            let mut number = number_str.parse::<i64>().unwrap();

            for str in prev.iter().rev() {
                number = array_map[str].get_number(number);
            }

            println!("{number}");
            break;
        }
        i += 1;
    }

}
