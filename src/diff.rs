// diff algorithm
// ref: diffy https://github.com/bmwill/diffy
// doc: https://stackoverflow.com/questions/42635889/myers-diff-algorithm-vs-hunt-mcilroy-algorithm
// doc: https://blog.jcoglan.com/2017/02/12/the-myers-diff-algorithm-part-1/

use std::{fmt, ops};

use crate::range::{Range, DiffRange};

/// `FurthestPathEndpoints` contains the endpoints of the furthest reaching `D-paths`. For each
/// recorded endpoint `(x,y)` in diagonal `k`, we only need to retain `x` because `y` can be
/// computed from `x - k`.
///
/// In other words, `FurthestPathEndpoints` is an array of integers where the value at index `k`
/// contains the row index of the endpoint of the furthest reaching path in diagonal `k`.
#[derive(Debug, Clone)]
pub struct FurthestPathEndpoints {
    offset: isize,
    endpoints: Vec<usize>,
}

impl FurthestPathEndpoints {
    fn new(max_d: usize) -> Self {
        Self {
            offset: max_d as isize,
            endpoints: vec![0; 2 * max_d + 1],
        }
    }

    fn len(&self) -> usize {
        self.endpoints.len()
    }
}

impl ops::Index<isize> for FurthestPathEndpoints {
    type Output = usize;

    fn index(&self, index: isize) -> &Self::Output {
        &self.endpoints[(index + self.offset) as usize]
    }
}

impl ops::IndexMut<isize> for FurthestPathEndpoints {
    fn index_mut(&mut self, index: isize) -> &mut Self::Output {
        &mut self.endpoints[(index + self.offset) as usize]
    }
}

/// `Snake` is a sequence of diagonal edges in the edit graph that connect the furthest reaching.
///
/// If a length is 0, it refers to the start and end points are the same.
#[derive(Debug, PartialEq)]
struct Snake {
    x_start: usize,
    y_start: usize,
    x_end: usize,
    y_end: usize,
}

fn max_d(a: usize, b: usize) -> usize {
    ((a + b + 1) / 2) + 1
}

fn find_snake_pivot<T: PartialEq>(
    old: Range<'_, [T]>,
    new: Range<'_, [T]>,
    vf: &mut FurthestPathEndpoints,
    vb: &mut FurthestPathEndpoints,
) -> (isize, Snake) {
    let n = old.len();
    let m = new.len();

    // the optimal edit script length is odd or even as delta is odd or even
    let delta = n as isize - m as isize;
    let odd = delta & 1 == 1;

    // initial point at (0, -1)
    vf[1] = 0;

    // initial point at (N, M+1)
    vb[1] = 0;

    let d_max = max_d(n, m);
    if vf.len() < d_max || vb.len() < d_max {
        panic!("Error: d_max is too large");
    }

    for d in 0..d_max as isize {
        // forward path
        for k in (-d..=d).rev().step_by(2) {
            let mut x = if k == -d || (k != d && vf[k - 1] < vf[k + 1]) {
                vf[k + 1]
            } else {
                vf[k - 1] + 1
            };

            let mut y = (x as isize - k) as usize;

            // start coordinate of the snake
            let (x_0, y_0) = (x, y);
            if let (Some(s1), Some(s2)) = (old.get(x..), new.get(y..)) {
                let advance = s1.common_prefix_len(s2);
                x += advance;
                y += advance;
            }

            vf[k] = x;

            if odd && (k - delta).abs() <= (d - 1) && vf[k] + vb[-(k - delta)] >= n {
                let snake = Snake {
                    x_start: x_0,
                    y_start: y_0,
                    x_end: x,
                    y_end: y,
                };

                return (2 * d - 1, snake);
            }
        }

        // backward
        for k in (-d..=d).rev().step_by(2) {
            let mut x = if k == -d || (k != d && vb[k - 1] < vb[k + 1]) {
                vb[k + 1]
            } else {
                vb[k - 1] + 1
            };

            let mut y = (x as isize - k) as usize;

            // the coordinate of the start of a snake
            let (x_0, y_0) = (x, y);
            if x < n && y < m {
                let advance = old.slice(..n - x).common_suffix_len(new.slice(..m - y));
                x += advance;
                y += advance;
            }

            // new best x value
            vb[k] = x;

            if !odd && (k - delta).abs() <= d && vb[k] + vf[-(k - delta)] >= n{
                let snake = Snake {
                    x_start: n - x,
                    y_start: m - y,
                    x_end: n - x_0,
                    y_end: m - y_0,
                };

                return (2 * d, snake);
            }
        }
    }

    unreachable!("Error: snake not found");
}

fn conquer<'a, 'b, T: PartialEq>(
    mut old: Range<'a, [T]>,
    mut new: Range<'b, [T]>,
    vf: &mut FurthestPathEndpoints,
    vb: &mut FurthestPathEndpoints,
    result: &mut Vec<DiffRange<'a, 'b, [T]>>,
) {
    // check common prefix
    let prefix_len = old.common_prefix_len(new);
    if prefix_len > 0 {
        let common_prefix = DiffRange::Equal(
            old.slice(..prefix_len),
            new.slice(..prefix_len),
        );

        result.push(common_prefix);
    }

    old = old.slice(prefix_len..);
    new = new.slice(prefix_len..);

    let common_suffix_len = old.common_suffix_len(new);
    let common_suffix = DiffRange::Equal(
        old.slice(old.len() - common_suffix_len..),
        new.slice(new.len() - common_suffix_len..),
    );

    old = old.slice(..old.len() - common_suffix_len);
    new = new.slice(..new.len() - common_suffix_len);

    // check common suffix
    match (old.is_empty(), new.is_empty()) {
        (true, true) => {},
        (true, false) => {
            DiffRange::Insert(new);
        },
        (false, true) => {
            DiffRange::Delete(old);
        },
        (_, _) => {
            let (_len, snake) = find_snake_pivot(old, new, vf, vb);

            let (old_a, old_b) = old.split_at(snake.x_start);
            let (new_a, new_b) = new.split_at(snake.y_start);

            conquer(old_a, new_a, vf, vb, result);
            conquer(old_b, new_b, vf, vb, result);
        }
    }

    if common_suffix_len > 0 {
        result.push(common_suffix);
    }
}

pub fn diff<'a, 'b, T: PartialEq>(old: &'a [T], new: &'b [T]) -> Vec<DiffRange<'a, 'b, [T]>> {
    let old_recs = Range::new(old, ..);
    let new_recs = Range::new(new, ..);

    let mut result = Vec::new();

    let max_d = max_d(old.len(), new.len());
    let mut vf = FurthestPathEndpoints::new(max_d);
    let mut vb = FurthestPathEndpoints::new(max_d);

    conquer(old_recs, new_recs, &mut vf, &mut vb, &mut result);

    result
}


#[cfg(test)]
mod diff_test {
    use super::*;

    #[test]
    fn test_diff() {
        let a = Range::new(&b"ABCABBA"[..], ..);
        let b = Range::new(&b"CBABAC"[..], ..);
        let max_d = max_d(a.len(), b.len());
        let mut vf = FurthestPathEndpoints::new(max_d);
        let mut vb = FurthestPathEndpoints::new(max_d);
        
        let result = find_snake_pivot(a, b, &mut vf, &mut vb);

        let expected = (5, Snake {
            x_start: 4,
            y_start: 1,
            x_end: 5,
            y_end: 2,
        });

        assert_eq!(result, expected);
    }
}