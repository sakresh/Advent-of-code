use std::io::*;
use std::fs::*;

fn part_1(path: &str) -> u32{
    let file = File::open(path);
    let mut content = String::new();
    let _ = file.unwrap().read_to_string(&mut content);
    let collection = content.lines().collect::<Vec<&str>>();
    let mut array_1: Vec<u32> = vec![];
    let mut array_2: Vec<u32> = vec![];
    for i in collection{
        let part: Vec<&str> = i.split_whitespace().collect();
        array_1.push(part[0].parse::<u32>().unwrap());
        array_2.push(part[1].parse::<u32>().unwrap());
    }
    array_1.sort();
    array_2.sort();
    let mut res: u32 = 0;
    for i in 0..array_1.len(){
        if array_1[i] > array_2[i]{
            res += array_1[i] - array_2[i];
            continue;
        }
        else{
            res += array_2[i] - array_1[i];
            continue;
        }
    }
    res
}

fn main() -> Result<()>{
    println!("{}", part_1("inputs/input-1.txt"));
    Ok(())
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn case_1(){
        assert_eq!(part_1("inputs/sample.txt"), 11);
    }

    #[test]
    fn case_2(){
        assert_eq!(part_1("inputs/input-2.txt"), 2066446);
    }
}
