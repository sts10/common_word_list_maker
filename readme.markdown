# Google Books Ngram Word List Builder

This program scrapes most commonly used words from Google Books Ngram data in order to create a word list of commonly used words. It scrapes 2012 Google Books Ngram data from this website: [https://storage.googleapis.com/books/ngrams/books/datasetsv3.html](https://storage.googleapis.com/books/ngrams/books/datasetsv3.html).

## How this program works

`run.sh` lists all the commands for a full run-through of the program. 

Step 1 is to programmatically **scrape** and clean the Google Books Ngram data for each letter of the alphabet. It does this for all 26 letters, producing a separate CSV file for each letter in the "csv" directory. It also adds that letter's words to a CSV file called "all_score_first.csv", which is a CSV where the number of times a word appears in the Google Book data is the first column.

Step 2 is to sort the all_score_first CSV file by the number of appearances, cut it to a hard-coded length (say 26,000 words) specified in `src/main.rs`, and print it to a new text file called "word_list.txt". This file is still sorted by the number of appearances, even though that data is not in the file. Of course you're welcome to sort it alphabetically later.

## Running the program 

1. Have Rust installed
2. Run `./run.sh`

## Tools for further working on the produced word list

If you'd like to easily do more manipulations of the resulting word list file, such as remove prefix words, I'd recommend another tool I built called [Tidy](https://github.com/sts10/tidy/).

## On licensing/usage

The Google Books Ngram data compilation I used "is licensed under a [Creative Commons Attribution 3.0 Unported License](http://creativecommons.org/licenses/by/3.0/)" ([source](https://storage.googleapis.com/books/ngrams/books/datasetsv3.html)). This project has no association with Google, nor does Google endorse this project. See LICENSE file for more information.
