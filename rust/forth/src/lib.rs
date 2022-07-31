pub type Value = i32;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug)]
enum Op {
    Number(Value),
    Add,
    Subtract,
    Multiply,
    Divide,
    Dup,
    Drop,
    Swap,
    Over,
    Nop,
    AliasCall(u32),
}

#[derive(Debug)]
struct Alias {
    name: String,
    ops: Vec<Op>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

pub struct Forth {
    stack: Vec<Value>,
    aliases: Vec<Alias>,
}

impl Forth {
    pub fn new() -> Forth {
        Forth {
            stack: Vec::new(),
            aliases: Vec::new(),
        }
    }

    pub fn stack(&self) -> &[Value] {
        &self.stack[..]
    }

    pub fn eval(&mut self, input: &str) -> Result<()> {
        let ops = self.parse(input)?;
        self.execute(ops)?;
        Ok(())
    }

    fn parse(&mut self, input: &str) -> Result<Vec<Op>> {
        let mut result: Vec<Op> = Vec::new();
        let mut stream = input.split(" ").map(|it| it.to_lowercase());
        while let Some(word) = stream.next() {
            result.push(self.parse_word(&word, &mut stream)?);
        }
        Ok(result)
    }

    fn execute(&mut self, ops: Vec<Op>) -> Result<()> {
        for op in ops {
            match op {
                Op::Number(number) => {
                    self.stack.push(number);
                }
                Op::Add => self.binary_op(|a, b| Ok(a + b))?,
                Op::Subtract => self.binary_op(|a, b| Ok(a - b))?,
                Op::Multiply => self.binary_op(|a, b| Ok(a * b))?,
                Op::Divide => self.binary_op(|a, b| match b {
                    0 => Err(Error::DivisionByZero),
                    _ => Ok(a / b),
                })?,
                Op::Dup => self.duplicate()?,
                Op::Drop => self.drop()?,
                Op::Swap => self.swap()?,
                Op::Over => self.over()?,
                Op::Nop => (),
                Op::AliasCall(idx) => {
                    let alias = self.aliases.iter().nth(idx as usize).unwrap();
                    self.execute(alias.ops.clone())?;
                }
            };
        }
        Ok(())
    }

    fn parse_word(&mut self, word: &str, stream: &mut impl Iterator<Item = String>) -> Result<Op> {
        let op = match &word[..] {
            value if self.alias_idx_by_name(value).is_some() => {
                Op::AliasCall(self.alias_idx_by_name(value).unwrap())
            }
            value if value.chars().all(|it| it.is_digit(10)) => {
                Op::Number(value.parse::<i32>().unwrap())
            }
            "+" => Op::Add,
            "-" => Op::Subtract,
            "*" => Op::Multiply,
            "/" => Op::Divide,
            "dup" => Op::Dup,
            "drop" => Op::Drop,
            "swap" => Op::Swap,
            "over" => Op::Over,
            ":" => self.parse_alias(stream)?,
            _ => return Err(Error::UnknownWord),
        };
        Ok(op)
    }

    fn parse_alias(&mut self, stream: &mut impl Iterator<Item = String>) -> Result<Op> {
        if let Some(alias) = stream.next() {
            if alias.chars().all(|it| it.is_digit(10)) {
                return Err(Error::InvalidWord);
            }
            let mut commands: Vec<String> = vec![];
            for word in stream {
                match &word[..] {
                    ";" => {
                        let ops = self.parse(&commands.join(" "))?;
                        self.aliases.push(Alias { name: alias, ops });
                        return Ok(Op::Nop);
                    }
                    _ => commands.push(word),
                }
            }
        }
        Err(Error::InvalidWord)
    }

    fn alias_idx_by_name(&self, word: &str) -> Option<u32> {
        self.aliases
            .iter()
            .enumerate()
            .rev()
            .find(|(_, it)| it.name == word)
            .map(|(idx, _)| idx as u32)
    }

    fn binary_op(&mut self, lambda: impl FnOnce(i32, i32) -> Result<i32>) -> Result<()> {
        let v2 = self.pop()?;
        let v1 = self.pop()?;
        self.stack.push(lambda(v1, v2)?);
        Ok(())
    }

    fn duplicate(&mut self) -> Result<()> {
        let v = self.pop()?;
        self.stack.push(v);
        self.stack.push(v);
        Ok(())
    }

    fn drop(&mut self) -> Result<()> {
        self.pop()?;
        Ok(())
    }

    fn swap(&mut self) -> Result<()> {
        let v2 = self.pop()?;
        let v1 = self.pop()?;
        self.stack.push(v2);
        self.stack.push(v1);
        Ok(())
    }

    fn over(&mut self) -> Result<()> {
        let v2 = self.pop()?;
        let v1 = self.pop()?;
        self.stack.push(v1);
        self.stack.push(v2);
        self.stack.push(v1);
        Ok(())
    }

    fn pop(&mut self) -> Result<i32> {
        if let Some(value) = self.stack.pop() {
            Ok(value)
        } else {
            Err(Error::StackUnderflow)
        }
    }
}
