use crate::token::{ArithOP, Command, FlowOP, HeapOP, Label, Number, StackOP, Token, IOOP};

pub struct Lexer {
    pub source: Vec<Token>,
    index: usize,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Self {
            source: source
                .chars()
                .filter_map(|ch| match ch {
                    ' ' => Some(Token::Space),
                    '\t' => Some(Token::Tab),
                    '\n' => Some(Token::NewLine),
                    _ => None,
                })
                .collect(),
            index: 0,
        }
    }

    fn next(&mut self) -> Result<Token, &'static str> {
        if self.index >= self.source.len() {
            Err("EOF")
        } else {
            let index = self.index;
            self.index += 1;
            Ok(self.source[index])
        }
    }

    fn take_till_terminal(&mut self) -> Vec<Token> {
        let items: Vec<Token> = self.source[self.index..]
            .iter()
            .take_while(|x| **x != Token::NewLine)
            .map(|t| t.to_owned())
            .collect();

        self.index += items.len() + 1;
        items
    }

    fn parse_number(&mut self) -> Result<Number, &'static str> {
        let sign = match self.next()? {
            Token::Tab => -1,
            Token::Space => 1,
            Token::NewLine => {
                println!("index: {}", self.index);
                return Err("invalid number");
            }
        };

        let (number, _) = self
            .take_till_terminal()
            .iter()
            .fold((0_i32, 0), |(sum, pos), t| {
                let value: i32 = t.into();
                (sum + 2_i32.pow(pos) * value, pos + 1)
            });

        Ok(number * sign)
    }

    fn parse_label(&mut self) -> Label {
        self.take_till_terminal()
    }

    fn parse_stack_imp(&mut self) -> Result<StackOP, &'static str> {
        let result = match self.next()? {
            Token::Space => StackOP::Push(self.parse_number()?),
            Token::Tab => match self.next()? {
                Token::Space => StackOP::DuplicateNth(self.parse_number()?),
                Token::NewLine => StackOP::DiscardTopN(self.parse_number()?),
                Token::Tab => panic!("invalid stack imp"),
            },
            Token::NewLine => match self.next()? {
                Token::Space => StackOP::Duplicate,
                Token::NewLine => StackOP::Discard,
                Token::Tab => StackOP::Swap,
            },
        };
        Ok(result)
    }

    fn parse_arith_imp(&mut self) -> Result<ArithOP, &'static str> {
        let result = match self.next()? {
            Token::Space => match self.next()? {
                Token::Space => ArithOP::Sum,
                Token::NewLine => ArithOP::Mul,
                Token::Tab => ArithOP::Sub,
            },
            Token::Tab => match self.next()? {
                Token::Space => ArithOP::Div,
                Token::Tab => ArithOP::Rem,
                Token::NewLine => return Err("invalid arith imp"),
            },
            Token::NewLine => return Err("invalid arith imp"),
        };
        Ok(result)
    }

    fn parse_heap_imp(&mut self) -> Result<HeapOP, &'static str> {
        let result = match self.next()? {
            Token::Space => HeapOP::StoreAt,
            Token::Tab => HeapOP::StoreAtStack,
            Token::NewLine => return Err("invalid heap imp"),
        };
        Ok(result)
    }

    fn parse_io_imp(&mut self) -> Result<IOOP, &'static str> {
        let result = match self.next()? {
            Token::Space => match self.next()? {
                Token::Space => IOOP::PrintChar,
                Token::Tab => IOOP::PrintNum,
                Token::NewLine => return Err("invalid io imp"),
            },
            Token::Tab => match self.next()? {
                Token::Space => IOOP::ReadChar,
                Token::Tab => IOOP::ReadNum,
                Token::NewLine => return Err("invalid io imp"),
            },
            Token::NewLine => return Err("invalid io imp"),
        };
        Ok(result)
    }

    fn parse_flow_imp(&mut self) -> Result<FlowOP, &'static str> {
        let result = match self.next()? {
            Token::Space => match self.next()? {
                Token::Space => FlowOP::Mark(self.parse_label()),
                Token::Tab => FlowOP::Call(self.parse_label()),
                Token::NewLine => FlowOP::Jump(self.parse_label()),
            },
            Token::Tab => match self.next()? {
                Token::Space => FlowOP::BranchZero(self.parse_label()),
                Token::Tab => FlowOP::BranchLt(self.parse_label()),
                Token::NewLine => FlowOP::Return,
            },
            Token::NewLine => match self.next()? {
                Token::NewLine => FlowOP::Exit,
                Token::Tab | Token::Space => return Err("invalid flow imp"),
            },
        };
        Ok(result)
    }

    pub fn parse(&mut self) -> Result<Vec<Command>, &'static str> {
        let mut commands = Vec::new();

        loop {
            let command = match self.next() {
                Ok(Token::Space) => Command::Stack(self.parse_stack_imp()?),
                Ok(Token::Tab) => match self.next()? {
                    Token::Space => Command::Arith(self.parse_arith_imp()?),
                    Token::Tab => Command::Heap(self.parse_heap_imp()?),
                    Token::NewLine => Command::IO(self.parse_io_imp()?),
                },
                Ok(Token::NewLine) => Command::Flow(self.parse_flow_imp()?),
                Err(e) if e == "EOF" => break,
                Err(e) => return Err(e),
            };

            commands.push(command);
        }
        Ok(commands)
    }
}
