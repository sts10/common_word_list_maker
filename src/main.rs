use std::collections::HashMap;
// use std::error::Error;
// use std::ffi::OsString;
use std::fs::File;
// use std::io;
// use std::process;
// use std::io::prelude::Read;
use std::env;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Entry {
    pub word_raw: String,
    pub word_parsed: String,
    pub year: usize,
    pub appearances: usize,
    pub unique_appearances: usize,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let letter = &args[1];
    let tsv_file = format!(
        "/home/sschlinkert/Downloads/googlebooks-eng-all-1gram-20120701-{}",
        letter,
    );
    let counts_vec = make_counts_vec_from_tsv_file_path(PathBuf::from(tsv_file));
    println!("Made counts vec. Now just writing it to a csv");
    write_count_vec_to_csv(counts_vec, letter);
    println!("Done printing csv for {}", letter);
}

fn make_counts_vec_from_tsv_file_path(file_path: PathBuf) -> Vec<(String, usize)> {
    let mut counts_hashmap: HashMap<String, usize> = HashMap::new();

    let file = match File::open(file_path) {
        Ok(f) => f,
        Err(_) => {
            panic!("Error reading:");
        }
    };
    let mut rdr = csv::ReaderBuilder::new().delimiter(b'\t').from_reader(file);
    // Loop over each record
    for result in rdr.records() {
        let record = match result {
            Ok(rec) => rec,
            Err(e) => {
                panic!(
                    "Error reading a line of the specified CSV file: {}. Aborting.",
                    e
                );
            }
        };
        if record[1].parse::<usize>().unwrap() > 1975 {
            let word = clean_word(record[0].to_string());
            // record[2] is overall appaearances; record[3] is the number of distinct samples (books) it
            // appeared in https://storage.googleapis.com/books/ngrams/books/datasetsv3.html
            let this_count = record[2].to_string().parse().unwrap();
            counts_hashmap
                .entry(word)
                .and_modify(|count| *count += this_count)
                .or_insert(this_count);
        }
    }
    // println!("{:?}", counts_hashmap);
    // convert to a Vector of Tuples and sort it by appearance count
    let mut count_vec: Vec<(String, usize)> = counts_hashmap.into_iter().collect();
    count_vec.sort_by(|a, b| a.1.cmp(&b.1));
    count_vec.reverse();
    for pair in &count_vec {
        println!("{:?}", pair);
    }
    println!("vector size for letter a is {}", count_vec.len());
    count_vec.drain(10000..);
    println!("drained: vector size for letter a is {}", count_vec.len());
    count_vec
}

fn write_count_vec_to_csv(counts_vec: Vec<(String, usize)>, letter: &str) {
    let file_path = format!("csv/{}.csv", letter);
    let mut wtr = csv::Writer::from_path(file_path).unwrap();

    for word in counts_vec {
        wtr.write_record(&[word.0, word.1.to_string()]).unwrap();
    }
    // wtr.flush();
    wtr.flush().unwrap();
}

fn clean_word(w: String) -> String {
    w.split("_").collect::<Vec<&str>>()[0]
        .to_string()
        .split(".")
        .collect::<Vec<&str>>()[0]
        .to_string()
        .to_lowercase()
}
