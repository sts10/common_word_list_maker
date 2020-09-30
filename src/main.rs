use csv::Reader;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
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
    println!("Arguments recieved: {:?}", args);
    if args.len() > 1 && !args[1].contains("csv") {
        // If we recieved any arguments, assume it's a letter and we're to parse a
        // TSV file of one letter's worth of Google Booke Ngram data
        let letter = &args[1];
        let tsv_file = format!("raw/{}", letter);
        // Make a Vector of tuples, where each tuple contains the word and its number of
        // appearances
        let counts_vec = make_counts_vec_from_tsv_file_path(PathBuf::from(tsv_file));
        println!("Made counts vec. Now appending that data to all_score_first CSV file.");
        // Append this data to a big CSV file that will eventually contain this data
        // for all 26 letters of the alphabet. CSV file will be called
        // all_score_first.csv, named that since the number of appearances (the
        // "score") is listed first
        append_count_data_to_full_csv_file(counts_vec);
        println!(
            "Done appending letter {} data to all_score_first csv file",
            letter
        );
    } else {
        println!("Did not receive a letter to process...");
        println!("Creating a raw word list of top 100,000 words, sorted by appearance count, and writing the list to a 
            text file called word_list_raw.txt");

        // Since the data in the all_score_first CSV file isn't sorted by number of
        // appearances, we have to do that in a separate step
        let all_counts_vec = make_sorted_counts_vec_from_complete_csv("csv/all_score_first.csv");
        // We'll now use this handy Vector to create a word list of the top 100,000 words, sorted
        // by appearance count
        // This word list will be printed to a new text file called word_list_raw.txt
        make_raw_word_list_from_counts_vec(all_counts_vec);
    }
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
            // record[2] is overall appearances; record[3] is the number of distinct samples (books) it
            // appeared in https://storage.googleapis.com/books/ngrams/books/datasetsv3.html
            let this_count = record[2].to_string().parse().unwrap();
            counts_hashmap
                .entry(word)
                .and_modify(|count| *count += this_count)
                .or_insert(this_count);
        }
    }

    // convert to a Vector of Tuples and sort it by appearance count
    let mut count_vec: Vec<(String, usize)> = counts_hashmap.into_iter().collect();
    count_vec.sort_by(|a, b| a.1.cmp(&b.1));
    count_vec.reverse();
    println!(
        "Before potentially draining, the vector size of words beginning with this letter is {}",
        count_vec.len()
    );
    if count_vec.len() > 100_000 {
        count_vec.drain(100_000..);
        println!(
            "After draining, vector size for this letter is {}",
            count_vec.len()
        );
    }
    count_vec
}

fn append_count_data_to_full_csv_file(counts_vec: Vec<(String, usize)>) {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open("csv/all_score_first.csv")
        .unwrap();
    let mut wtr_by_score = csv::Writer::from_writer(file);

    for word in counts_vec {
        wtr_by_score
            .write_record(&[word.1.to_string(), word.0])
            .unwrap();
    }
    wtr_by_score.flush().unwrap();
}

fn clean_word(w: String) -> String {
    w.split("_").collect::<Vec<&str>>()[0]
        .to_string()
        .split(".")
        .collect::<Vec<&str>>()[0]
        .to_string()
        .to_lowercase()
}

fn make_sorted_counts_vec_from_complete_csv(file_path: &str) -> Vec<(String, usize)> {
    let mut full_counts_hashmap: HashMap<String, usize> = HashMap::new();

    let mut rdr = Reader::from_path(file_path).expect("Error reading CSV file");
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
        let word = record[1].to_string();
        let this_count = record[0].to_string().parse().unwrap();
        full_counts_hashmap
            .entry(word)
            .and_modify(|count| *count += this_count)
            .or_insert(this_count);
    }
    // convert to a Vector of Tuples and sort it by appearance count
    let mut full_count_vec: Vec<(String, usize)> = full_counts_hashmap.into_iter().collect();
    full_count_vec.sort_by(|a, b| a.1.cmp(&b.1));
    full_count_vec.reverse();
    full_count_vec
}

fn make_raw_word_list_from_counts_vec(full_count_vec: Vec<(String, usize)>) {
    let mut list: Vec<String> = Vec::new();
    let words_to_print = 100_000;

    let mut i = 0;
    for word_info in full_count_vec {
        list.push(word_info.0);
        i += 1;
        if i > words_to_print {
            break;
        }
    }
    let mut f = File::create("word_list_raw.txt").expect("Unable to create file");
    for word in &list {
        writeln!(f, "{}", word).expect("Unable to write data to file");
    }
}
