use std::fs;

fn list_from_file(path: &str) -> Vec<Vec<i32>> {
    let content = fs::read_to_string(path)
        .expect("Unable to find file");

    let lists: Vec<_> = content
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line
            .split(' ')
            .collect::<Vec<_>>()
            .iter()
            .map(|item| item.parse::<i32>().unwrap())
            .collect::<Vec<_>>())
        .collect::<Vec<_>>();

    return lists;
}

fn main() {
    let lists = list_from_file("input/2.txt");
    let mut safe_total: u32 = 0;

    for list in lists {
        let mut is_safe = true;
        let mut is_decreasing = None;

        for i in 1..list.len() {
            let step: i32 = list[i] - list[i-1];
            if step.abs() > 3 || step == 0{
                is_safe = false;
                break;
            }
        
            is_decreasing = match is_decreasing {
                None => { 
                    if step < 0 {
                        Some(true)
                    } else {
                        Some(false)
                    }
                }
                Some(is) => {
                    if step < 0 && !is {
                        is_safe = false;
                        break;
                    } else if step > 0 && is {
                        is_safe = false;
                        break;
                    }

                    Some(is)
                }
            }
        }

        if is_safe {
            safe_total += 1;
        }
    }
    
    println!("{}", safe_total);
}
