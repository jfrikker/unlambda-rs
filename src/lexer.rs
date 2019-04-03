pub enum Token {
    Apply,
    Dot(char),
    I,
    S,
    K,
    R,
    E
}

pub fn lex(program: &str) -> Result<Vec<Token>, Error> {
    let mut program = program.chars();
    let mut res = Vec::new();
    
    while let Some(c) = program.next() {
        let token = match c {
            '`' => Some(Token::Apply),
            '.' => match program.next() {
                Some(c) => Some(Token::Dot(c)),
                None => return Err(Error::UnexpectedEndOfInput)
            },
            'i' => Some(Token::I),
            's' => Some(Token::S),
            'k' => Some(Token::K),
            'r' => Some(Token::R),
            'e' => Some(Token::E),
            '\n' => None,
            _ => return Err(Error::IllegalCharacter(c))
        };
        token.map(|t| res.push(t));
    }

    Ok(res)
}

pub enum Error {
    IllegalCharacter(char),
    UnexpectedEndOfInput
}