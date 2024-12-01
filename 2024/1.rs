use std::fs;

fn part1(list1: &Vec<i32>, list2: &Vec<i32>) -> u32 {
    let mut total: u32 = 0;
    for i in 0..list1.len() {
        let diff: u32 = (list1[i] - list2[i]).abs() as u32;
        total += diff;
    }

    return total;
}

fn part2(list1: &Vec<i32>, list2: &Vec<i32>) -> u32 {
    let mut similarity: u32 = 0;
    for i in 0..list1.len() {
        let mut s = 0_u32;
        for j in 0..list2.len() {
            if list2[j] == list1[i] {
                s += 1;
            }
        }

        similarity += s * list1[i] as u32;
    }

    return similarity;
}

fn main() {
    let list_path = "input/lists.txt";
    let lists = fs::read_to_string(list_path)
        .expect("Should have been able to find list source file");

    let mut list1: Vec<i32> = vec!();
    let mut list2: Vec<i32> = vec!();
    for line in lists.split('\n') {
        let parts: Vec<&str> = line.split("   ").collect();

        if parts.len() == 2 {
            list1.push(parts[0].parse::<i32>().unwrap());
            list2.push(parts[1].parse::<i32>().unwrap());
        }
    }

    println!("List 1 has {} items", list1.len());
    println!("List 2 has {} items", list2.len());

    list1.sort();
    list2.sort();

    let total = part1(&list1, &list2);
    let similarity = part2(&list1, &list2);

    println!("Total difference: {}", total);
    println!("Similarity: {}", similarity);
}
