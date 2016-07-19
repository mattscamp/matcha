use std::cmp;
use std::mem;

use memcmp::Memcmp;

use super::Match;

const CHAR_COUNT: usize = 256;

// The preprocessing function for Boyer Moore's bad character shift
fn bad_character_table(needle: &[u8], len: usize) -> Vec<usize> {
    let mut res = vec![len; CHAR_COUNT];
    let i = 0;
    for i in i..len {
        res[needle[i] as usize] = (len - i - 1) as usize;
    }
    res
}

fn get_md2(needle: &[u8], len: usize) -> usize {
    let mut x: usize = len - 1;
    let mut y: usize = x - 1;

    while x >= 0 {
        if needle[x] == needle[y] {
            break;
        }
        x -= 1;
    }

    (y - x) as usize
}

/// The main BruteForceMatcher type for setting configuration and running BruteForceMatcher.
pub struct TBMMatcher;

impl TBMMatcher {
    /// Finds matches from a vector of bytes and returns a vector of Matches
    pub fn matches(needle: &[u8], haystack: &[u8]) -> Vec<Match> {
        let needle_len = needle.len(); // isize to avoid so many castings
        let haystack_len = haystack.len(); // isize to avoid so many castings

        let mut matches = Vec::new();

        // Our preprocessors
        let mut i = needle_len - 1;
        let mut r: Vec<usize> = bad_character_table(&needle, needle_len);
        let md2 = get_md2(&needle, needle_len);

        'main: while i < haystack_len {
            let mut k = r[haystack[i] as usize];

            while k != 0 {
                i += k;
                if i >= haystack_len {
                    break 'main;
                }
                k = r[haystack[i] as usize];
            }

            if i >= haystack_len || (i + needle_len) > haystack_len {
                break;
            }

            if &needle.memcmp(&haystack[i..(i + needle_len)]) == &false && i < haystack_len {
                matches.push(Match {
                    start: (i + 1 - needle_len),
                    end: (i + 1),
                });
                i += needle_len;
            }

            i += md2;
        }

        matches
    }
}

#[test]
fn test_tbm_matches_len() {
    let s = String::from("test hello there and hello again");
    let needle = String::from("hello");
    let matches: Vec<Match> = TBMMatcher::matches(needle.as_bytes(), s.as_bytes());
    assert_eq!(matches.len(), 2);
}

#[test]
fn test_tbm_matches_pos() {
    let s = String::from("test hello there and hello again");
    let needle = String::from("hello");
    let matches: Vec<Match> = TBMMatcher::matches(needle.as_bytes(), s.as_bytes());
    assert_eq!(matches[0].start, 5);
    assert_eq!(matches[0].end, 10);
}

#[test]
fn test_tbm_out_of_space() {
    let b: Vec<u8> =
        vec![47, 114, 111, 111, 116, 47, 116, 101, 115, 116, 115, 47, 106, 115, 111, 110, 47, 116,
             97, 114, 103, 101, 116, 47, 100, 101, 98, 117, 103, 47, 106, 115, 111, 110, 45, 54,
             98, 52, 56, 52, 99, 51, 102, 57, 49, 56, 54, 102, 50, 101, 98, 46, 100, 58, 32, 115,
             114, 99, 47, 108, 105, 98, 46, 114, 115, 32 ];
    let matches: Vec<Match> = TBMMatcher::matches("suki".as_bytes(), &b);
}

