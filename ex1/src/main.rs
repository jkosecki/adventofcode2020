
const SEARCHED_VALUE:i32 = 2020;

fn main() {
    let mut values: Vec<i32> = util::iter_from_file("src/data2").map(|str| str.parse::<i32>().unwrap()).collect();
    values.sort();

    'outer: for x in values.iter() {
        for y in values.iter().rev() {
            let sum = x + y;

            if sum == SEARCHED_VALUE {
                println!("The result is {}", x * y);
                break 'outer;
            }
        }
    }

    'outer2: for (i,x) in values.iter().enumerate() {
        for j in i+1..values.len() {
            for k in j+1..values.len() {
                let sum = x + values[j] + values[k];

                if sum == SEARCHED_VALUE {
                    println!("The result is {}", x * values[j] * values[k]);
                    break 'outer2;
                }
            }
        }

    }
}