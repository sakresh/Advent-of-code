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
    for i in 0..contents.len(){
        res = regular_expression.
            find_iter(contents[i]).
            map(|x| x.as_str()).
            map(|x| x.parse::<u32>().unwrap()).
            collect();
        sum += res.iter().product::<u32>();
    }
    sum
}

fn contains(input: &str) -> Vec<&str>{
    let res: Vec<&str>;
    // Regular expression for part-2 has to be written
    let regular_expression = regex::Regex::new(r"mul\(\d+,\s*\d+\)").unwrap();
    res = regular_expression.
        find_iter(input).
        map(|x| x.as_str()).
        collect();
    res
}

fn main() -> io::Result<()>{
    let path = "inputs/input-1.txt";
    println!("{:?}", compute(path));
    Ok(())
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn check1(){
        assert_eq!(compute("inputs/sample.txt"), 161);
    }

    #[test]
    fn check2(){
        assert_eq!(compute("inputs/input-1.txt"), 166630675);
    }

}
