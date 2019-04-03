mod lexer;
mod parser;
mod unlambda;

use unlambda::evaluate;

fn main() {
    let prog = r"`.d`.c`.d`.c`.d`.c`.d``e
`````````````.H.e.l.l.o.,. .W.o.r.l.dii```````````````iid.l.r.o.W. .,.o.l.l.e.H.`````````````
e``d.`c.`d.`c.`d.`c.`d.`";
    match evaluate(prog, std::io::stdout()) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}
