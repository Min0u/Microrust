// LISTE DES MODULES
mod parsing;
mod parser;
mod error;
mod identifier;
mod eval;
mod namespace;
mod namespacestack;
mod r#type;
mod heap;
mod memory;
mod memorycell;

// LISTE DES IMPORTS
use crate::r#type::Type;
use crate::parsing::value::Value;
use crate::error::{Error};
use crate::identifier::Identifier;
use std::io::{self, BufRead, Write};
use crate::parsing::instruction::Instruction;
use crate::namespacestack::NameSpaceStack;
use crate::parser::Parse;

// AFFICHAGE DU PROMPT
fn prompt() {
    print!("µRust # ");
    // Permet de vider le buffer de sortie
    io::stdout().flush().unwrap();
}

fn parse_exec(input: &str, nss: &mut NameSpaceStack) -> Result<(Option<Identifier>, Value), Error> {
    match Instruction::parse(input) {
        Ok(instr) => {
            instr.exec(nss).map_err(|err| Error::EvalError(err))
        }
        Err(e) => Err(Error::ParseError(e)),
    }
}

// FONCTION PRINCIPALE
fn main(){
    prompt();
    let ns = namespace::NameSpace::new();
    let mut nss = NameSpaceStack::new();
    nss.push(ns);

    for line in io::stdin().lock().lines() {
        let line = line.unwrap();

        match parse_exec(&line, &mut nss) {
            Ok((id, val)) => {
                if val == Value::Unit {
                    println!("{} : unit = ()", id.unwrap_or(Identifier::from("-")));
                } else {
                    println!("{} : {} = {}", id.unwrap_or(Identifier::from("-")), Type::from(&val), val);
                }
            }
            Err(e) => {
                println!("{}", e);
            }
        }
        prompt();
    }
}

// Tests de parse_exec
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut nss = NameSpaceStack::new();
        let ns = namespace::NameSpace::new();
        nss.push(ns);

        // 1 + 1 => - : isize = 2
        let r = parse_exec("1 + 1", &mut nss);
        let s2 = "- : isize = 2";
        if let Ok((id, val)) = r {
            if val == Value::Unit {
                assert_eq!(format!("{} : unit = ()", id.unwrap_or(Identifier::from("-"))), s2)

            } else {
                assert_eq!(format!("{} : {} = {}", id.unwrap_or(Identifier::from("-")), Type::from(&val), val), s2);
            }
        }

        // 1 + 2 - 3 * 4 / 5 => - : isize = 1
        let r = parse_exec("1 + 2 - 3 * 4 / 5", &mut nss);
        let s2 = "- : isize = 1";
        if let Ok((id, val)) = r {
            if val == Value::Unit {
                assert_eq!(format!("{} : unit = ()", id.unwrap_or(Identifier::from("-"))), s2)

            } else {
                assert_eq!(format!("{} : {} = {}", id.unwrap_or(Identifier::from("-")), Type::from(&val), val), s2);
            }
        }

        // let un = 1 => un : isize = 1
        let r = parse_exec("let un = 1", &mut nss);
        let s2 = "un : isize = 1";
        if let Ok((id, val)) = r {
            if val == Value::Unit {
                assert_eq!(format!("{} : unit = ()", id.unwrap_or(Identifier::from("-"))), s2)

            } else {
                assert_eq!(format!("{} : {} = {}", id.unwrap_or(Identifier::from("-")), Type::from(&val), val), s2);
            }
        }

        // un => - : isize = 1
        let r = parse_exec("un", &mut nss);
        let s2 = "- : isize = 1";
        if let Ok((id, val)) = r {
            if val == Value::Unit {
                assert_eq!(format!("{} : unit = ()", id.unwrap_or(Identifier::from("-"))), s2)

            } else {
                assert_eq!(format!("{} : {} = {}", id.unwrap_or(Identifier::from("-")), Type::from(&val), val), s2);
            }
        }

        // un + 1 => - : isize = 2
        let r = parse_exec("un + 1", &mut nss);
        let s2 = "- : isize = 2";
        if let Ok((id, val)) = r {
            if val == Value::Unit {
                assert_eq!(format!("{} : unit = ()", id.unwrap_or(Identifier::from("-"))), s2)

            } else {
                assert_eq!(format!("{} : {} = {}", id.unwrap_or(Identifier::from("-")), Type::from(&val), val), s2);
            }
        }

        // 1 / (1 - 1) => Evaluation Error: Division by zero, `(1 - 1)` evaluates to 0
        let r = parse_exec("1 / (1 - 1)", &mut nss);
        let s2 = "Evaluation Error: Division by zero, `(1 - 1)` evaluates to 0";
        if let Err(e) = r {
            assert_eq!(format!("{}", e), s2);
        }

        // let repete = 0 => repete : isize = 0
        let r = parse_exec("let repete = 0", &mut nss);
        let s2 = "repete : isize = 0";
        if let Ok((id, val)) = r {
            if val == Value::Unit {
                assert_eq!(format!("{} : unit = ()", id.unwrap_or(Identifier::from("-"))), s2)

            } else {
                assert_eq!(format!("{} : {} = {}", id.unwrap_or(Identifier::from("-")), Type::from(&val), val), s2);
            }
        }

        // let repete = 0 => Evaluation Error: Identifier `repete` already defined.
        let r = parse_exec("let repete = 0", &mut nss);
        let s2 = "Evaluation Error: Identifier `repete` already defined.";
        if let Err(e) = r {
            assert_eq!(format!("{}", e), s2);
        }

        // let pasdefini = 1 / 0 => Evaluation Error: Division by zero, `0' evaluates to 0
        let r = parse_exec("let pasdefini = 1 / 0", &mut nss);
        let s2 = "Evaluation Error: Division by zero, `0` evaluates to 0";
        if let Err(e) = r {
            assert_eq!(format!("{}", e), s2);
        }

        // let pasdefini = toujourspasdefini => Evaluation Error: Undefined identifier `toujourspasdefini`.
        let r = parse_exec("let pasdefini = toujourspasdefini", &mut nss);
        let s2 = "Evaluation Error: Undefined identifier `toujourspasdefini`.";
        if let Err(e) = r {
            assert_eq!(format!("{}", e), s2);
        }

        // pasdefini => Evaluation Error: Undefined identifier `pasdefini`.
        let r = parse_exec("pasdefini", &mut nss);
        let s2 = "Evaluation Error: Undefined identifier `pasdefini`.";
        if let Err(e) = r {
            assert_eq!(format!("{}", e), s2);
        }

        // let x = 0 => x : isize = 0
        let r = parse_exec("let x = 0", &mut nss);
        let s2 = "x : isize = 0";
        if let Ok((id, val)) = r {
            if val == Value::Unit {
                assert_eq!(format!("{} : unit = ()", id.unwrap_or(Identifier::from("-"))), s2)

            } else {
                assert_eq!(format!("{} : {} = {}", id.unwrap_or(Identifier::from("-")), Type::from(&val), val), s2);
            }
        }

        // {let x = 1; x+1} => - : isize = 2
        let r = parse_exec("{let x = 1; x+1}", &mut nss);
        let s2 = "- : isize = 2";
        if let Ok((id, val)) = r {
            if val == Value::Unit {
                assert_eq!(format!("{} : unit = ()", id.unwrap_or(Identifier::from("-"))), s2)

            } else {
                assert_eq!(format!("{} : {} = {}", id.unwrap_or(Identifier::from("-")), Type::from(&val), val), s2);
            }
        }

        // x + 1 => - : isize = 1
        let r = parse_exec("x + 1", &mut nss);
        let s2 = "- : isize = 1";
        if let Ok((id, val)) = r {
            if val == Value::Unit {
                assert_eq!(format!("{} : unit = ()", id.unwrap_or(Identifier::from("-"))), s2)

            } else {
                assert_eq!(format!("{} : {} = {}", id.unwrap_or(Identifier::from("-")), Type::from(&val), val), s2);
            }
        }

        // {let tmp = 0} => - : isize = 0
        let r = parse_exec("{let tmp = 0}", &mut nss);
        let s2 = "- : isize = 0";
        if let Ok((id, val)) = r {
            if val == Value::Unit {
                assert_eq!(format!("{} : unit = ()", id.unwrap_or(Identifier::from("-"))), s2)

            } else {
                assert_eq!(format!("{} : {} = {}", id.unwrap_or(Identifier::from("-")), Type::from(&val), val), s2);
            }
        }

        // tmp => Evaluation Error: Undefined identifier `tmp`.
        let r = parse_exec("tmp", &mut nss);
        let s2 = "Evaluation Error: Undefined identifier `tmp`.";
        if let Err(e) = r {
            assert_eq!(format!("{}", e), s2);
        }

        // 0 == 0 => - : bool = true
        let r = parse_exec("0 == 0", &mut nss);
        let s2 = "- : bool = true";
        if let Ok((id, val)) = r {
            if val == Value::Unit {
                assert_eq!(format!("{} : unit = ()", id.unwrap_or(Identifier::from("-"))), s2)

            } else {
                assert_eq!(format!("{} : {} = {}", id.unwrap_or(Identifier::from("-")), Type::from(&val), val), s2);
            }
        }

        // 1 - 1 != 1 + 1 => - : bool = true
        let r = parse_exec("1 - 1 != 1 + 1", &mut nss);
        let s2 = "- : bool = true";
        if let Ok((id, val)) = r {
            if val == Value::Unit {
                assert_eq!(format!("{} : unit = ()", id.unwrap_or(Identifier::from("-"))), s2)

            } else {
                assert_eq!(format!("{} : {} = {}", id.unwrap_or(Identifier::from("-")), Type::from(&val), val), s2);
            }
        }

        // true == false => - : bool = false
        let r = parse_exec("true == false", &mut nss);
        let s2 = "- : bool = false";
        if let Ok((id, val)) = r {
            if val == Value::Unit {
                assert_eq!(format!("{} : unit = ()", id.unwrap_or(Identifier::from("-"))), s2)

            } else {
                assert_eq!(format!("{} : {} = {}", id.unwrap_or(Identifier::from("-")), Type::from(&val), val), s2);
            }
        }

        // true == 1 => Evaluation Error: Type mismatch in expression `1`. Expected: bool. Found: isize
        let r = parse_exec("true == 1", &mut nss);
        let s2 = "Evaluation Error: Type mismatch in expression `1`. Expected: bool. Found: isize";
        if let Err(e) = r {
            assert_eq!(format!("{}", e), s2);
        }

        // true != 1 => Evaluation Error: Type mismatch in expression `1`. Expected: bool. Found: isize
        let r = parse_exec("true != 1", &mut nss);
        let s2 = "Evaluation Error: Type mismatch in expression `1`. Expected: bool. Found: isize";
        if let Err(e) = r {
            assert_eq!(format!("{}", e), s2);
        }

        // true == (1 == 1) => - : bool = true
        let r = parse_exec("true == (1 == 1)", &mut nss);
        let s2 = "- : bool = true";
        if let Ok((id, val)) = r {
            if val == Value::Unit {
                assert_eq!(format!("{} : unit = ()", id.unwrap_or(Identifier::from("-"))), s2)

            } else {
                assert_eq!(format!("{} : {} = {}", id.unwrap_or(Identifier::from("-")), Type::from(&val), val), s2);
            }
        }

        // 0 < 0  => - : bool = false
        let r = parse_exec("0 < 0", &mut nss);
        let s2 = "- : bool = false";
        if let Ok((id, val)) = r {
            if val == Value::Unit {
                assert_eq!(format!("{} : unit = ()", id.unwrap_or(Identifier::from("-"))), s2)

            } else {
                assert_eq!(format!("{} : {} = {}", id.unwrap_or(Identifier::from("-")), Type::from(&val), val), s2);
            }
        }

        // 1 - 1 >= un + 1
        let r = parse_exec("1 - 1 >= un + 1", &mut nss);
        let s2 = "- : bool = false";
        if let Ok((id, val)) = r {
            if val == Value::Unit {
                assert_eq!(format!("{} : unit = ()", id.unwrap_or(Identifier::from("-"))), s2)

            } else {
                assert_eq!(format!("{} : {} = {}", id.unwrap_or(Identifier::from("-")), Type::from(&val), val), s2);
            }
        }

        // true > false => Evaluation Error: Type mismatch in expression `true`. Expected: isize. Found: bool
        let r = parse_exec("true > false", &mut nss);
        let s2 = "Evaluation Error: Type mismatch in expression `true`. Expected: isize. Found: bool";
        if let Err(e) = r {
            assert_eq!(format!("{}", e), s2);
        }

        // true || false && un == 1
        let r = parse_exec("true || false && un == 1", &mut nss);
        let s2 = "- : bool = true";
        if let Ok((id, val)) = r {
            if val == Value::Unit {
                assert_eq!(format!("{} : unit = ()", id.unwrap_or(Identifier::from("-"))), s2)

            } else {
                assert_eq!(format!("{} : {} = {}", id.unwrap_or(Identifier::from("-")), Type::from(&val), val), s2);
            }
        }

        // x == 0 || 1 / x == 1 => - : bool = true
        let r = parse_exec("x == 0 || 1 / x == 1", &mut nss);
        let s2 = "- : bool = true";
        if let Ok((id, val)) = r {
            if val == Value::Unit {
                assert_eq!(format!("{} : unit = ()", id.unwrap_or(Identifier::from("-"))), s2)

            } else {
                assert_eq!(format!("{} : {} = {}", id.unwrap_or(Identifier::from("-")), Type::from(&val), val), s2);
            }
        }

        // 1 / x == 1 || x == 0 => Evaluation Error: Division by zero, `x` evaluates to 0
        let r = parse_exec("1 / x == 1 || x == 0", &mut nss);
        let s2 = "Evaluation Error: Division by zero, `x` evaluates to 0";
        if let Err(e) = r {
            assert_eq!(format!("{}", e), s2);
        }

        // (un == 1) ? 1 : 2 => - : isize = 1
        let r = parse_exec("(un == 1) ? 1 : 2", &mut nss);
        let s2 = "- : isize = 1";
        if let Ok((id, val)) = r {
            if val == Value::Unit {
                assert_eq!(format!("{} : unit = ()", id.unwrap_or(Identifier::from("-"))), s2)

            } else {
                assert_eq!(format!("{} : {} = {}", id.unwrap_or(Identifier::from("-")), Type::from(&val), val), s2);
            }
        }

        // (un != 1) ? 1 : 2 => - : isize = 2
        let r = parse_exec("(un != 1) ? 1 : 2", &mut nss);
        let s2 = "- : isize = 2";
        if let Ok((id, val)) = r {
            if val == Value::Unit {
                assert_eq!(format!("{} : unit = ()", id.unwrap_or(Identifier::from("-"))), s2)

            } else {
                assert_eq!(format!("{} : {} = {}", id.unwrap_or(Identifier::from("-")), Type::from(&val), val), s2);
            }
        }

        // let mut acc = 0
        let r = parse_exec("let mut acc = 0", &mut nss);
        let s2 = "acc : isize = 0";
        if let Ok((id, val)) = r {
            if val == Value::Unit {
                assert_eq!(format!("{} : unit = ()", id.unwrap_or(Identifier::from("-"))), s2)

            } else {
                assert_eq!(format!("{} : {} = {}", id.unwrap_or(Identifier::from("-")), Type::from(&val), val), s2);
            }
        }

        // acc = acc + 1 => - : isize = 1
        let r = parse_exec("acc = acc + 1", &mut nss);
        let s2 = "- : isize = 1";
        if let Ok((id, val)) = r {
            if val == Value::Unit {
                assert_eq!(format!("{} : unit = ()", id.unwrap_or(Identifier::from("-"))), s2)

            } else {
                assert_eq!(format!("{} : {} = {}", id.unwrap_or(Identifier::from("-")), Type::from(&val), val), s2);
            }
        }

        // acc => - : isize = 1
        let r = parse_exec("acc", &mut nss);
        let s2 = "- : isize = 1";
        if let Ok((id, val)) = r {
            if val == Value::Unit {
                assert_eq!(format!("{} : unit = ()", id.unwrap_or(Identifier::from("-"))), s2)

            } else {
                assert_eq!(format!("{} : {} = {}", id.unwrap_or(Identifier::from("-")), Type::from(&val), val), s2);
            }
        }

        // {let mut flag = false; flag = flag || x == 0; flag} => - : bool = true
        let r = parse_exec("{let mut flag = false; flag = flag || x == 0; flag}", &mut nss);
        let s2 = "- : bool = true";
        if let Ok((id, val)) = r {
            if val == Value::Unit {
                assert_eq!(format!("{} : unit = ()", id.unwrap_or(Identifier::from("-"))), s2)

            } else {
                assert_eq!(format!("{} : {} = {}", id.unwrap_or(Identifier::from("-")), Type::from(&val), val), s2);
            }
        }

        // {let mut mayfreeze = 0; {let mayfreeze = 1; mayfreeze = 2}} => Evaluation Error: Identifier `mayfreeze` is not mutable.
        let r = parse_exec("{let mut mayfreeze = 0; {let mayfreeze = 1; mayfreeze = 2}}", &mut nss);
        let s2 = "Evaluation Error: Value at `mayfreeze` is not mutable.";
        if let Err(e) = r {
            assert_eq!(format!("{}", e), s2);
        }

        // {let maymelt = 0; {let mut maymelt = 0; maymelt = 1}} => - : isize = 1
        let r = parse_exec("{let maymelt = 0; {let mut maymelt = 0; maymelt = 1}}", &mut nss);
        let s2 = "- : isize = 1";
        if let Ok((id, val)) = r {
            if val == Value::Unit {
                assert_eq!(format!("{} : unit = ()", id.unwrap_or(Identifier::from("-"))), s2)

            } else {
                assert_eq!(format!("{} : {} = {}", id.unwrap_or(Identifier::from("-")), Type::from(&val), val), s2);
            }
        }

        // if (acc == 0) { acc = acc + 1} else { acc = acc - 1} => - : isize = 0
        let r = parse_exec("if (acc == 0) { acc = acc + 1} else { acc = acc - 1}", &mut nss);
        let s2 = "- : isize = 0";
        if let Ok((id, val)) = r {
            if val == Value::Unit {
                assert_eq!(format!("{} : unit = ()", id.unwrap_or(Identifier::from("-"))), s2)

            } else {
                assert_eq!(format!("{} : {} = {}", id.unwrap_or(Identifier::from("-")), Type::from(&val), val), s2);
            }
        }

        // acc => - : isize = 0
        let r = parse_exec("acc", &mut nss);
        let s2 = "- : isize = 0";
        if let Ok((id, val)) = r {
            if val == Value::Unit {
                assert_eq!(format!("{} : unit = ()", id.unwrap_or(Identifier::from("-"))), s2)

            } else {
                assert_eq!(format!("{} : {} = {}", id.unwrap_or(Identifier::from("-")), Type::from(&val), val), s2);
            }
        }

        // let mut k = 0 => k : isize = 0
        let r = parse_exec("let mut k = 0", &mut nss);
        let s2 = "k : isize = 0";
        if let Ok((id, val)) = r {
            if val == Value::Unit {
                assert_eq!(format!("{} : unit = ()", id.unwrap_or(Identifier::from("-"))), s2)

            } else {
                assert_eq!(format!("{} : {} = {}", id.unwrap_or(Identifier::from("-")), Type::from(&val), val), s2);
            }
        }

        // while (k < 4) {k = k + 3} => - : unit = ()
        let r = parse_exec("while (k < 4) {k = k + 3}", &mut nss);
        let s2 = "- : unit = ()";
        if let Ok((id, val)) = r {
            if val == Value::Unit {
                assert_eq!(format!("{} : unit = ()", id.unwrap_or(Identifier::from("-"))), s2)

            } else {
                assert_eq!(format!("{} : {} = {}", id.unwrap_or(Identifier::from("-")), Type::from(&val), val), s2);
            }
        }

        // k => - : isize = 6
        let r = parse_exec("k", &mut nss);
        let s2 = "- : isize = 6";
        if let Ok((id, val)) = r {
            if val == Value::Unit {
                assert_eq!(format!("{} : unit = ()", id.unwrap_or(Identifier::from("-"))), s2)

            } else {
                assert_eq!(format!("{} : {} = {}", id.unwrap_or(Identifier::from("-")), Type::from(&val), val), s2);
            }
        }

        // let i = 0 => i : isize = 0
        let r = parse_exec("let i = 0", &mut nss);
        let s2 = "i : isize = 0";
        if let Ok((id, val)) = r {
            if val == Value::Unit {
                assert_eq!(format!("{} : unit = ()", id.unwrap_or(Identifier::from("-"))), s2)

            } else {
                assert_eq!(format!("{} : {} = {}", id.unwrap_or(Identifier::from("-")), Type::from(&val), val), s2);
            }
        }

        // &i => - : Ptr = @[0, i]
        let r = parse_exec("&i", &mut nss);
        let s2 = "- : Ptr = @[0, i]";
        if let Ok((id, val)) = r {
            if val == Value::Unit {
                assert_eq!(format!("{} : unit = ()", id.unwrap_or(Identifier::from("-"))), s2)

            } else {
                assert_eq!(format!("{} : {} = {}", id.unwrap_or(Identifier::from("-")), Type::from(&val), val), s2);
            }
        }

        // {let i = 8; &i} => - : Ptr = @[1, i]
        let r = parse_exec("{let i = 8; &i}", &mut nss);
        let s2 = "- : Ptr = @[1, i]";
        if let Ok((id, val)) = r {
            if val == Value::Unit {
                assert_eq!(format!("{} : unit = ()", id.unwrap_or(Identifier::from("-"))), s2)

            } else {
                assert_eq!(format!("{} : {} = {}", id.unwrap_or(Identifier::from("-")), Type::from(&val), val), s2);
            }
        }

        // {let y = 8; &i} => - : Ptr = @[0, i]
        let r = parse_exec("{let y = 8; &i}", &mut nss);
        let s2 = "- : Ptr = @[0, i]";
        if let Ok((id, val)) = r {
            if val == Value::Unit {
                assert_eq!(format!("{} : unit = ()", id.unwrap_or(Identifier::from("-"))), s2)

            } else {
                assert_eq!(format!("{} : {} = {}", id.unwrap_or(Identifier::from("-")), Type::from(&val), val), s2);
            }
        }
    }
}
