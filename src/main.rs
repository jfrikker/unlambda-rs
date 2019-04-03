mod lexer;
mod parser;
mod unlambda;

use unlambda::evaluate;

fn main() {
    let prog = r"```s``s``sii`ki`k.*``s``s`ks``s`k`s`ks``s``s`ks``s`k`s`kr``s`k`sikk`k``s`ksk";
    match evaluate(prog, std::io::stdout()) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}
