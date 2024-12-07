use std::fs;
use std::io::*;

fn compare(vec: &[u32]) -> bool{
    let value: Vec<u32> = vec.windows(2).
        map(|x| {
            if x[0] > x[1]{
                x[0] - x[1]
            }
            else{
                x[1] - x[0]
            }
        }).
        collect();
    let res: bool = value.iter().all(|x| *x<=3 && *x>=1);
    res
}

fn is_sorted_function(vec: &[u32]) -> bool{
    if vec.windows(2).all(|x| x[0] > x[1]) || vec.windows(2).all(|x| x[0] < x[1]){
        return true;
    }
    false
}

fn part_1(path: &str) -> u32{
    let content = fs::read_to_string(path);
    let bind = content.unwrap();
    let vector: Vec<&str> = bind.lines().collect();
    let mut sum = 0;
    for i in 0..vector.len(){
        let temp: Vec<&str> = vector[i].split_whitespace().collect();
        let values: Vec<u32> = temp
            .into_iter()
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        if is_sorted_function(&values) && compare(&values){
            sum += 1;
        }
    }
    sum
}

fn main() -> Result<()>{
    let res: u32 = part_1("inputs/input-1.txt");
    println!("{res}");
    Ok(())
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn check1(){
        assert_eq!(part_1("inputs/sample.txt"), 2);
    }

    #[test]
    fn check2(){
        assert_eq!(part_1("inputs/input-1.txt"), 257);
    }
}
