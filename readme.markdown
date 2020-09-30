# Common Word List Maker

This program scrapes most commonly used words from Google Books Ngram data in order to create a word list of commonly used words. 

It's hard-coded to scrape 2012 Google Books Ngram data from this website: [https://storage.googleapis.com/books/ngrams/books/datasetsv3.html](https://storage.googleapis.com/books/ngrams/books/datasetsv3.html).

## What this program does

**Step 1** is to programmatically **scrape** and clean the Google Books Ngram data for words starting with each letter of the alphabet. It sums and merges counts from all years after 1975, and then takes the top 100,000 words for each letter. 

It does this for all 26 letters, producing a separate CSV file for each letter in the "csv" directory. It also adds every letter's words to a CSV file called "all_score_first.csv", which is a CSV where the number of times a word appears in the Google Book data is the first column, and the word itself is the second column.

**Step 2** is to sort the all_score_first CSV file by the number of appearances, cut it to a hard-coded length (currently 100,000 words) specified in `src/main.rs`, and print it to a new text file called "word_list_raw.txt". Note that this file is sorted by the number of appearances of each word, even though that data is not present in the file. 

`run.sh` includes all the of the necessary commands to perform steps 1 and 2, leaving you with a "raw" word list file located at `./word_list_raw.txt`.

## Further editing of your word list

A list of 100,000 words is likely too long for most uses. It also has one- and two- letter words that you may also want to eliminate, as well as indecent words. 

Thus, at this point you may want to create your own word list from this word_list_raw.txt file. As a start, you can extract any number of the top-most-appearing words by running `head -30000 word_list_raw.txt > cleaned_word_list.txt`. 

For more advanced editing, I'd recommend using another tool I wrote called Tidy. Once [installed](https://github.com/sts10/tidy/), Tidy provides you a few options of how to create your own, cleaned-up list.

You can read more about these options in Tidy's documentation, but as an example, if you want to (1) filter out words NOT in your Mac/Linux OS's dictionary, (2) remove words in the provided reject_words.txt file (which is adapted from [this list](https://gist.github.com/micahflee/99809514a6b8556ea4dc)), (3) eliminate prefix words, (4) eliminate words shorter than 4 characters, and (5) ensure all words are lowercase, you'd run: `tidy -o word_list.txt -lpe -m 4 -a /usr/share/dict/words -r reject_words.txt word_list_raw.txt`.

## Running the program 

Assuming you have [Rust installed](https://www.rust-lang.org/tools/install)...

1. Run `./run.sh`, the end product of which is the "word_list_raw.txt" file.
2. (Optional): `cargo install --git https://github.com/sts10/tidy && tidy -o word_list.txt -lpe -m 4 -a /usr/share/dict/words -r reject_words.txt word_list_raw.txt`, which creates a cleaned-up "word_list.txt"

## On licensing/usage

The Google Books Ngram data compilation I used "is licensed under a [Creative Commons Attribution 3.0 Unported License](http://creativecommons.org/licenses/by/3.0/)" ([source](https://storage.googleapis.com/books/ngrams/books/datasetsv3.html)). This project has no association with Google, nor does Google endorse this project. See LICENSE file for more information on how this project is licensed.
