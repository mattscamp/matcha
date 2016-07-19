use memcmp::Memcmp;
use std::str;
use super::Match;

pub struct SimpleMatcher;

impl SimpleMatcher {
    /// Finds matches from a vector of bytes and returns a vector of Matches
    pub fn matches(needle: &[u8], haystack: &[u8]) -> Vec<Match> {
        let mut i = 0;
        let mut res = Vec::new();
        let needle_len = needle.len();
        let haystack_len = haystack.len();

        while i < haystack_len {
            if needle[0] == haystack[i] {
                // is the current char == first char?
                if i + needle_len > haystack_len {
                    return res; // This is the end
                }

                if &needle.memcmp(&haystack[i..(i + needle_len)]) == &true {
                    res.push(Match {
                        start: i,
                        end: i + needle_len,
                    });
                    i = i + needle_len - 1;
                }
            }
            i += 1;
        }
        res
    }

    pub fn contains(needle: &[u8], haystack: &[u8]) -> bool {
        let mut i = 0;
        let needle_len = needle.len();
        let haystack_len = haystack.len();

        while i < haystack_len {
            if needle[0] == haystack[i] {
                // is the current char == first char?
                if &needle.memcmp(&haystack[i..(i + needle_len)]) == &true {
                    return true;
                }
            }
            i += 1;
        }
        false
    }
}

#[test]
fn test_simple_matches_len() {
    let s = String::from("test hello there and hello again");
    let needle = String::from("hello");
    let matches: Vec<Match> = SimpleMatcher::matches(needle.as_bytes(), s.as_bytes());
    assert_eq!(matches.len(), 2);
}

#[test]
fn test_simple_matches_pos() {
    let s = String::from("test hello there and hello again");
    let needle = String::from("hello");
    let matches: Vec<Match> = SimpleMatcher::matches(needle.as_bytes(), s.as_bytes());
    assert_eq!(matches[0].start, 5);
    assert_eq!(matches[0].end, 10);
}

#[test]
fn test_simple_out_of_space() {
    let b: Vec<u8> =
        vec![47, 114, 111, 111, 116, 47, 116, 101, 115, 116, 115, 47, 106, 115, 111, 110, 47, 116,
             97, 114, 103, 101, 116, 47, 100, 101, 98, 117, 103, 47, 106, 115, 111, 110, 45, 54,
             98, 52, 56, 52, 99, 51, 102, 57, 49, 56, 54, 102, 50, 101, 98, 46, 100, 58, 32, 115,
             114, 99, 47, 108, 105, 98, 46, 114, 115, 32 ];
    let matches: Vec<Match> = SimpleMatcher::matches("suki".as_bytes(), &b);
}
