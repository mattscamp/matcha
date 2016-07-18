#![feature(test)]
extern crate memcmp;

#[derive(Debug,Clone)]
pub struct Match {
    pub start: usize,
    pub end: usize,
}

pub mod simple;