use std::io::*;
use std::fs::*;

fn part_2(path: &str) -> i32{
    let file = File::open(path);
    let mut content = String::new();
    let _ = file.unwrap().read_to_string(&mut content);
    let collection = content.lines().collect::<Vec<&str>>();
    let mut array_1: Vec<i32> = vec![];
    let mut array_2: Vec<i32> = vec![];
    for i in collection{
        let part: Vec<&str> = i.split_whitespace().collect();
        array_1.push(part[0].parse::<i32>().unwrap());
        array_2.push(part[1].parse::<i32>().unwrap());
    }
    let mut res: i32 = 0;
    for i in 0..array_1.len(){
        let mut counter: i32 = 0;
        for j in 0..array_2.len(){
            if array_1[i] == array_2[j]{
                counter += 1;
            }
        }
        res += counter * array_1[i];
    }
    res
}

fn main() -> Result<()>{
    println!("{}", part_2("inputs/input-2.txt"));
    Ok(())
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn case_1(){
        assert_eq!(part_2("inputs/sample.txt"), 31);
    }

    #[test]
    fn case_2(){
        assert_eq!(part_2("inputs/input-2.txt"), 24931009);
    }
}
