use crate::lexer::{self, lex};
use crate::parser::{TreeNode, parse};
use std::io::Write;
use std::mem::swap;
use std::rc::Rc;

pub type ValueRef = Rc<Value>;
pub type TreeRef = Rc<TreeNode>;
type PStack = Vec<StackFrame>;

pub enum Value {
    Dot(char),
    I,
    S,
    S1(ValueRef),
    S2(ValueRef, ValueRef),
    K,
    K1(ValueRef),
    R
}

enum StackFrame {
    EvaluatingFunc(TreeRef),
    EvaluatingArg(ValueRef)
}

pub fn evaluate<O>(program: &str, out: O) -> Result<ValueRef, Error>
    where O: Write {
    let tree = parse(lex(program)?);

    match &*tree {
        TreeNode::Apply(func, arg) => Evaluator::new(func.clone(), arg.clone(), out).evaluate(),
        t => Ok(convert_basic(&t))
    }
}

struct Evaluator<O> {
    next: TreeRef,
    stack: PStack,
    out: O
}

fn convert_basic(tree: &TreeNode) -> ValueRef {
    match *tree {
        TreeNode::Apply(_, _) => panic!("whoops"),
        TreeNode::Dot(c) => Rc::new(Value::Dot(c)),
        TreeNode::I => Rc::new(Value::I),
        TreeNode::S => Rc::new(Value::S),
        TreeNode::K => Rc::new(Value::K),
        TreeNode::R => Rc::new(Value::R)
    }
}

impl <O> Evaluator<O>
    where O: Write {
    fn new(func: TreeRef, arg: TreeRef, out: O) -> Evaluator<O> {
        let mut stack = Vec::new();
        stack.push(StackFrame::EvaluatingFunc(arg));
        Evaluator {
            next: func,
            stack,
            out
        }
    }

    fn evaluate(mut self) -> Result<ValueRef, Error> {
        let mut last = None;
        while !self.stack.is_empty() {
            let mut temp_last = None;
            swap(&mut last, &mut temp_last);
            match temp_last {
                Some(inner_last) => {
                    let frame = self.stack.pop().unwrap();
                    match frame {
                        StackFrame::EvaluatingFunc(arg) => {
                            self.stack.push(StackFrame::EvaluatingArg(inner_last));
                            self.next = arg;
                        },
                        StackFrame::EvaluatingArg(func) => last = Some(self.apply(func, inner_last)?)
                    }
                },
                None => {
                    match &*self.next {
                        TreeNode::Apply(func, arg) => {
                            self.stack.push(StackFrame::EvaluatingFunc(arg.clone()));
                            self.next = func.clone();
                        },
                        t => last = Some(convert_basic(&t))
                    }
                }
            }
        }

        Ok(last.unwrap())
    }
    
    fn apply(&mut self, func: ValueRef, arg: ValueRef) -> Result<ValueRef, Error> {
        match &*func {
            Value::Dot(c) => {
                write!(self.out, "{}", c).map_err(|e| Error::OutputError(e))?;
                Ok(arg)
            },
            Value::I => Ok(arg),
            Value::S => Ok(Rc::new(Value::S1(arg.clone()))),
            Value::S1(arg1) => Ok(Rc::new(Value::S2(arg1.clone(), arg))),
            Value::S2(arg1, arg2) => {
                let func = self.apply(arg1.clone(), arg.clone())?;
                let s_arg = self.apply(arg2.clone(), arg)?;
                self.apply(func, s_arg)
            },
            Value::K => Ok(Rc::new(Value::K1(arg))),
            Value::K1(arg1) => Ok(arg1.clone()),
            Value::R => {
                write!(self.out, "\n").map_err(|e| Error::OutputError(e))?;
                Ok(arg)
            }
        }
    }
}

#[derive(Debug)]
pub enum Error {
    IllegalCharacter(char),
    UnexpectedEndOfInput,
    OutputError(std::io::Error)
}

impl From<lexer::Error> for Error {
    fn from(err: lexer::Error) -> Error {
        match err {
            lexer::Error::IllegalCharacter(c) => Error::IllegalCharacter(c),
            lexer::Error::UnexpectedEndOfInput => Error::UnexpectedEndOfInput
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::IllegalCharacter(c) => write!(f, "Illegal character '{}'", c),
            Error::UnexpectedEndOfInput => write!(f, "Unexpected end of input"),
            Error::OutputError(underlying) => write!(f, "Error writing output: {}", underlying)
        }
    }
}