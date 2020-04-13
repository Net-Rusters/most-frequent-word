
use clap;
use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::path::Path;
use std::collections::HashMap;

fn main() {
    let matches = clap::App::new("most-frequent")
                                .version("1.0.0")
                                .author("Yilmaz Baris Kaplan")
                                .about("Returns the most frequent word in a text file")
                                .arg(clap::Arg::with_name("INPUT")
                                    .help("The text file that the most frequent word will be found")
                                    .required(true))
                                .get_matches();
    let input = matches.value_of("INPUT").unwrap();

    let path = Path::new(input);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("Could not open the file {}", display),
        Ok(file) => file,
    };

    let reader = BufReader::new(file);

    let mut words: HashMap<String, u32> = HashMap::new();
    let mut max_count = 0u32;
    
    for line in reader.lines() {
        if let Ok(line_string) = line {
            let mut word_string = line_string.to_string();
            let mut split = word_string.split(" ");
            for s in split {
                let s_string = s.to_string();
                let count = words.entry(s_string).or_insert(0);
                *count += 1;
                if *count > max_count {
                    max_count = *count;
                }
            }
        }
    }

    println!("The frequency of most frequent words are {}.", max_count);
    println!("Words: ");

    for (word, count) in words.iter() {
        if *count == max_count {
            println!("{}", word);
        }
    }
}

