use super::lexer::Token;
use std::rc::Rc;

pub enum TreeNode {
    Apply(Rc<TreeNode>, Rc<TreeNode>),
    Dot(char),
    I,
    S,
    K,
    R,
    E
}

pub fn parse(tokens: Vec<Token>) -> Rc<TreeNode> {
    parse_helper(&mut tokens.into_iter())
}

fn parse_helper<I>(tokens: &mut I) -> Rc<TreeNode>
    where I: Iterator<Item=Token> {
    match tokens.next() {
        None => panic!("Not enough tokens!"),
        Some(Token::Dot(c)) => Rc::new(TreeNode::Dot(c)),
        Some(Token::I) => Rc::new(TreeNode::I),
        Some(Token::S) => Rc::new(TreeNode::S),
        Some(Token::K) => Rc::new(TreeNode::K),
        Some(Token::R) => Rc::new(TreeNode::R),
        Some(Token::E) => Rc::new(TreeNode::E),
        Some(Token::Apply) => {
            let func = parse_helper(tokens);
            let arg = parse_helper(tokens);
            Rc::new(TreeNode::Apply(func, arg))
        }
    }
}