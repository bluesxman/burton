
// AST //////////////////////////////

enum Keyword {
    If,
    Define,
    Quote,
    Lambda,
    Set
}

enum Symbol {
    Keyword,
    NonKeyword(str)
}

enum Atom {
    Number,
    Symbol
}

enum SymbolicExpr {
    Atom,
    ListExpr(Vec<SymbolicExpr>)
}

fn main() {
    let x = 5i;

    println!("Hello, world!");
    println!("{}", x);
}
