use std::{
    fs::File,
    io::{BufReader, BufRead},
    ops::Add, fmt::Display,
};

const SNAFU_BASE: isize = 5;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Default, Clone, Copy)]
enum SnafuDigit {
    DoubleMinus = -2,
    Minus = -1,
    #[default]
    Zero = 0,
    One = 1,
    Two = 2,
}

impl Add<Self> for SnafuDigit {
    type Output = Snafu;
    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::DoubleMinus, Self::DoubleMinus) => Snafu::new(b"-1"),
            (Self::DoubleMinus, Self::Minus) => Snafu::new(b"-2"),
            (Self::DoubleMinus, Self::Zero) => Snafu::new(b"="),
            (Self::DoubleMinus, Self::One) => Snafu::new(b"-"),
            (Self::DoubleMinus, Self::Two) => Snafu::new(b"0"),
            //
            (Self::Minus, Self::Minus) => Snafu::new(b"="),
            (Self::Minus, Self::Zero) => Snafu::new(b"-"),
            (Self::Minus, Self::One) => Snafu::new(b"0"),
            (Self::Minus, Self::Two) => Snafu::new(b"1"),
            //
            (Self::Zero, Self::Zero) => Snafu::new(b"0"),
            (Self::Zero, Self::One) => Snafu::new(b"1"),
            (Self::Zero, Self::Two) => Snafu::new(b"2"),
            // 
            (Self::One, Self::One) => Snafu::new(b"2"),
            (Self::One, Self::Two) => Snafu::new(b"1="),
            // 
            (Self::Two, Self::Two) => Snafu::new(b"1-"),
            //
            (x, y) => Self::add(y,x),
        }
    }
}


impl From<&SnafuDigit> for isize {
    fn from(value: &SnafuDigit) -> Self {
        *value as isize
    }
}

impl From<SnafuDigit> for isize {
    fn from(value: SnafuDigit) -> Self {
        value.into()
    }
}

impl SnafuDigit {
    fn to_snafu(self) -> Snafu {
        Snafu(vec![self])
    }
}

impl From<&u8> for SnafuDigit {
    fn from(digit: &u8) -> Self {
        match digit {
            b'2' => Self::Two,
            b'1' => Self::One,
            b'0' => Self::Zero,
            b'-' => Self::Minus,
            b'=' => Self::DoubleMinus,
            _ => panic!("unknown digit"),
        }
    }
}

impl Display for SnafuDigit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::DoubleMinus => '=',
            Self::Minus => '-',
            Self::Zero => '0',
            Self::One => '1',
            Self::Two => '2',
        })
    }
}

// saved from LSD to MSD
#[derive(Debug, PartialEq, Eq, Clone)]
struct Snafu(Vec<SnafuDigit>);

impl Snafu {
    fn new(arr: &[u8]) -> Self {
        Self(arr.iter().rev().map(|digit| digit.into()).collect())
    }
}

impl Display for Snafu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for d in self.0.iter().rev() {
            write!(f, "{}", d)?;
        }
        Ok(())
    }
}

impl Default for Snafu {
    fn default() -> Self {
        Snafu(vec![SnafuDigit::default()])
    }
}

impl Add<Self> for Snafu {
    type Output = Snafu;
    fn add(self, rhs: Self) -> Self::Output {
        // longest number first
        let (x, y) = if self.0.len() > rhs.0.len() {(&self.0,&rhs.0)} else {(&rhs.0,&self.0)};
        let mut carry = Snafu(vec![]);
        let mut res = Snafu::new(b"");
        for i in 0.. {
            if i >= x.len() && carry.0.is_empty() {
                break;
            }

            carry = match (x.get(i), y.get(i), carry) {
                (Some(l), Some(r), carry) if carry.0.is_empty() => {
                    *l + *r
                }
                (Some(l), Some(r), carry) => {
                    *l + *r + carry
                }
                (Some(_), None, carry) if carry.0.is_empty() => {
                    res.0.extend_from_slice(&x[i..]);
                    return res
                }
                (Some(l), None, carry) => {
                    l.to_snafu() + carry
                }
                (None, Some(_), carry) if carry.0.is_empty() => {
                    res.0.extend_from_slice(&y[i..]);
                    return res
                }
                (None, Some(r), carry) => {
                    r.to_snafu() + carry
                }
                (None, None, carry) => {
                    res.0.extend(carry.0);
                    return res;
                }
            };
            let digit = carry.0.remove(0);
            res.0.push(digit);
        }
        res
    }
}

impl FromIterator<u8> for Snafu {
    fn from_iter<I: IntoIterator<Item = u8>>(iter: I) -> Self {
        let mut digits: Vec<SnafuDigit> = iter.into_iter().map(|b| (&b).into()).collect();
        digits.reverse();
        Self(digits)
    }
}

impl From<&Snafu> for isize {
    fn from(value: &Snafu) -> Self {
        let mut res = 0;
        let mut weight: isize = 1;
        for digit in value.0.iter() {
            let v = weight * (*digit) as isize;
            res += v;
            weight *= SNAFU_BASE;
        }
        res
    }
}

impl From<Snafu> for isize {
    fn from(value: Snafu) -> Self {
        (&value).into()
    }
}

#[allow(unused)]
fn test_conversion(dec: isize, snafu: Snafu) {
    let res: isize = snafu.into();
    assert_eq!(res, dec);
}

fn main() {
    let f = File::open("input/day25.txt").unwrap();
    let read = BufReader::new(f);
    let lines = read.lines();

    let mut acc = Snafu::default();
    for line in lines {
        let line = line.unwrap();
        let snafu: Snafu = line.bytes().collect();

        acc = acc + snafu;
    }
    println!("{}", acc);
}
