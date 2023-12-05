use alloc::string::String;

pub trait Solution {
    fn solve_p1(_input: &str) -> AocSol;
    fn solve_p2(_input: &str) -> AocSol;
}

#[derive(Debug)]
pub enum AocSol {
    String(String),
    Unsigned(usize),
    Signed(isize),
}

impl core::fmt::Display for AocSol {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            AocSol::String(a) => f.write_str(a),
            AocSol::Unsigned(a) => f.write_fmt(format_args!("{}", a)),
            AocSol::Signed(a) => f.write_fmt(format_args!("{}", a)),
        }
    }
}

impl From<usize> for AocSol {
    fn from(value: usize) -> Self {
        Self::Unsigned(value)
    }
}

impl From<isize> for AocSol {
    fn from(value: isize) -> Self {
        Self::Signed(value)
    }
}

impl From<String> for AocSol {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}
