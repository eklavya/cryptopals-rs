use itertools::Itertools;
use std::collections::HashSet;

pub fn log_weight_score(inp: &str) -> f32 {
    inp.chars().fold(0.0, |acc, c| acc + log_weight(c))
}

pub fn char_freq_score(inp: &str) -> u64 {
    let common = ['e', 't', 'a', 'o', 'i', 'n', 's', 'h', 'r', 'd', 'l', 'u']
        .iter()
        .collect::<HashSet<_>>();
    let allowed = ['\'', '\"', ',', '.', '?', ' ', '\n']
        .iter()
        .collect::<HashSet<_>>();
    let score = inp.to_ascii_lowercase().chars().fold(0, |score, c| {
        // println!(" char is {} score is {}", c, if common.contains(&c) {1} else {0} - if c.is_ascii_alphanumeric() {0} else {1});
        score + {
            if common.contains(&c) {
                1
            } else {
                0
            }
        } - {
            if c.is_ascii_alphanumeric() || allowed.contains(&c) {
                0
            } else {
                1
            }
        }
    });
    if score < 0 {
        0
    } else {
        score as u64
    }
}

pub fn bigram_score(inp: &str) -> f32 {
    inp.chars()
        .tuple_windows()
        .fold(0.0, |acc, (c1, c2)| acc + bigram_weights(&[c1, c2]))
}

pub fn eng_socre(inp: &str) -> u64 {
    let log_w_score = log_weight_score(inp);
    let bigram_score = bigram_score(inp);
    let score = log_w_score + bigram_score;
    if score.is_sign_negative() {
        0
    } else {
        score as u64
    }
}

const fn bigram_weights(s: &[char; 2]) -> f32 {
    match s {
        ['a', 'a'] => 0.0,
        ['a', 'b'] => 1.5,
        ['a', 'c'] => 1.6,
        ['a', 'd'] => 1.2,
        ['a', 'e'] => 0.0,
        ['a', 'f'] => 1.0,
        ['a', 'g'] => 1.3,
        ['a', 'h'] => 0.0,
        ['a', 'i'] => 1.2,
        ['a', 'j'] => 0.0,
        ['a', 'k'] => 1.0,
        ['a', 'l'] => 1.9,
        ['a', 'm'] => 1.3,
        ['a', 'n'] => 2.2,
        ['a', 'o'] => 0.3,
        ['a', 'p'] => 1.5,
        ['a', 'q'] => 0.0,
        ['a', 'r'] => 2.0,
        ['a', 's'] => 1.8,
        ['a', 't'] => 2.1,
        ['a', 'u'] => 1.1,
        ['a', 'v'] => 1.4,
        ['a', 'w'] => 0.8,
        ['a', 'x'] => 0.0,
        ['a', 'y'] => 1.4,
        ['a', 'z'] => 0.0,

        ['b', 'a'] => 0.9,
        ['b', 'b'] => 0.0,
        ['b', 'c'] => 0.0,
        ['b', 'd'] => 0.0,
        ['b', 'e'] => 1.8,
        ['b', 'f'] => 0.0,
        ['b', 'g'] => 0.0,
        ['b', 'h'] => 0.0,
        ['b', 'i'] => 0.8,
        ['b', 'j'] => 0.3,
        ['b', 'k'] => 0.0,
        ['b', 'l'] => 1.3,
        ['b', 'm'] => 0.0,
        ['b', 'n'] => 0.0,
        ['b', 'o'] => 1.0,
        ['b', 'p'] => 0.0,
        ['b', 'q'] => 0.0,
        ['b', 'r'] => 0.8,
        ['b', 's'] => 0.7,
        ['b', 't'] => 0.0,
        ['b', 'u'] => 1.4,
        ['b', 'v'] => 0.0,
        ['b', 'w'] => 0.0,
        ['b', 'x'] => 0.0,
        ['b', 'y'] => 1.3,
        ['b', 'z'] => 0.0,

        ['c', 'a'] => 1.6,
        ['c', 'b'] => 0.0,
        ['c', 'c'] => 1.1,
        ['c', 'd'] => 0.0,
        ['c', 'e'] => 1.7,
        ['c', 'f'] => 0.0,
        ['c', 'g'] => 0.0,
        ['c', 'h'] => 1.7,
        ['c', 'i'] => 1.2,
        ['c', 'j'] => 0.0,
        ['c', 'k'] => 0.9,
        ['c', 'l'] => 1.2,
        ['c', 'm'] => 0.0,
        ['c', 'n'] => 0.0,
        ['c', 'o'] => 1.8,
        ['c', 'p'] => 0.0,
        ['c', 'q'] => 0.0,
        ['c', 'r'] => 0.8,
        ['c', 's'] => 0.0,
        ['c', 't'] => 1.6,
        ['c', 'u'] => 1.2,
        ['c', 'v'] => 0.0,
        ['c', 'w'] => 0.0,
        ['c', 'x'] => 0.0,
        ['c', 'y'] => 0.0,
        ['c', 'z'] => 0.0,

        ['d', 'a'] => 1.7,
        ['d', 'b'] => 1.3,
        ['d', 'c'] => 0.6,
        ['d', 'd'] => 1.0,
        ['d', 'e'] => 1.6,
        ['d', 'f'] => 1.1,
        ['d', 'g'] => 0.3,
        ['d', 'h'] => 0.5,
        ['d', 'i'] => 1.8,
        ['d', 'j'] => 0.0,
        ['d', 'k'] => 0.0,
        ['d', 'l'] => 0.8,
        ['d', 'm'] => 1.0,
        ['d', 'n'] => 0.7,
        ['d', 'o'] => 1.6,
        ['d', 'p'] => 0.8,
        ['d', 'q'] => 0.0,
        ['d', 'r'] => 1.0,
        ['d', 's'] => 1.5,
        ['d', 't'] => 1.6,
        ['d', 'u'] => 0.9,
        ['d', 'v'] => 0.6,
        ['d', 'w'] => 1.0,
        ['d', 'x'] => 0.0,
        ['d', 'y'] => 0.8,
        ['d', 'z'] => 0.0,

        ['e', 'a'] => 2.1,
        ['e', 'b'] => 1.0,
        ['e', 'c'] => 1.8,
        ['e', 'd'] => 2.0,
        ['e', 'e'] => 1.6,
        ['e', 'f'] => 1.4,
        ['e', 'g'] => 1.3,
        ['e', 'h'] => 1.2,
        ['e', 'i'] => 1.6,
        ['e', 'j'] => 0.0,
        ['e', 'k'] => 0.3,
        ['e', 'l'] => 1.7,
        ['e', 'm'] => 1.6,
        ['e', 'n'] => 2.1,
        ['e', 'o'] => 1.7,
        ['e', 'p'] => 1.5,
        ['e', 'q'] => 1.1,
        ['e', 'r'] => 2.2,
        ['e', 's'] => 2.2,
        ['e', 't'] => 1.9,
        ['e', 'u'] => 0.8,
        ['e', 'v'] => 1.2,
        ['e', 'w'] => 1.6,
        ['e', 'x'] => 1.2,
        ['e', 'y'] => 1.2,
        ['e', 'z'] => 0.0,

        ['f', 'a'] => 1.3,
        ['f', 'b'] => 0.3,
        ['f', 'c'] => 1.0,
        ['f', 'd'] => 0.0,
        ['f', 'e'] => 1.4,
        ['f', 'f'] => 1.1,
        ['f', 'g'] => 0.0,
        ['f', 'h'] => 0.8,
        ['f', 'i'] => 1.3,
        ['f', 'j'] => 0.0,
        ['f', 'k'] => 0.0,
        ['f', 'l'] => 1.0,
        ['f', 'm'] => 0.5,
        ['f', 'n'] => 0.3,
        ['f', 'o'] => 1.6,
        ['f', 'p'] => 0.5,
        ['f', 'q'] => 0.0,
        ['f', 'r'] => 0.6,
        ['f', 's'] => 0.9,
        ['f', 't'] => 1.6,
        ['f', 'u'] => 1.0,
        ['f', 'v'] => 0.0,
        ['f', 'w'] => 0.6,
        ['f', 'x'] => 0.0,
        ['f', 'y'] => 0.0,
        ['f', 'z'] => 0.0,

        ['g', 'a'] => 1.0,
        ['g', 'b'] => 0.3,
        ['g', 'c'] => 0.0,
        ['g', 'd'] => 0.0,
        ['g', 'e'] => 1.5,
        ['g', 'f'] => 0.5,
        ['g', 'g'] => 0.0,
        ['g', 'h'] => 1.2,
        ['g', 'i'] => 1.0,
        ['g', 'j'] => 0.0,
        ['g', 'k'] => 0.0,
        ['g', 'l'] => 0.6,
        ['g', 'm'] => 0.0,
        ['g', 'n'] => 0.5,
        ['g', 'o'] => 1.4,
        ['g', 'p'] => 0.0,
        ['g', 'q'] => 0.0,
        ['g', 'r'] => 1.3,
        ['g', 's'] => 0.8,
        ['g', 't'] => 1.1,
        ['g', 'u'] => 0.9,
        ['g', 'v'] => 0.0,
        ['g', 'w'] => 0.3,
        ['g', 'x'] => 0.0,
        ['g', 'y'] => 0.0,
        ['g', 'z'] => 0.0,

        ['h', 'a'] => 1.9,
        ['h', 'b'] => 0.0,
        ['h', 'c'] => 0.3,
        ['h', 'd'] => 0.0,
        ['h', 'e'] => 2.4,
        ['h', 'f'] => 0.3,
        ['h', 'g'] => 0.0,
        ['h', 'h'] => 0.7,
        ['h', 'i'] => 1.9,
        ['h', 'j'] => 0.0,
        ['h', 'k'] => 0.0,
        ['h', 'l'] => 0.5,
        ['h', 'm'] => 0.0,
        ['h', 'n'] => 0.3,
        ['h', 'o'] => 1.7,
        ['h', 'p'] => 0.0,
        ['h', 'q'] => 0.0,
        ['h', 'r'] => 0.9,
        ['h', 's'] => 0.5,
        ['h', 't'] => 1.3,
        ['h', 'u'] => 0.3,
        ['h', 'v'] => 0.0,
        ['h', 'w'] => 0.8,
        ['h', 'x'] => 0.0,
        ['h', 'y'] => 0.0,
        ['h', 'z'] => 0.0,

        ['i', 'a'] => 1.3,
        ['i', 'b'] => 0.8,
        ['i', 'c'] => 1.7,
        ['i', 'd'] => 1.2,
        ['i', 'e'] => 1.6,
        ['i', 'f'] => 1.4,
        ['i', 'g'] => 1.0,
        ['i', 'h'] => 0.0,
        ['i', 'i'] => 0.0,
        ['i', 'j'] => 0.0,
        ['i', 'k'] => 0.9,
        ['i', 'l'] => 1.6,
        ['i', 'm'] => 1.5,
        ['i', 'n'] => 2.2,
        ['i', 'o'] => 1.8,
        ['i', 'p'] => 0.5,
        ['i', 'q'] => 0.0,
        ['i', 'r'] => 1.3,
        ['i', 's'] => 2.0,
        ['i', 't'] => 1.9,
        ['i', 'u'] => 0.0,
        ['i', 'v'] => 1.1,
        ['i', 'w'] => 0.0,
        ['i', 'x'] => 0.0,
        ['i', 'y'] => 0.0,
        ['i', 'z'] => 0.6,

        ['j', 'a'] => 0.0,
        ['j', 'b'] => 0.0,
        ['j', 'c'] => 0.0,
        ['j', 'd'] => 0.0,
        ['j', 'e'] => 0.3,
        ['j', 'f'] => 0.0,
        ['j', 'g'] => 0.0,
        ['j', 'h'] => 0.0,
        ['j', 'i'] => 0.0,
        ['j', 'j'] => 0.0,
        ['j', 'k'] => 0.0,
        ['j', 'l'] => 0.0,
        ['j', 'm'] => 0.0,
        ['j', 'n'] => 0.0,
        ['j', 'o'] => 0.6,
        ['j', 'p'] => 0.0,
        ['j', 'q'] => 0.0,
        ['j', 'r'] => 0.0,
        ['j', 's'] => 0.0,
        ['j', 't'] => 0.0,
        ['j', 'u'] => 0.6,
        ['j', 'v'] => 0.0,
        ['j', 'w'] => 0.0,
        ['j', 'x'] => 0.0,
        ['j', 'y'] => 0.0,
        ['j', 'z'] => 0.0,

        ['k', 'a'] => 0.0,
        ['k', 'b'] => 0.0,
        ['k', 'c'] => 0.0,
        ['k', 'd'] => 0.0,
        ['k', 'e'] => 1.4,
        ['k', 'f'] => 0.0,
        ['k', 'g'] => 0.0,
        ['k', 'h'] => 0.0,
        ['k', 'i'] => 0.9,
        ['k', 'j'] => 0.0,
        ['k', 'k'] => 0.0,
        ['k', 'l'] => 0.0,
        ['k', 'm'] => 0.0,
        ['k', 'n'] => 0.5,
        ['k', 'o'] => 0.5,
        ['k', 'p'] => 0.0,
        ['k', 'q'] => 0.0,
        ['k', 'r'] => 0.0,
        ['k', 's'] => 0.3,
        ['k', 't'] => 0.0,
        ['k', 'u'] => 0.0,
        ['k', 'v'] => 0.0,
        ['k', 'w'] => 0.5,
        ['k', 'x'] => 0.0,
        ['k', 'y'] => 0.5,
        ['k', 'z'] => 0.0,

        ['l', 'a'] => 1.5,
        ['l', 'b'] => 0.8,
        ['l', 'c'] => 0.9,
        ['l', 'd'] => 1.4,
        ['l', 'e'] => 1.9,
        ['l', 'f'] => 0.7,
        ['l', 'g'] => 0.0,
        ['l', 'h'] => 0.0,
        ['l', 'i'] => 1.8,
        ['l', 'j'] => 0.0,
        ['l', 'k'] => 0.5,
        ['l', 'l'] => 1.7,
        ['l', 'm'] => 0.6,
        ['l', 'n'] => 0.0,
        ['l', 'o'] => 1.4,
        ['l', 'p'] => 0.3,
        ['l', 'q'] => 0.3,
        ['l', 'r'] => 0.3,
        ['l', 's'] => 1.1,
        ['l', 't'] => 1.3,
        ['l', 'u'] => 0.9,
        ['l', 'v'] => 0.3,
        ['l', 'w'] => 0.7,
        ['l', 'x'] => 0.0,
        ['l', 'y'] => 1.7,
        ['l', 'z'] => 0.0,

        ['m', 'a'] => 1.7,
        ['m', 'b'] => 1.0,
        ['m', 'c'] => 0.0,
        ['m', 'd'] => 0.3,
        ['m', 'e'] => 1.7,
        ['m', 'f'] => 0.0,
        ['m', 'g'] => 0.0,
        ['m', 'h'] => 0.0,
        ['m', 'i'] => 1.4,
        ['m', 'j'] => 0.0,
        ['m', 'k'] => 0.0,
        ['m', 'l'] => 0.0,
        ['m', 'm'] => 0.7,
        ['m', 'n'] => 0.5,
        ['m', 'o'] => 1.4,
        ['m', 'p'] => 1.2,
        ['m', 'q'] => 0.0,
        ['m', 'r'] => 0.0,
        ['m', 's'] => 0.8,
        ['m', 't'] => 0.8,
        ['m', 'u'] => 1.1,
        ['m', 'v'] => 0.0,
        ['m', 'w'] => 0.3,
        ['m', 'x'] => 0.0,
        ['m', 'y'] => 0.5,
        ['m', 'z'] => 0.0,

        ['n', 'a'] => 1.7,
        ['n', 'b'] => 0.8,
        ['n', 'c'] => 1.5,
        ['n', 'd'] => 2.1,
        ['n', 'e'] => 1.8,
        ['n', 'f'] => 0.9,
        ['n', 'g'] => 1.9,
        ['n', 'h'] => 1.0,
        ['n', 'i'] => 1.6,
        ['n', 'j'] => 0.5,
        ['n', 'k'] => 0.5,
        ['n', 'l'] => 1.0,
        ['n', 'm'] => 0.8,
        ['n', 'n'] => 1.0,
        ['n', 'o'] => 1.8,
        ['n', 'p'] => 0.8,
        ['n', 'q'] => 0.0,
        ['n', 'r'] => 0.7,
        ['n', 's'] => 1.7,
        ['n', 't'] => 2.0,
        ['n', 'u'] => 1.1,
        ['n', 'v'] => 0.6,
        ['n', 'w'] => 1.2,
        ['n', 'x'] => 0.0,
        ['n', 'y'] => 1.1,
        ['n', 'z'] => 0.0,

        ['o', 'a'] => 1.0,
        ['o', 'b'] => 1.3,
        ['o', 'c'] => 1.3,
        ['o', 'd'] => 1.2,
        ['o', 'e'] => 0.5,
        ['o', 'f'] => 2.0,
        ['o', 'g'] => 0.5,
        ['o', 'h'] => 0.5,
        ['o', 'i'] => 1.1,
        ['o', 'j'] => 0.0,
        ['o', 'k'] => 0.7,
        ['o', 'l'] => 1.2,
        ['o', 'm'] => 1.6,
        ['o', 'n'] => 2.2,
        ['o', 'o'] => 1.4,
        ['o', 'p'] => 1.5,
        ['o', 'q'] => 0.0,
        ['o', 'r'] => 2.1,
        ['o', 's'] => 1.6,
        ['o', 't'] => 1.7,
        ['o', 'u'] => 2.0,
        ['o', 'v'] => 1.1,
        ['o', 'w'] => 1.6,
        ['o', 'x'] => 0.0,
        ['o', 'y'] => 0.6,
        ['o', 'z'] => 0.3,

        ['p', 'a'] => 1.3,
        ['p', 'b'] => 0.0,
        ['p', 'c'] => 0.0,
        ['p', 'd'] => 0.0,
        ['p', 'e'] => 1.6,
        ['p', 'f'] => 0.0,
        ['p', 'g'] => 0.0,
        ['p', 'h'] => 0.8,
        ['p', 'i'] => 0.9,
        ['p', 'j'] => 0.0,
        ['p', 'k'] => 0.0,
        ['p', 'l'] => 1.5,
        ['p', 'm'] => 0.0,
        ['p', 'n'] => 0.0,
        ['p', 'o'] => 1.4,
        ['p', 'p'] => 1.4,
        ['p', 'q'] => 0.0,
        ['p', 'r'] => 1.6,
        ['p', 's'] => 0.5,
        ['p', 't'] => 1.1,
        ['p', 'u'] => 0.8,
        ['p', 'v'] => 0.0,
        ['p', 'w'] => 0.0,
        ['p', 'x'] => 0.0,
        ['p', 'y'] => 0.3,
        ['p', 'z'] => 0.0,

        ['q', 'a'] => 0.0,
        ['q', 'b'] => 0.0,
        ['q', 'c'] => 0.0,
        ['q', 'd'] => 0.0,
        ['q', 'e'] => 0.0,
        ['q', 'f'] => 0.0,
        ['q', 'g'] => 0.0,
        ['q', 'h'] => 0.0,
        ['q', 'i'] => 0.0,
        ['q', 'j'] => 0.0,
        ['q', 'k'] => 0.0,
        ['q', 'l'] => 0.0,
        ['q', 'm'] => 0.0,
        ['q', 'n'] => 0.0,
        ['q', 'o'] => 0.0,
        ['q', 'p'] => 0.0,
        ['q', 'q'] => 0.0,
        ['q', 'r'] => 0.0,
        ['q', 's'] => 0.0,
        ['q', 't'] => 0.0,
        ['q', 'u'] => 1.3,
        ['q', 'v'] => 0.0,
        ['q', 'w'] => 0.0,
        ['q', 'x'] => 0.0,
        ['q', 'y'] => 0.0,
        ['q', 'z'] => 0.0,

        ['r', 'a'] => 1.8,
        ['r', 'b'] => 0.6,
        ['r', 'c'] => 1.1,
        ['r', 'd'] => 1.2,
        ['r', 'e'] => 2.2,
        ['r', 'f'] => 0.8,
        ['r', 'g'] => 0.8,
        ['r', 'h'] => 0.5,
        ['r', 'i'] => 1.9,
        ['r', 'j'] => 0.0,
        ['r', 'k'] => 1.0,
        ['r', 'l'] => 1.1,
        ['r', 'm'] => 1.2,
        ['r', 'n'] => 1.1,
        ['r', 'o'] => 1.7,
        ['r', 'p'] => 0.9,
        ['r', 'q'] => 0.0,
        ['r', 'r'] => 1.3,
        ['r', 's'] => 1.6,
        ['r', 't'] => 1.8,
        ['r', 'u'] => 0.8,
        ['r', 'v'] => 0.7,
        ['r', 'w'] => 1.0,
        ['r', 'x'] => 0.0,
        ['r', 'y'] => 1.2,
        ['r', 'z'] => 0.0,

        ['s', 'a'] => 1.9,
        ['s', 'b'] => 1.1,
        ['s', 'c'] => 1.3,
        ['s', 'd'] => 0.8,
        ['s', 'e'] => 1.9,
        ['s', 'f'] => 1.1,
        ['s', 'g'] => 0.8,
        ['s', 'h'] => 1.5,
        ['s', 'i'] => 1.6,
        ['s', 'j'] => 0.0,
        ['s', 'k'] => 0.3,
        ['s', 'l'] => 0.8,
        ['s', 'm'] => 1.1,
        ['s', 'n'] => 1.3,
        ['s', 'o'] => 1.9,
        ['s', 'p'] => 1.4,
        ['s', 'q'] => 0.3,
        ['s', 'r'] => 0.8,
        ['s', 's'] => 1.6,
        ['s', 't'] => 2.1,
        ['s', 'u'] => 1.5,
        ['s', 'v'] => 0.3,
        ['s', 'w'] => 1.4,
        ['s', 'x'] => 0.0,
        ['s', 'y'] => 0.6,
        ['s', 'z'] => 0.0,

        ['t', 'a'] => 1.7,
        ['t', 'b'] => 1.1,
        ['t', 'c'] => 0.8,
        ['t', 'd'] => 1.0,
        ['t', 'e'] => 2.0,
        ['t', 'f'] => 0.7,
        ['t', 'g'] => 0.0,
        ['t', 'h'] => 2.5,
        ['t', 'i'] => 2.1,
        ['t', 'j'] => 0.0,
        ['t', 'k'] => 0.0,
        ['t', 'l'] => 1.1,
        ['t', 'm'] => 1.1,
        ['t', 'n'] => 0.9,
        ['t', 'o'] => 2.0,
        ['t', 'p'] => 0.9,
        ['t', 'q'] => 0.0,
        ['t', 'r'] => 1.5,
        ['t', 's'] => 1.5,
        ['t', 't'] => 1.7,
        ['t', 'u'] => 1.3,
        ['t', 'v'] => 0.6,
        ['t', 'w'] => 1.2,
        ['t', 'x'] => 0.0,
        ['t', 'y'] => 1.3,
        ['t', 'z'] => 0.0,

        ['u', 'a'] => 1.3,
        ['u', 'b'] => 0.7,
        ['u', 'c'] => 1.2,
        ['u', 'd'] => 1.0,
        ['u', 'e'] => 1.0,
        ['u', 'f'] => 0.0,
        ['u', 'g'] => 1.1,
        ['u', 'h'] => 0.3,
        ['u', 'i'] => 0.7,
        ['u', 'j'] => 0.0,
        ['u', 'k'] => 0.0,
        ['u', 'l'] => 1.4,
        ['u', 'm'] => 1.0,
        ['u', 'n'] => 1.5,
        ['u', 'o'] => 0.3,
        ['u', 'p'] => 1.2,
        ['u', 'q'] => 0.0,
        ['u', 'r'] => 1.7,
        ['u', 's'] => 1.6,
        ['u', 't'] => 1.7,
        ['u', 'u'] => 0.0,
        ['u', 'v'] => 0.0,
        ['u', 'w'] => 0.0,
        ['u', 'x'] => 0.0,
        ['u', 'y'] => 0.0,
        ['u', 'z'] => 0.0,

        ['v', 'a'] => 1.2,
        ['v', 'b'] => 0.0,
        ['v', 'c'] => 0.0,
        ['v', 'd'] => 0.0,
        ['v', 'e'] => 1.7,
        ['v', 'f'] => 0.0,
        ['v', 'g'] => 0.0,
        ['v', 'h'] => 0.0,
        ['v', 'i'] => 1.3,
        ['v', 'j'] => 0.0,
        ['v', 'k'] => 0.0,
        ['v', 'l'] => 0.0,
        ['v', 'm'] => 0.0,
        ['v', 'n'] => 0.0,
        ['v', 'o'] => 0.8,
        ['v', 'p'] => 0.0,
        ['v', 'q'] => 0.0,
        ['v', 'r'] => 0.0,
        ['v', 's'] => 0.0,
        ['v', 't'] => 0.0,
        ['v', 'u'] => 0.0,
        ['v', 'v'] => 0.0,
        ['v', 'w'] => 0.0,
        ['v', 'x'] => 0.0,
        ['v', 'y'] => 0.0,
        ['v', 'z'] => 0.0,

        ['w', 'a'] => 1.5,
        ['w', 'b'] => 0.0,
        ['w', 'c'] => 0.5,
        ['w', 'd'] => 0.6,
        ['w', 'e'] => 1.5,
        ['w', 'f'] => 0.0,
        ['w', 'g'] => 0.0,
        ['w', 'h'] => 1.7,
        ['w', 'i'] => 1.6,
        ['w', 'j'] => 0.0,
        ['w', 'k'] => 0.0,
        ['w', 'l'] => 0.6,
        ['w', 'm'] => 0.0,
        ['w', 'n'] => 1.0,
        ['w', 'o'] => 1.2,
        ['w', 'p'] => 0.3,
        ['w', 'q'] => 0.0,
        ['w', 'r'] => 0.0,
        ['w', 's'] => 0.5,
        ['w', 't'] => 0.8,
        ['w', 'u'] => 0.0,
        ['w', 'v'] => 0.0,
        ['w', 'w'] => 0.3,
        ['w', 'x'] => 0.0,
        ['w', 'y'] => 0.0,
        ['w', 'z'] => 0.0,

        ['x', 'a'] => 0.5,
        ['x', 'b'] => 0.0,
        ['x', 'c'] => 0.7,
        ['x', 'd'] => 0.0,
        ['x', 'e'] => 0.0,
        ['x', 'f'] => 0.0,
        ['x', 'g'] => 0.0,
        ['x', 'h'] => 0.0,
        ['x', 'i'] => 0.6,
        ['x', 'j'] => 0.0,
        ['x', 'k'] => 0.0,
        ['x', 'l'] => 0.0,
        ['x', 'm'] => 0.0,
        ['x', 'n'] => 0.0,
        ['x', 'o'] => 0.0,
        ['x', 'p'] => 0.6,
        ['x', 'q'] => 0.0,
        ['x', 'r'] => 0.0,
        ['x', 's'] => 0.0,
        ['x', 't'] => 0.0,
        ['x', 'u'] => 0.0,
        ['x', 'v'] => 0.0,
        ['x', 'w'] => 0.0,
        ['x', 'x'] => 0.0,
        ['x', 'y'] => 0.0,
        ['x', 'z'] => 0.0,

        ['y', 'a'] => 1.0,
        ['y', 'b'] => 1.0,
        ['y', 'c'] => 1.0,
        ['y', 'd'] => 0.6,
        ['y', 'e'] => 1.1,
        ['y', 'f'] => 0.5,
        ['y', 'g'] => 0.7,
        ['y', 'h'] => 0.7,
        ['y', 'i'] => 1.3,
        ['y', 'j'] => 0.0,
        ['y', 'k'] => 0.0,
        ['y', 'l'] => 0.8,
        ['y', 'm'] => 0.6,
        ['y', 'n'] => 0.5,
        ['y', 'o'] => 1.4,
        ['y', 'p'] => 0.8,
        ['y', 'q'] => 0.0,
        ['y', 'r'] => 0.7,
        ['y', 's'] => 1.2,
        ['y', 't'] => 1.3,
        ['y', 'u'] => 0.0,
        ['y', 'v'] => 0.5,
        ['y', 'w'] => 1.1,
        ['y', 'x'] => 0.0,
        ['y', 'y'] => 0.0,
        ['y', 'z'] => 0.0,

        ['z', 'a'] => 0.0,
        ['z', 'b'] => 0.0,
        ['z', 'c'] => 0.0,
        ['z', 'd'] => 0.0,
        ['z', 'e'] => 0.7,
        ['z', 'f'] => 0.0,
        ['z', 'g'] => 0.0,
        ['z', 'h'] => 0.0,
        ['z', 'i'] => 0.3,
        ['z', 'j'] => 0.0,
        ['z', 'k'] => 0.0,
        ['z', 'l'] => 0.0,
        ['z', 'm'] => 0.0,
        ['z', 'n'] => 0.0,
        ['z', 'o'] => 0.0,
        ['z', 'p'] => 0.0,
        ['z', 'q'] => 0.0,
        ['z', 'r'] => 0.0,
        ['z', 's'] => 0.0,
        ['z', 't'] => 0.0,
        ['z', 'u'] => 0.0,
        ['z', 'v'] => 0.0,
        ['z', 'w'] => 0.0,
        ['z', 'x'] => 0.0,
        ['z', 'y'] => 0.0,
        ['z', 'z'] => 0.0,
        _ => -1.0,
    }
}

const fn log_weight(c: char) -> f32 {
    match c {
        'a' => 1.9,
        'b' => 1.2,
        'c' => 1.4,
        'd' => 1.6,
        'e' => 2.1,
        'f' => 1.3,
        'g' => 1.3,
        'h' => 1.8,
        'i' => 1.8,
        'j' => 0.3,
        'k' => 0.9,
        'l' => 1.6,
        'm' => 1.4,
        'n' => 1.8,
        'o' => 1.9,
        'p' => 1.3,
        'q' => 0.0,
        'r' => 1.8,
        's' => 1.8,
        't' => 1.9,
        'u' => 1.4,
        'v' => 1.0,
        'w' => 1.4,
        'x' => 0.0,
        'y' => 1.3,
        'z' => 0.0,
        _ => -1.0,
    }
}
