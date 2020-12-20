#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref RE: regex::Regex = regex::Regex::new("(\\d+)-(\\d+) (.)").unwrap();
}

fn main() {
    let values = util::iter_from_file("resources/data.txt");

    let mut count = 0;
    let mut new_count = 0;
    for mut str in values {
        match str.find(':') {
            Some(x) => {
                let slice = str.split_off(x);
                let password = slice.strip_prefix(": ").unwrap();

                let policy = build_policy(&str);

                if fulfils_policy(password, &policy) {
                    count = count + 1;
                }
                if fulfils_new_policy(password, &policy) {
                    new_count = new_count + 1;
                }
            }
            None => println!("Can't find : to split"),
        }
    }

    println!("Matches: {} and {}", count, new_count);
}

#[derive(Debug)]
struct Policy {
    character: char,
    min: usize,
    max: usize,
}

fn build_policy(str: &String) -> Policy {
    let captures = RE.captures(str).unwrap();

    Policy {
        character: captures[3].chars().next().unwrap(),
        min: captures[1].parse::<usize>().unwrap(),
        max: captures[2].parse::<usize>().unwrap(),
    }
}

fn fulfils_policy(password: &str, policy: &Policy) -> bool {
    let count = password.matches(policy.character).count();

    count >= policy.min && count <= policy.max
}

fn fulfils_new_policy(password: &str, policy: &Policy) -> bool {
    let chars: Vec<char> = password.chars().collect();
    let char1 = chars[policy.min - 1];
    let char2 = chars[policy.max - 1];

    if char1 == policy.character && char2 == policy.character {
        return false;
    } else if char1 == policy.character || char2 == policy.character {
        return true;
    } else {
        return false;
    }
}
