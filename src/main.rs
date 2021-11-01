use std::fs::{self, File};
use std::io::Write;

use structopt::StructOpt;

use racoon::compiler::{
    irbuilder::*,
    syntax::{*, visitor::AstVisitorMut},
};

mod options;

fn main() {
    let options = options::Options::from_args();

    let input_file = options.input_file;
    let input = fs::read_to_string(input_file)
        .expect("Failed to read from input file");

    let lexer = lexer::Lexer::new(input.chars());
    // println!("{:?}", lexer::Lexer::new(input.chars()).into_iter().collect_vec());

    let mut parser = parser::Parser::new(lexer);
    // println!("{:?}", parser::Parser::new(lexer::Lexer::new(input.chars())).parse());
    let mut ast = match parser.parse() {
        Ok(p) => p,
        Err(e) => {
            println!("{:?}", e);
            return;
        }
    };

    let mut ty_checker = typeck::TypeChecker::new();
    if let Err(e) = ty_checker.visit_program(&mut ast) {
        println!("{:?}", e);
        return;
    };

    let mut ir_builder = irbuilder::IrBuilder::new();
    let ir = match ir_builder.visit(&ast) {
        Ok(_) => ir_builder.ctx.cur_module,
        Err(e) => {
            println!("{:?}", e);
            return;
        }
    };

    let output_file = options.output_file;
    let mut output = File::create(output_file)
        .expect("Failed to open or create output file");
    writeln!(output, "{}", ir).expect("Failed to write output file");
}
