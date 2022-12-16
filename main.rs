use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let path:&str=r"C:\Users\Ellis\Documents\ds210final\gplus_combined.txt";
    let file = read_file(path);
    let file_hash=create_hashmap(file);
    let longest_key_result=longest_key(&file_hash);
    println!("The key with the most connections is {}",longest_key_result.0);
    println!("The number of connections is {}", longest_key_result.1);
}

// read the file 

fn read_file(path: &str) -> Vec<(u128,u128)>{
    let mut result: Vec<(u128, u128)> = Vec::new();
    let file = File::open(path).expect("Could not open file");
    let buf_reader = std::io::BufReader::new(file).lines();
    for line in buf_reader {
        let line_str = line.expect("Error reading");
        let v: Vec<&str> = line_str.trim().split(' ').collect();
        let x = v[0].parse::<u128>().unwrap();
        let y = v[1].parse::<u128>().unwrap();
        result.push((x, y));
    }
    return result;
}

// create hashmap for nodes 
fn create_hashmap(vec:Vec<(u128,u128)>)-> HashMap<u128,Vec<u128>>{
    let mut hm=HashMap::<u128,Vec<u128>>::new();
    for t in vec{
        // check if t.0 is already a key 
        if hm.contains_key(&t.0){
            hm.entry(t.0).or_default().push(t.1);
        } else { 
        // if not, create a new key and insert the pair 
        hm.insert(t.0,Vec::new());
        hm.entry(t.0).or_default().push(t.1);
        }
    }
    return hm;
}

//hm.entry(key:t.0).or_default().push(t.1)

// which key has the longest length vector 
fn longest_key(hm: &HashMap<u128,Vec<u128>>)->(u128,usize){
    let mut max:usize=0; 
    let mut key_max:u128=0;
    // iterate through hashmap
    for (key,val) in hm.iter(){
        if val.len()>max{
            key_max=*key;
            max=val.len();
        } 
    }
    return (key_max, max)
}

