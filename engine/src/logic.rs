//! Story logic: conditions (`if = "trust >= 2 && item('Photograph')"`) and
//! text with counters in it (`"You have {evidence} leads."`).
//!
//! Conditions understand
//! - whole numbers, `true`, `false`
//! - counters by name (`trust`); a counter never set is 0
//! - `flag("x")`, `item("x")`, `visited("scene_id")` (single or double quotes)
//! - `+ -`, comparisons `== != < <= > >=`, `!`, `&&`, `||`, parentheses
//!
//! A bare number or counter is true when it isn't 0.

use anyhow::{Result, bail};

use crate::state::State;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Num(i64),
    Var(String),
    Flag(String),
    Item(String),
    Visited(String),
    Not(Box<Expr>),
    Bin(Box<Expr>, Op, Box<Expr>),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Op {
    Add,
    Sub,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    And,
    Or,
}

#[derive(Debug, Clone, PartialEq)]
enum Tok {
    Num(i64),
    Ident(String),
    Str(String),
    Op(&'static str),
    Open,
    Close,
}

fn tokenize(src: &str) -> Result<Vec<Tok>> {
    let chars: Vec<char> = src.chars().collect();
    let mut out = Vec::new();
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        match c {
            ' ' | '\t' | '\n' => i += 1,
            '(' => {
                out.push(Tok::Open);
                i += 1;
            }
            ')' => {
                out.push(Tok::Close);
                i += 1;
            }
            '"' | '\'' => {
                let end = chars[i + 1..]
                    .iter()
                    .position(|&d| d == c)
                    .map(|p| i + 1 + p);
                let Some(end) = end else {
                    bail!("unclosed quote")
                };
                out.push(Tok::Str(chars[i + 1..end].iter().collect()));
                i = end + 1;
            }
            '0'..='9' => {
                let start = i;
                while i < chars.len() && chars[i].is_ascii_digit() {
                    i += 1;
                }
                let s: String = chars[start..i].iter().collect();
                out.push(Tok::Num(s.parse()?));
            }
            c if c.is_alphabetic() || c == '_' => {
                let start = i;
                while i < chars.len() && (chars[i].is_alphanumeric() || chars[i] == '_') {
                    i += 1;
                }
                out.push(Tok::Ident(chars[start..i].iter().collect()));
            }
            _ => {
                let two: String = chars[i..(i + 2).min(chars.len())].iter().collect();
                let op = ["==", "!=", "<=", ">=", "&&", "||"]
                    .into_iter()
                    .find(|o| *o == two);
                if let Some(op) = op {
                    out.push(Tok::Op(op));
                    i += 2;
                } else if let Some(op) = ["<", ">", "+", "-", "!"]
                    .into_iter()
                    .find(|o| o.starts_with(c))
                {
                    out.push(Tok::Op(op));
                    i += 1;
                } else {
                    bail!("unexpected '{c}'");
                }
            }
        }
    }
    Ok(out)
}

struct Parser {
    toks: Vec<Tok>,
    pos: usize,
}

impl Parser {
    fn peek(&self) -> Option<&Tok> {
        self.toks.get(self.pos)
    }

    fn eat_op(&mut self, ops: &[&'static str]) -> Option<&'static str> {
        if let Some(Tok::Op(o)) = self.peek()
            && ops.contains(o)
        {
            let o = *o;
            self.pos += 1;
            return Some(o);
        }
        None
    }

    fn binary(
        &mut self,
        ops: &[&'static str],
        next: fn(&mut Self) -> Result<Expr>,
    ) -> Result<Expr> {
        let mut left = next(self)?;
        while let Some(o) = self.eat_op(ops) {
            let right = next(self)?;
            left = Expr::Bin(Box::new(left), op_of(o), Box::new(right));
        }
        Ok(left)
    }

    fn or(&mut self) -> Result<Expr> {
        self.binary(&["||"], Self::and)
    }

    fn and(&mut self) -> Result<Expr> {
        self.binary(&["&&"], Self::not)
    }

    fn not(&mut self) -> Result<Expr> {
        if self.eat_op(&["!"]).is_some() {
            return Ok(Expr::Not(Box::new(self.not()?)));
        }
        self.cmp()
    }

    fn cmp(&mut self) -> Result<Expr> {
        let left = self.sum()?;
        if let Some(o) = self.eat_op(&["==", "!=", "<=", ">=", "<", ">"]) {
            let right = self.sum()?;
            return Ok(Expr::Bin(Box::new(left), op_of(o), Box::new(right)));
        }
        Ok(left)
    }

    fn sum(&mut self) -> Result<Expr> {
        self.binary(&["+", "-"], Self::atom)
    }

    fn atom(&mut self) -> Result<Expr> {
        let tok = self.toks.get(self.pos).cloned();
        self.pos += 1;
        match tok {
            Some(Tok::Num(n)) => Ok(Expr::Num(n)),
            Some(Tok::Op("-")) => Ok(Expr::Bin(
                Box::new(Expr::Num(0)),
                Op::Sub,
                Box::new(self.atom()?),
            )),
            Some(Tok::Open) => {
                let e = self.or()?;
                if self.toks.get(self.pos) != Some(&Tok::Close) {
                    bail!("missing ')'");
                }
                self.pos += 1;
                Ok(e)
            }
            Some(Tok::Ident(name)) if name == "true" => Ok(Expr::Num(1)),
            Some(Tok::Ident(name)) if name == "false" => Ok(Expr::Num(0)),
            Some(Tok::Ident(name)) if self.peek() == Some(&Tok::Open) => {
                self.pos += 1;
                let arg = match self.toks.get(self.pos).cloned() {
                    Some(Tok::Str(s)) => s,
                    _ => bail!("{name}(...) needs a quoted name, like {name}(\"x\")"),
                };
                self.pos += 1;
                if self.toks.get(self.pos) != Some(&Tok::Close) {
                    bail!("missing ')' after {name}(\"{arg}\"");
                }
                self.pos += 1;
                match name.as_str() {
                    "flag" => Ok(Expr::Flag(arg)),
                    "item" => Ok(Expr::Item(arg)),
                    "visited" => Ok(Expr::Visited(arg)),
                    _ => bail!("unknown function '{name}' (use flag, item or visited)"),
                }
            }
            Some(Tok::Ident(name)) => Ok(Expr::Var(name)),
            Some(t) => bail!("unexpected {t:?}"),
            None => bail!("condition ends too early"),
        }
    }
}

fn op_of(o: &str) -> Op {
    match o {
        "+" => Op::Add,
        "-" => Op::Sub,
        "==" => Op::Eq,
        "!=" => Op::Ne,
        "<" => Op::Lt,
        "<=" => Op::Le,
        ">" => Op::Gt,
        ">=" => Op::Ge,
        "&&" => Op::And,
        _ => Op::Or,
    }
}

/// Parse a condition. Errors say what's wrong so authors can fix the story file.
pub fn parse(src: &str) -> Result<Expr> {
    let toks = tokenize(src).map_err(|e| anyhow::anyhow!("condition \"{src}\": {e}"))?;
    if toks.is_empty() {
        bail!("condition is empty");
    }
    let mut p = Parser { toks, pos: 0 };
    let e = p
        .or()
        .map_err(|e| anyhow::anyhow!("condition \"{src}\": {e}"))?;
    if p.pos != p.toks.len() {
        bail!("condition \"{src}\": unexpected {:?}", p.toks[p.pos]);
    }
    Ok(e)
}

impl Expr {
    pub fn value(&self, s: &State) -> i64 {
        let b = |x: bool| x as i64;
        match self {
            Expr::Num(n) => *n,
            Expr::Var(v) => s.vars.get(v).copied().unwrap_or(0),
            Expr::Flag(f) => b(s.flags.contains(f)),
            Expr::Item(i) => b(s.items.contains(i)),
            Expr::Visited(v) => b(s.visited.contains(v)),
            Expr::Not(e) => b(e.value(s) == 0),
            Expr::Bin(l, op, r) => {
                let (l, r) = (l.value(s), r.value(s));
                match op {
                    Op::Add => l.saturating_add(r),
                    Op::Sub => l.saturating_sub(r),
                    Op::Eq => b(l == r),
                    Op::Ne => b(l != r),
                    Op::Lt => b(l < r),
                    Op::Le => b(l <= r),
                    Op::Gt => b(l > r),
                    Op::Ge => b(l >= r),
                    Op::And => b(l != 0 && r != 0),
                    Op::Or => b(l != 0 || r != 0),
                }
            }
        }
    }

    pub fn holds(&self, s: &State) -> bool {
        self.value(s) != 0
    }

    /// Every counter name the condition reads.
    pub fn vars(&self, out: &mut Vec<String>) {
        match self {
            Expr::Var(v) => out.push(v.clone()),
            Expr::Not(e) => e.vars(out),
            Expr::Bin(l, _, r) => {
                l.vars(out);
                r.vars(out);
            }
            _ => {}
        }
    }
}

/// True when the condition holds. Story files are validated on load, so a bad
/// condition here only happens with hand-edited saves; it counts as false.
pub fn check(cond: Option<&str>, s: &State) -> bool {
    match cond {
        None => true,
        Some(c) => parse(c).map(|e| e.holds(s)).unwrap_or(false),
    }
}

/// Put counter values into text: `{trust}` becomes the number; `{{` and `}}`
/// are literal braces.
pub fn fill(text: &str, s: &State) -> String {
    if !text.contains('{') && !text.contains('}') {
        return text.to_string();
    }
    let mut out = String::with_capacity(text.len());
    let mut chars = text.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '{' if chars.peek() == Some(&'{') => {
                chars.next();
                out.push('{');
            }
            '}' if chars.peek() == Some(&'}') => {
                chars.next();
                out.push('}');
            }
            '{' => {
                let name: String = chars.by_ref().take_while(|&d| d != '}').collect();
                out.push_str(&s.vars.get(name.trim()).copied().unwrap_or(0).to_string());
            }
            _ => out.push(c),
        }
    }
    out
}

/// Check that `{...}` placeholders in text are well formed.
pub fn check_text(text: &str) -> Result<()> {
    let mut chars = text.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '{' if chars.peek() == Some(&'{') => {
                chars.next();
            }
            '}' if chars.peek() == Some(&'}') => {
                chars.next();
            }
            '{' => {
                let mut name = String::new();
                loop {
                    match chars.next() {
                        Some('}') => break,
                        Some(d) => name.push(d),
                        None => bail!("unclosed placeholder '{{{name}' (missing '}}')"),
                    }
                }
                let name = name.trim();
                if name.is_empty() || !name.chars().all(|c| c.is_alphanumeric() || c == '_') {
                    bail!(
                        "bad placeholder '{{{name}}}' (use {{counter_name}}, or {{{{ for a brace)"
                    );
                }
            }
            '}' => bail!("stray '}}' (use }}}} for a literal brace)"),
            _ => {}
        }
    }
    Ok(())
}
