#![allow(unused)]
use clap::Parser;

// Style in which to parse
#[derive(Parser)]
struct CLI {
    // first number
    arg1: f64,
    // operation to do
    operation: &str,
    // second number
    arg2: f64,
}


