// Given a list of integers, use a vector and return the mean (the average value),
// median (when sorted, the value in the middle position),
// and mode (the value that occurs most often; a hash map will be helpful here) of the list.

use std::collections::HashMap;
use std::io;

fn main() {
    println!("Enter a list of integers separated by spaces, hit enter when done");
    let mut nums: Vec<i32> = Vec::new();
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Unable to read line");
    // remove trailing new line

    let mut strs : Vec<&str> = input.split(" ").collect();
    println!("{:?}",strs);
    for s in strs.iter(){
        nums.push(s.trim().parse::<i32>().expect("Unable to parse num"));
    }
    println!("MEAN:{}",mean(&nums));
    println!("MEDIAN:{}",median(&mut nums));
    println!("MODE:{}",mode(&nums));

}

fn mean(nums : &Vec<i32>) -> f32{
    let mut sum : f32 = 0.0;
    for i in nums.iter(){

        sum += *i as f32;
    }
    return sum / nums.len() as f32;
}

fn median(nums: &mut Vec<i32>) -> i32{
    nums.sort();
    return nums[nums.len()/2];
}

fn mode(nums: &Vec<i32>) -> i32{
    let mut map : HashMap<i32,i32> = HashMap::new();
    for i in nums{
        let entry = map.entry(*i).or_insert(0);
        *entry += 1;
    }

    let mut top : Option<&i32> = Some(&0);
    let mut val : i32 = 0;
    for k in map.keys(){
        if map.get(k) > top{
            top = map.get(k);
            val = *k;
        }
    }
    return val;
}
