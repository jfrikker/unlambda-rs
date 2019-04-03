pub enum Token {
    Apply,
    Dot(char),
    I,
    S,
    K,
    R
}

pub fn lex(program: &str) -> Result<Vec<Token>, Error> {
    let mut program = program.chars();
    let mut res = Vec::new();
    
    while let Some(c) = program.next() {
        let token = match c {
            '`' => Token::Apply,
            '.' => match program.next() {
                Some(c) => Token::Dot(c),
                None => return Err(Error::UnexpectedEndOfInput)
            },
            'i' => Token::I,
            's' => Token::S,
            'k' => Token::K,
            'r' => Token::R,
            _ => return Err(Error::IllegalCharacter(c))
        };
        res.push(token);
    }

    Ok(res)
}

pub enum Error {
    IllegalCharacter(char),
    UnexpectedEndOfInput
}