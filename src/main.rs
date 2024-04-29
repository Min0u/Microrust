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

// LISTE DES IMPORTS
use crate::r#type::Type;
use crate::parsing::value::Value;
use crate::error::{Error};
use crate::identifier::Identifier;
use std::any::{Any};
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