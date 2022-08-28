pub type Value = i32;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, PartialEq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

#[derive(Debug, Copy, Clone)]
enum Token {
    Add,
    Sub,
    Mul,
    Div,
    Dup,
    Drop,
    Swap,
    Over,
    Num(i32),
    Def(usize),
}

#[derive(Debug, Clone)]
struct Def {
    name: String,
    tokens: Vec<Token>,
}

pub struct Forth {
    stack: Vec<Value>,
    custom: Vec<Def>,
}

impl Forth {
    pub fn new() -> Forth {
        Forth {
            stack: vec![],
            custom: vec![],
        }
    }

    pub fn stack(&self) -> &[Value] {
        &self.stack
    }

    pub fn eval(&mut self, input: &str) -> Result<()> {
        let mut words = input.split(" ").map(|s| s.to_lowercase());
        while let Some(word) = words.next() {
            if word == ":" {
                self.parse_new_def(&mut words)?
            } else {
                self.handle_token(self.to_token(&word)?)?;
            }
        }
        Ok(())
    }

    fn to_token(&self, word: &str) -> Result<Token> {
        if let Some(idx) = self.find_latest_def(&word) {
            return Ok(Token::Def(idx));
        }
        let token = match word {
            "+" => Token::Add,
            "-" => Token::Sub,
            "*" => Token::Mul,
            "/" => Token::Div,
            "dup" => Token::Dup,
            "drop" => Token::Drop,
            "swap" => Token::Swap,
            "over" => Token::Over,
            w => match w.parse::<Value>() {
                Ok(n) => Token::Num(n),
                _ => return Err(Error::UnknownWord),
            },
        };
        Ok(token)
    }
    fn handle_token(&mut self, token: Token) -> Result<()> {
        match token {
            Token::Add => {
                let a = self.pop_stack()?;
                let b = self.pop_stack()?;
                self.push_stack(b + a);
            }
            Token::Sub => {
                let a = self.pop_stack()?;
                let b = self.pop_stack()?;
                self.push_stack(b - a);
            }
            Token::Mul => {
                let a = self.pop_stack()?;
                let b = self.pop_stack()?;
                self.push_stack(b * a);
            }
            Token::Div => {
                let a = self.pop_stack()?;
                let b = self.pop_stack()?;
                if a == 0 {
                    return Err(Error::DivisionByZero);
                }
                self.push_stack(b / a);
            }
            Token::Dup => {
                let a = self.get_stack_back(0)?;
                self.push_stack(a);
            }
            Token::Drop => {
                self.pop_stack()?;
            }
            Token::Swap => {
                let a = self.pop_stack()?;
                let b = self.pop_stack()?;
                self.push_stack(a);
                self.push_stack(b);
            }
            Token::Over => {
                let b = self.get_stack_back(1)?;
                self.push_stack(b);
            }
            Token::Num(n) => self.push_stack(n),
            Token::Def(i) => {
                for t in self.custom[i].tokens.clone() {
                    self.handle_token(t)?
                }
            }
        }
        Ok(())
    }

    fn find_latest_def(&self, name: &str) -> Option<usize> {
        self.custom
            .iter()
            .enumerate()
            .rfind(|(_, Def { name: n, .. })| *n == name)
            .map(|(i, _)| i)
    }

    fn parse_new_def<T>(&mut self, words: &mut T) -> Result<()>
    where
        T: Iterator<Item = String>,
    {
        let name = words.next().ok_or(Error::InvalidWord)?;
        if name.parse::<Value>().is_ok() {
            return Err(Error::InvalidWord);
        }
        let mut tokens = vec![];
        while let Some(word) = words.next() {
            if word == ";" {
                self.custom.push(Def { name, tokens });
                return Ok(());
            }
            tokens.push(self.to_token(&word)?);
        }
        Err(Error::InvalidWord)
    }

    fn pop_stack(&mut self) -> Result<Value> {
        self.stack.pop().ok_or(Error::StackUnderflow)
    }

    fn push_stack(&mut self, value: Value) {
        self.stack.push(value)
    }

    fn get_stack_back(&mut self, index: usize) -> Result<Value> {
        if index >= self.stack.len() {
            return Err(Error::StackUnderflow);
        }
        Ok(self.stack[self.stack.len() - index - 1])
    }
}
