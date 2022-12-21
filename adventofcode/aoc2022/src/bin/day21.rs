use glam::*;
use std::cmp::*;
use std::collections::*;
use std::io::{stdin, Read};
use std::ops::*;

fn main() -> anyhow::Result<()> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    let monkeys: HashMap<&str, Monkey> = input
        .trim()
        .lines()
        .map(|line| {
            let (name, value) = line.split_once(": ").unwrap();
            let monkey = if let Ok(x) = value.parse() {
                Monkey::Literal(x)
            } else {
                let mut parts = value.split_whitespace();
                let left = parts.next().unwrap();
                let op = BinOp::parse(parts.next().unwrap()).unwrap();
                let right = parts.next().unwrap();

                Monkey::BinOp(op, left, right)
            };
            (name, monkey)
        })
        .collect();

    // Part 1
    let mut ctx = Context {
        monkeys: &monkeys,
        cache: HashMap::new(),
    };
    let ans = ctx.evaluate("root");
    println!("{}", ans);

    // Part 2
    fn fold(monkeys: &HashMap<&str, Monkey>, name: &str) -> Expr {
        if name == "humn" {
            return Expr::You;
        }
        match &monkeys[name] {
            &Monkey::Literal(x) => Expr::Literal(x),
            &Monkey::BinOp(op, l, r) => match (fold(monkeys, l), fold(monkeys, r)) {
                (Expr::Literal(lx), Expr::Literal(rx)) => Expr::Literal(op.apply(lx, rx)),
                (le, re) => Expr::BinOp(Box::new((op, le, re))),
            },
        }
    }

    let equation = fold(&monkeys, "root");
    eprintln!("{:?}", equation);
    let (mut lhs, mut rhs) = match equation {
        Expr::BinOp(bx) => match *bx {
            (_, Expr::Literal(x), le) => (le, x),
            (_, le, Expr::Literal(x)) => (le, x),
            _ => unreachable!(),
        },
        _ => unreachable!(),
    };
    while let Expr::BinOp(bx) = lhs {
        eprintln!();
        eprintln!("{:?} = {}", bx, rhs);
        let (op, l, r) = *bx;
        if let Expr::Literal(x) = l {
            eprintln!("{} {:?} {}", rhs, op.left_inverse(), x);
            rhs = op.left_inverse().apply(rhs, x);
            lhs = r;
        } else if let Expr::Literal(x) = r {
            eprintln!("{} {:?} {}", rhs, op.right_inverse(), x);
            rhs = op.right_inverse().apply(rhs, x);
            lhs = l;
        } else {
            panic!();
        }
    }

    match lhs {
        Expr::You => {
            println!("{}", rhs);
        }
        _ => panic!(),
    }

    Ok(())
}

pub struct Context<'a> {
    monkeys: &'a HashMap<&'a str, Monkey<'a>>,
    cache: HashMap<&'a str, i64>,
}

impl<'a> Context<'a> {
    fn evaluate(&mut self, name: &'a str) -> i64 {
        if let Some(&x) = self.cache.get(&name) {
            x
        } else {
            let x = match &self.monkeys[name] {
                &Monkey::Literal(x) => x,
                &Monkey::BinOp(op, l, r) => op.apply(self.evaluate(l), self.evaluate(r)),
            };
            self.cache.insert(name, x);
            x
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum BinOp {
    Plus,
    Minus,
    MinusRev,
    Times,
    Div,
    DivRev,
}

impl BinOp {
    fn left_inverse(&self) -> Self {
        match self {
            Self::Plus => Self::Minus,
            Self::Minus => Self::MinusRev,
            Self::Times => Self::Div,
            Self::Div => Self::DivRev,
            _ => panic!(),
        }
    }

    fn right_inverse(&self) -> Self {
        match self {
            Self::Plus => Self::Minus,
            Self::Minus => Self::Plus,
            Self::Times => Self::Div,
            Self::Div => Self::Times,
            _ => panic!(),
        }
    }
    fn parse(s: &str) -> Option<Self> {
        match s {
            "+" => Some(Self::Plus),
            "-" => Some(Self::Minus),
            "*" => Some(Self::Times),
            "/" => Some(Self::Div),
            _ => None,
        }
    }

    fn apply(&self, left: i64, right: i64) -> i64 {
        match self {
            Self::Plus => left + right,
            Self::Minus => left - right,
            Self::MinusRev => right - left,
            Self::Times => left * right,
            Self::Div => {
                assert!(left % right == 0, "{} / {}", left, right);
                left / right
            }
            Self::DivRev => {
                assert!(right % left == 0, "{} / {}", right, left);
                right / left
            }
        }
    }
}

pub enum Monkey<'a> {
    Literal(i64),
    BinOp(BinOp, &'a str, &'a str),
}

#[derive(Debug)]
pub enum Expr {
    Literal(i64),
    BinOp(Box<(BinOp, Expr, Expr)>),
    You,
}
