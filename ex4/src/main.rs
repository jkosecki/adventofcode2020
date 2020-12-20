#[macro_use] extern crate lazy_static;

use std::collections::HashMap;


lazy_static! {
    // static ref RE: regex::Regex = regex::Regex::new("((\\w{3}):(\\S*))\\s?").unwrap();
    static ref RE: regex::Regex = regex::Regex::new("((\\w{3}):(\\S*))").unwrap();

    static ref REQUIRED: Vec<&'static str> = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
}

fn main() {
    let values = util::iter_from_file("resources/data2.txt");

    let mut passport_data = Vec::new();
    let mut passport_line: String = String::new();
    for line in values {
        passport_line = passport_line + " " + &line;
        if line == "" {
            // println!("Passport finished: {}", passport_line);
            passport_data.push(passport_line);

            passport_line = String::new(); 
        }
    }
    // println!("Passport finished: {}", passport_line);
    passport_data.push(passport_line);

    // println!("");
    let mut count = 0;
    for line in passport_data.iter() {
        // println!("Line {} has match: {}", line, RE.is_match(line));

        let map: HashMap<&str,&str> = RE.find_iter(line)
            .map(|mat| mat.as_str())
            .map(|entry| {
                let pair: Vec<&str> = entry.split(':').take(2).collect();

                (pair[0], pair[1])
            })
            .collect();
        // println!("Map: {:?}", map);

        let mut val = 1;
        for required in REQUIRED.iter() {
            if map.contains_key(required) {
                // println!("Map contains key: {}", required);
                val = val << 1;
            } else {
                // println!("Map DOESN'T contain key: {}", required);
            }
        }
        if val == 128 {
            count = count + 1;
        }

        // println!("");
    }
    println!("Valid passports: {}", count)
}

// struct Passport {
//     byr: String,
//     iyr: String,
//     eyr: String,
//     hgt: String,
//     hcl: String,
//     ecl: String,
//     pid: String,
//     cid: String
// }
