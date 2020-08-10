use std::collections::HashMap;
use std::fs::File;
// use std::io::prelude::Read;
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
    let tsv_file = "/home/sschlinkert/Downloads/googlebooks-eng-all-1gram-20120701-a";
    print_records_from_tsv(PathBuf::from(tsv_file));
}

fn print_records_from_tsv(file_path: PathBuf) {
    // let mut entries: Vec<Entry> = vec![];
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
        // println!("This record is {:?}", record);
        let word = clean_word(record[0].to_string());
        let this_count = record[2].to_string().parse().unwrap();
        counts_hashmap
            .entry(word)
            .and_modify(|count| *count += this_count)
            .or_insert(this_count);
    }
    // println!("{:?}", counts_hashmap);
    // convert to a Vector of Tuples and sort it by appearance count
    let mut count_vec: Vec<(String, usize)> = counts_hashmap.into_iter().collect();
    count_vec.sort_by(|a, b| a.1.cmp(&b.1));
    // count_vec.reverse();
    for pair in &count_vec {
        println!("{:?}", pair);
    }
}

fn clean_word(w: String) -> String {
    w.split("_").collect::<Vec<&str>>()[0]
        .to_string()
        .split(".")
        .collect::<Vec<&str>>()[0]
        .to_string()
        .to_lowercase()
}
