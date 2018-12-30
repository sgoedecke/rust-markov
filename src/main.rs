extern crate rand;

use rand::Rng;
use std::fs;
use std::env;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename)
        .expect("Could not read file");

    let first_words: Vec<&str> = contents.trim().split('.')
        .collect::<Vec<&str>>()
        .iter()
        .map(|s| s.trim().split(' ').next().unwrap())
        .filter(|s| s != &"")
        .collect();

    let mut lookup_table: HashMap<&str, Vec<&str>> = HashMap::new();
    let words: Vec<&str> = contents.trim().split(' ').collect();
    for (i, word) in words.iter().enumerate() {
        if i+1 < words.len() { // TODO: guard clause
            lookup_table.entry(word).or_insert([].to_vec()).push(words[i+1]);
        }
    }

    let mut output = String::from("");
    for _x in 0..10 {
        output.push_str(&generate_sentence(&first_words, &lookup_table));
        output.push(' ');
    }
    println!("{}", output);
}

fn generate_sentence(first_words: &Vec<&str>, lookup_table: &HashMap<&str, Vec<&str>>) -> String {
    let mut sentence: Vec<&str> = [].to_vec();
    sentence.push(rand::thread_rng().choose(&first_words).unwrap());
    loop {
        let last_word = sentence.last().unwrap();
        if last_word.chars().last().unwrap() == '.' {
            break
        }
        let candidates = lookup_table.get(last_word).unwrap();
        sentence.push(rand::thread_rng().choose(&candidates).unwrap());
    }
    sentence.join(&String::from(" "))
}
