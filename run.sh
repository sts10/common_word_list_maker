#!/bin/bash


do_letter() {
  echo $1
  curl -o raw/$1.gz http://storage.googleapis.com/books/ngrams/books/googlebooks-eng-all-1gram-20120701-$1.gz 
  gunzip raw/$1.gz
  cargo run --release -- $1
  rm raw/$1
}

do_letter a
do_letter b
do_letter c
do_letter d
do_letter e
do_letter f
do_letter g
do_letter h
do_letter i
do_letter j
do_letter k
do_letter l
do_letter m
do_letter o
do_letter p
do_letter q
do_letter r
do_letter s
do_letter t
do_letter u
do_letter v
do_letter w
do_letter x
do_letter y
do_letter z

# Make the word list (no argument provided is what tells the Rust code 
# that that's what we want to do)
cargo run --release
