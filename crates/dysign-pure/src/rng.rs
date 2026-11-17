//! bionic `srand`/`random` 的 TYPE_3 加法发生器（deg 31 / sep 3）。
//!
//! 只复现被 libmetasec 实际用到的那条路径：`srand(seed)` 建表并热身
//! `10 * 31` 次，之后每次 `random()` 取 `(state[f] >> 1) & 0x7fffffff`。

const DEG: usize = 31;
const SEP: usize = 3;

pub struct Type3 {
    state: [u32; DEG],
    f: usize,
    r: usize,
}

impl Type3 {
    pub fn new(seed: u32) -> Self {
        let mut state = [0u32; DEG];
        state[0] = seed;
        for i in 1..DEG {
            let prev = state[i - 1] as i64;
            let mut v = 16807 * (prev % 127773) - 2836 * (prev / 127773);
            if v <= 0 {
                v += 2147483647;
            }
            state[i] = v as u32;
        }
        let mut rng = Type3 {
            state,
            f: SEP,
            r: 0,
        };
        for _ in 0..10 * DEG {
            rng.next_u31();
        }
        rng
    }

    pub fn next_u31(&mut self) -> u32 {
        self.state[self.f] = self.state[self.f].wrapping_add(self.state[self.r]);
        let out = (self.state[self.f] >> 1) & 0x7fff_ffff;
        self.f += 1;
        if self.f >= DEG {
            self.f = 0;
            self.r += 1;
            if self.r >= DEG {
                self.r = 0;
            }
        } else {
            self.r += 1;
            if self.r >= DEG {
                self.r = 0;
            }
        }
        out
    }

    /// 第 `n` 个（1 起）输出。
    pub fn nth0(&mut self, n: usize) -> u32 {
        let mut v = 0;
        for _ in 0..n {
            v = self.next_u31();
        }
        v
    }
}
