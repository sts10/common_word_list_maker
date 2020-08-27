# Common Word List Maker

This program scrapes most commonly used words from Google Books Ngram data in order to create a word list of commonly used words. It scrapes 2012 Google Books Ngram data from this website: [https://storage.googleapis.com/books/ngrams/books/datasetsv3.html](https://storage.googleapis.com/books/ngrams/books/datasetsv3.html).

## How this program works

**Step 1** is to programmatically **scrape** and clean the Google Books Ngram data for words starting with each letter of the alphabet. It sums and merges counts from all years after 1975, and then takes the top 20,000 words for each letter. 

It does this for all 26 letters, producing a separate CSV file for each letter in the "csv" directory. It also adds every letter's words to a CSV file called "all_score_first.csv", which is a CSV where the number of times a word appears in the Google Book data is the first column, and the word itself is the second column.

**Step 2** is to sort the all_score_first CSV file by the number of appearances, cut it to a hard-coded length (say 26,000 words) specified in `src/main.rs`, and print it to a new text file called "word_list_raw.txt". This file is still sorted by the number of appearances, even though that data is not in the file. 

`run.sh` includes all the of the commands to perform steps 1 and 2.

**Step 3** (optional) is to sort and further clean "word_list_raw.txt". We're going to do this with another tool I wrote called Tidy. Install [Tidy](https://github.com/sts10/tidy/) and then you have a few options.

For example, if you want to (1) filter out words NOT in your Mac/Linux OS's dictionary, (2) remove words in reject_words.txt (which is adapted from [this list](https://gist.github.com/micahflee/99809514a6b8556ea4dc)), (3) eliminate prefix words, and (4) ensure all words are lowercase and 4 characters or longer, you'd run: `tidy -o word_list.txt -lpe -m 4 -a /usr/share/dict/words -r reject_words.txt word_list_raw.txt`.

## Running the program 

1. Have Rust installed
2. Run `./run.sh`
3. (Optional): `cargo install --git https://github.com/sts10/tidy && tidy -o word_list.txt -lpe -m 4 -a /usr/share/dict/words -r reject_words.txt word_list_raw.txt`

## On licensing/usage

The Google Books Ngram data compilation I used "is licensed under a [Creative Commons Attribution 3.0 Unported License](http://creativecommons.org/licenses/by/3.0/)" ([source](https://storage.googleapis.com/books/ngrams/books/datasetsv3.html)). This project has no association with Google, nor does Google endorse this project. See LICENSE file for more information.
