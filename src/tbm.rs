use std::cmp;
use std::mem;

use memcmp::Memcmp;

use super::Match;

const CHAR_COUNT: usize = 256;

// The preprocessing function for Boyer Moore's bad character shift
fn bad_character_table(needle: &[u8], len: usize) -> Vec<usize> {
    let mut res = vec![len; CHAR_COUNT];
    for i in 0..len {
        res[needle[i] as usize] = (len - i - 1) as usize;
    }
    res
}

fn get_md2(needle: &[u8], len: usize) -> usize {
    let x: isize = len as isize - 1;
    let mut y: isize = x - 1;

    while y >= 0 {
        if needle[x as usize] == needle[y as usize] {
            break;
        }
        y -= 1;
    }
    (x - y) as usize
}

/// The main TBMMatcher type for setting configuration and running TBMMathcher.
pub struct TBMMatcher;

impl TBMMatcher {
    /// Finds matches from a vector of bytes and returns a vector of Matches
    pub fn matches(needle: &[u8], haystack: &[u8]) -> Vec<Match> {
        let needle_len = needle.len();
        let haystack_len = haystack.len();

        let mut matches = Vec::new();

        // Our preprocessors
        let mut i = 0;
        let mut r: Vec<usize> = bad_character_table(&needle, needle_len);
        let shift = get_md2(&needle, needle_len);

        'outer: while (i + needle_len - 1) < haystack_len {
            let mut k = r[haystack[i + needle_len - 1] as usize];

            'inner: while k != 0 {
                i += k;
                if (i + needle_len - 1) >= haystack_len {
                    break 'outer;
                }
                k = r[haystack[i + needle_len - 1] as usize];
            }


            if i >= haystack_len || (i + needle_len) > haystack_len {
                break;
            }

            if &needle.memcmp(&haystack[i..(i + needle_len)]) == &true && i < haystack_len {
                matches.push(Match {
                    start: i,
                    end: (i + needle_len),
                });
                i += needle_len;
                continue;
            }

            i += shift;
        }

        matches
    }
}

#[test]
fn test_tbm_matches_len() {
    let s = String::from("test hello there and hello again test");
    let needle = String::from("hello");
    let matches: Vec<Match> = TBMMatcher::matches(needle.as_bytes(), s.as_bytes());
    assert_eq!(matches.len(), 2);
}

#[test]
fn test_tbm_matches_pos() {
    let s = String::from("test hello there and hello again test");
    let needle = String::from("hello");
    let matches: Vec<Match> = TBMMatcher::matches(needle.as_bytes(), s.as_bytes());
    assert_eq!(matches[0].start, 5);
    assert_eq!(matches[0].end, 10);
}

#[test]
fn test_tbm_no_matches() {
    let s = String::from("test hello there and hello again");
    let needle = String::from("suki");
    let matches: Vec<Match> = TBMMatcher::matches(needle.as_bytes(), s.as_bytes());
    assert_eq!(matches.len(), 0);
}
