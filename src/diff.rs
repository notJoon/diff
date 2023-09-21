// diff algorithm
// ref: diffy https://github.com/bmwill/diffy
// doc: https://stackoverflow.com/questions/42635889/myers-diff-algorithm-vs-hunt-mcilroy-algorithm
// doc: https://blog.jcoglan.com/2017/02/12/the-myers-diff-algorithm-part-1/

use std::{fmt, ops};

use crate::range::Range;

/// `FurthestPathEndpoints` contains the endpoints of the furthest reaching `D-paths`. For each
/// recorded endpoint `(x,y)` in diagonal `k`, we only need to retain `x` because `y` can be
/// computed from `x - k`.
///
/// In other words, `FurthestPathEndpoints` is an array of integers where the value at index `k`
/// contains the row index of the endpoint of the furthest reaching path in diagonal `k`.
#[derive(Debug, Clone)]
struct FurthestPathEndpoints {
    offset: isize,
    endpoints: Vec<usize>, // Look into initializing this to -1 and storing isize
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
struct Snake {
    x_start: usize,
    y_start: usize,
    x_end: usize,
    y_end: usize,
}

impl fmt::Display for Snake {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({}, {}) -> ({}, {})",
            self.x_start, self.y_start, self.x_end, self.y_end
        )
    }
}

fn max_d(a: usize, b: usize) -> usize {
    ((a + b + 1) / 2) + 1
}

fn find_snake_pivot<T: PartialEq>(
    old: Range<'_, [T]>,
    new: Range<'_, [T]>,
    vf: &mut FurthestPathEndpoints,
    vb: &mut FurthestPathEndpoints,
) {
    let a = old.len();
    let b = new.len();

    // the optimal edit script length is odd or even as delta is odd or even
    let delta = a as isize - b as isize;
    let odd = delta & 1 == 1;

    // initial point at (0, -1)
    vf[1] = 0;

    // initial point at (N, M+1)
    vb[1] = 0;

    let d_max = max_d(a, b);
    if vf.len() < d_max || vb.len() < d_max {
        panic!("Error: d_max is too large");
    }

    for d in 0..d_max as isize {
        for k in (-d..d).rev().step_by(2) {
            let mut x = if k == -d || (k != d && vf[k - 1] < vf[k + 1]) {
                vf[k + 1]
            } else {
                vf[k - 1] + 1
            };

            let mut y = (x as isize - k) as usize;

            // start coordinate of the snake
            let (x_0, y_0) = (x, y);
        }
    }
}
