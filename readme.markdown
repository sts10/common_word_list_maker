# Common Word List Maker

This program scrapes most commonly used words from Google Books Ngram data in order to create a word list of commonly used words. 

It's hard-coded to scrape 2012 Google Books Ngram data from this website: [https://storage.googleapis.com/books/ngrams/books/datasetsv3.html](https://storage.googleapis.com/books/ngrams/books/datasetsv3.html).

**Looking for ready-to-use word lists?** I'd point you to [this separate repo](https://github.com/sts10/generated-wordlists).

## What this program does

**Step 1** is to programmatically **scrape** and clean the Google Books Ngram data for words starting with each letter of the alphabet. It sums and merges counts from all years **after 1975** (a somewhat arbitrary choice on my part), and then takes the top 100,000 words for each letter. 

It does this for all 26 letters, producing a separate CSV file for each letter in the "csv" directory. It also adds every letter's words to a CSV file called "all_score_first.csv", which is a CSV where the number of times a word appears in the Google Book data is the first column, and the word itself is the second column.

**Step 2** is to sort the all_score_first CSV file by the number of appearances, cut it to a hard-coded length (currently 100,000 words) specified in `src/main.rs`, and print it to a new text file called "word_list_raw.txt". Note that this file is sorted by the number of appearances of each word, even though that data is not present in the file. 

`run.sh` includes all the of the necessary commands to perform steps 1 and 2, leaving you with a "raw" word list file located at `./word_list_raw.txt`.

## Creating your own, more usable word list

A list of 100,000 words is likely too long for most uses. It also has one- and two- letter words, as well as indecent words that you may also want to eliminate.

To do this, you'll likely want to make strategic cuts to the word_list_raw.txt file. As a start, you can extract any number of the top-most-appearing words using the `head` command: for example, to write the top 31,000 words to a new file, you could run `head -31000 word_list_raw.txt > cleaned_word_list.txt`. 

For more advanced editing, I'd recommend using another tool I wrote called [Tidy](https://github.com/sts10/tidy/). Once installed, Tidy provides you a few options of how further clean up your word list, which you can read about in the tool's documentation.

As an example, if you want to take the first 31,000 words from `word_list_raw.txt`, then (1) filter out words NOT in your Mac/Linux OS's dictionary, (2) remove words from a text file of "reject words" (you can find such a list [here](https://github.com/zacanger/profane-words)), (3) eliminate [prefix words](https://en.wikipedia.org/wiki/Prefix_code), (4) eliminate words shorter than 4 characters, and (5) ensure all words are lowercase, you'd run: `tidy -o example_word_list.txt --take-first 31000 -lPA -m 4 -a /usr/share/dict/words -r reject_words.txt word_list_raw.txt`.

An example of a "cleaned" word list of 16,607 words that I made using Tidy can be found at `./example_word_list.txt`. More generated word lists can be found in `generated_wordlists/` directory and [in this separate code repo](https://github.com/sts10/generated-wordlists). (As another example, `generated_wordlists/diceware.txt` was made from `./example_word_list.txt` using Tidy, with the command `tidy -D 6 -c 7776 -x 5 --output generated_wordlists/diceware.txt example_word_list.txt`.)

## How to run this program 

Prerequisites: 
* [Rust installed](https://www.rust-lang.org/tools/install)
* `curl`
* `gunzip`

Run `./run.sh`, the end product of which is the "word_list_raw.txt" file.

See above for options to further edit the raw word list.

## On licensing/usage

The Google Books Ngram data compilation I used "is licensed under a [Creative Commons Attribution 3.0 Unported License](http://creativecommons.org/licenses/by/3.0/)" ([source](https://storage.googleapis.com/books/ngrams/books/datasetsv3.html)). This project has no association with Google, nor does Google endorse this project. See LICENSE file for more information on how this project is licensed.
