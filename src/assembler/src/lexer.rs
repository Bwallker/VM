use color_eyre::eyre::eyre;
use color_eyre::Result;
use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "./grammar.pest"] // relative to src
struct Grammar;

pub fn lex(program: &str) -> Result<Pair<Rule>> {
    println!("Entered lex!");
    let lexed = Grammar::parse(Rule::file, program);
    println!("Done lexing!");
    Ok(lexed.map_err(|x| eyre!(x))?.into_iter().next().unwrap())
}
