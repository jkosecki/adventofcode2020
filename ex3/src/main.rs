fn main() {
    let values = util::iter_from_file("resources/data2.txt");

    let mut map = Vec::new();
    for line in values {
        let mut row = Vec::new();

        for point in line.chars() {
            row.push(if point == '#' { true } else { false })
        }

        map.push(row);
    }

    let c1 = ride(&map, (1, 1));
    let c2 = ride(&map, (1, 3));
    let c3 = ride(&map, (1, 5));
    let c4 = ride(&map, (1, 7));
    let c5 = ride(&map, (2, 1));

    println!("Result: {}", c1 * c2 * c3 * c4 * c5);
}


fn ride(map: &Vec<Vec<bool>>, diff: (usize, usize)) -> usize {
    let mut count = 0;
    let height = map.len();
    let width = map[0].len();
    let mut i = 0;
    let mut j = 0;

    while i < height {
        // println!("At {}:{}", i, j);
        if map[i][j] {
            // println!("Tree at {}:{}", i, j);
            count = count + 1;
        }
        j = (j + diff.1) % width;
        i = i + diff.0;
    }
    println!("Trees with move {:?}: {}", diff, count);

    count
}
