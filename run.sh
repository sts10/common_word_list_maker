#!/bin/bash


do_letter() {
  curl -o raw/$1.gz http://storage.googleapis.com/books/ngrams/books/googlebooks-eng-all-1gram-20120701-$1.gz 
  gunzip raw/$1.gz
  cargo run --release -- $1
  rm raw/$1
}

do_letter e
