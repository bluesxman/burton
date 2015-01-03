
let DEFAULT_ATOM_SIZE = 32u;

enum SymbolicExpr {
    Number(f64),
    Symbol(&str),
    ListExpr(Vec<SymbolicExpr>)
}

enum State {
    Start,
    List,
    Symbol,
    Integer,
    IncompleteFloating,
    Floating,
}

fn to_atom(state: State, accum: &str) -> Result<SymbolicExpr, &str> {
    match (state) {
        State::Symbol => {
            SymbolicExpr::Symbol(accum.to_slice());
        }
        State::Integer | State::Floating => {
            match (from_str(accum.to_slice())) {
                Some(i) => {
                    SymbolicExpr::Number(i as f64);
                }
                None => return Err("Cannot parse number")
            }
        }
        _ => {
            return Err("Invalid atom");
        }
    }
}

// Non-recursive parse using state machine
fn read(code: &str) -> Result<Vec<SymbolicExpr>, &str> {
    let mut accum = String::with_capacity(DEFAULT_ATOM_SIZE);
    let mut chars = code.to_string();
    let mut exprs = Vec::new();
    let mut stack = Vec::new();
    let mut state = State::Start;

    loop {
        match (chars.pop()) {
            None => {
                if(state != State::Start) {
                    match (to_atom(state, accum.to_slice())) {
                        Ok(sexpr) => exprs.push(sexpr);
                        e @ Err(_) => return e
                    }
                    return Ok(exprs)
                }
            }

            // Whitespace which can only terminate atoms
            Some(' ') | Some('\f') | Some('\n')| Some('\r') | Some('\t') | Some('\v') => {
                if(state != State::Start) {
                    match (to_atom(state, accum.to_slice())) {
                        Ok(sexpr) => {
                            exprs.push(sexpr);
                            accum.clear();
                            state = State::Start;
                        }
                        e @ Err(_) => return e
                    }
                }
            }

            Some(c) => {
                match (state, c) {
                    (State::Start, '(') => {
                        state = State::List;
                        stack.push(exprs);
                        exprs = Vec::new();
                    }

                    (State::List, ')') => {
                        if(state != State::Start) {
                            match (to_atom(state, accum.to_slice())) {
                                Ok(sexpr) => {
                                    exprs.push(sexpr);
                                    accum.clear();
                                }
                                e @ Err(_) => return e
                            }
                        }
                        let list = SymbolicExpr::ListExpr(exprs);
                        state = State::Start;
                        exprs = match stack.pop() {
                            Some(parent) => {
                                parent.push(list);
                                parent
                            }
                            None => return Err("Missing '('")
                        }
                    }

                    (State::Start, '0' ... '9') => {
                        state = State::Integer;
                        accum.push(c);
                    }

                    (State::Start, _) => {
                        state = State::Symbol;
                        accum.push(c);
                    }

                    (State::Integer, '.') => {
                        state = State::IncompleteFloating;
                        accum.push(c);
                    }

                    (State::IncompleteFloating, '0' ... '9') => {
                        state = State::Floating;
                        accum.push(c);
                    }

                    (State::Integer, '0' ... '9') | (State::Floating, '0' ... '9') => {
                        accum.push(c);
                    }

                    (State::Integer, _) | (State::Floating, _) | (State::IncompleteFloating, _) =>
                        return Err("Invalid number");

                    (State::Symbol, _) => {
                        accum.push(c);
                    }

                    _ => return Err("Parse error")
                }
            }
        }
    }
}

fn print_read(ast: Result<Vec<SymbolicExpr>, &str>) {
    match (ast) {
        Ok(sexprs) => {
            for(s in sexprs) {
                match (s) {
                    Number(n) => println!("Number({})", n);
                    Symbol(x) => println!("Symbol({})", x);
                    _ => println!("List");
                }
            }
        }
        Err(s) => println! s;
    }
}

fn main() {
    let code = "12.3"

    print_read(read(code));
}
