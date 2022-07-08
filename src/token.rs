use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Token {
    Space,
    Tab,
    NewLine,
}

impl From<&Token> for i32 {
    fn from(t: &Token) -> Self {
        match t {
            Token::Space => 0,
            Token::Tab => 1,
            Token::NewLine => panic!("cannot convert token to digit"),
        }
    }
}

pub type Number = i32;
pub type Label = Vec<Token>;

pub fn label_to_string(label: &Label) -> String {
    label
        .iter()
        .map(|ch| match ch {
            Token::Space => '.',
            Token::Tab => '-',
            Token::NewLine => panic!("invalid label"),
        })
        .collect::<String>()
}

#[derive(Debug)]
pub enum Command {
    Stack(StackOP),
    Arith(ArithOP),
    Heap(HeapOP),
    IO(IOOP),
    Flow(FlowOP),
}

impl Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Command::Stack(op) => write!(f, "Stack: {}", op),
            Command::Arith(op) => write!(f, "Arith: {}", op),
            Command::Heap(op) => write!(f, "Heap: {}", op),
            Command::IO(op) => write!(f, "IO: {}", op),
            Command::Flow(op) => write!(f, "Flow: {}", op),
        }
    }
}

#[derive(Debug)]
pub enum StackOP {
    Push(Number),
    Duplicate,
    DuplicateNth(Number),
    Discard,
    DiscardTopN(Number),
    Swap,
}

impl Display for StackOP {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StackOP::Push(num) => write!(f, "Push({})", num),
            StackOP::Duplicate => write!(f, "Duplicate"),
            StackOP::DuplicateNth(num) => write!(f, "DuplicateNth({})", num),
            StackOP::Discard => write!(f, "Discard"),
            StackOP::DiscardTopN(num) => write!(f, "DiscardTopN({})", num),
            StackOP::Swap => write!(f, "Swap"),
        }
    }
}

#[derive(Debug)]
pub enum ArithOP {
    Sum,
    Sub,
    Mul,
    Div,
    Rem,
}

impl Display for ArithOP {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArithOP::Sum => write!(f, "Sum"),
            ArithOP::Sub => write!(f, "Sub"),
            ArithOP::Mul => write!(f, "Mul"),
            ArithOP::Div => write!(f, "Div"),
            ArithOP::Rem => write!(f, "Rem"),
        }
    }
}

#[derive(Debug)]
pub enum HeapOP {
    StoreAt,
    StoreAtStack,
}

impl Display for HeapOP {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HeapOP::StoreAt => write!(f, "StoreAt"),
            HeapOP::StoreAtStack => write!(f, "StoreAtStack"),
        }
    }
}

#[derive(Debug)]
pub enum IOOP {
    PrintChar,
    PrintNum,
    ReadChar,
    ReadNum,
}

impl Display for IOOP {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IOOP::PrintChar => write!(f, "PrintChar"),
            IOOP::PrintNum => write!(f, "PrintNum"),
            IOOP::ReadChar => write!(f, "ReadChar"),
            IOOP::ReadNum => write!(f, "ReadNum"),
        }
    }
}

#[derive(Debug)]
pub enum FlowOP {
    Mark(Label),
    Call(Label),
    Jump(Label),
    BranchZero(Label),
    BranchLt(Label),
    Return,
    Exit,
}

impl Display for FlowOP {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FlowOP::Mark(lbl) => write!(f, "Mark({})", label_to_string(lbl)),
            FlowOP::Call(lbl) => write!(f, "Call({})", label_to_string(lbl)),
            FlowOP::Jump(lbl) => write!(f, "Jump({})", label_to_string(lbl)),
            FlowOP::BranchZero(lbl) => write!(f, "BranchZero({})", label_to_string(lbl)),
            FlowOP::BranchLt(lbl) => write!(f, "BranchLt({})", label_to_string(lbl)),
            FlowOP::Return => write!(f, "Return"),
            FlowOP::Exit => write!(f, "Exit"),
        }
    }
}
