#!/bin/bash

rm word_list_raw.txt
rm -rf csv
rm -rf raw
mkdir csv
mkdir raw

scrape_and_process_letter() {
  echo $1
  curl -o raw/$1.gz http://storage.googleapis.com/books/ngrams/books/googlebooks-eng-all-1gram-20120701-$1.gz 
  gunzip raw/$1.gz
  cargo run --release -- $1
  rm raw/$1
}

scrape_and_process_letter a
scrape_and_process_letter b
scrape_and_process_letter c
scrape_and_process_letter d
scrape_and_process_letter e
scrape_and_process_letter f
scrape_and_process_letter g
scrape_and_process_letter h
scrape_and_process_letter i
scrape_and_process_letter j
scrape_and_process_letter k
scrape_and_process_letter l
scrape_and_process_letter m
scrape_and_process_letter o
scrape_and_process_letter p
scrape_and_process_letter q
scrape_and_process_letter r
scrape_and_process_letter s
scrape_and_process_letter t
scrape_and_process_letter u
scrape_and_process_letter v
scrape_and_process_letter w
scrape_and_process_letter x
scrape_and_process_letter y
scrape_and_process_letter z

# Make the raw word list (no argument provided is what tells the Rust code 
# that that's what we want to do)
cargo run --release
