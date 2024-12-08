use std::io;
use std::fs;
use regex;

fn compute(input: &str) -> u32{
    let file_contents = fs::read_to_string(input);
    let bind = file_contents.unwrap();
    let contents = contains(&bind);
    let mut res: Vec<u32>;
    let regular_expression = regex::Regex::new(r"\d+").unwrap();
    let mut sum: u32 = 0;
    let mut flag = true;
    for i in 0..contents.len(){
        if contents[i] == "don\'t()"{
            flag = false;
            continue;
        }
        if contents[i] == "do()"{
            flag = true;
            continue;
        }
        if flag{
            res = regular_expression.
                find_iter(contents[i]).
                map(|x| x.as_str()).
                map(|x| x.parse::<u32>().unwrap()).
                collect();
            sum += res.iter().product::<u32>();
        }
    }
    sum
}

fn contains(input: &str) -> Vec<&str>{
    let res: Vec<&str>;
    let regular_expression = regex::Regex::new(r"(mul\(([0-9]+),([0-9]+)\)|do\(\)|don't\(\))").unwrap();
    res = regular_expression.
        find_iter(input).
        map(|x| x.as_str()).
        collect();
    res
}

fn main() -> io::Result<()>{
    let path = "inputs/input-2.txt";
    println!("{}", compute(path));
    Ok(())
}

mod test{
#[cfg(test)]
    use super::*;

    #[test]
    fn check1(){
        assert_eq!(compute("inputs/sample-2.txt"), 48);
    }

    #[test]
    fn check2(){
        assert_eq!(compute("inputs/input-2.txt"), 93465710);
    }

}
