// SPDX-FileCopyrightText: 2024 University of Rochester
//
// SPDX-License-Identifier: MIT

pub mod dump;
pub mod parse;
use clap::{ArgGroup, Parser, ValueEnum, ValueHint};
use clio::*;
use parse::iter_access_expr;
use parse::iter_if_conditions;
use parse::iter_loop_conditions;
use parse::parse_wgsl;
use std::io::Read;
use std::io::Write;
// use dump::dump;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum ConditionMode {
    /// Dump all conditional expressions (`If` statements and Loops)
    All,
    /// Dump only expressions for `If` statements
    If,
    /// Dump only expressions for Loop statements
    Loop,
}

#[derive(Parser)]
#[clap(
    name = "wgsl-dump",
    arg_required_else_help = true,
    about = "Extract and dump expressions from wgsl files.",
    long_about = "Extract and dump expressions from wgsl files.\n\nOne of --dump-conditions or --dump-indicies must be specified.",
    group(ArgGroup::new("mode").args(&["dump_conditions", "dump_indices"]).required(true))
)]
struct Opt {
    #[clap(
        short,
        long,
        value_parser,
        default_value("-"),
        value_hint(ValueHint::FilePath)
    )]
    input: Input,

    #[clap(long, value_parser, default_value = "-", help(""))]
    output: OutputPath,

    #[clap(long, value_enum, default_missing_value("All"), require_equals(true), num_args(0..=1), help("Specify condition dumping mode (`all` by default)"))]
    dump_conditions: Option<ConditionMode>,

    #[clap(long, help = "Index dumping mode (e.g. x[y+z])")]
    dump_indices: bool,
}

fn main() {
    let mut opt = Opt::parse();
    let mut buf = String::new();
    opt.input.read_to_string(&mut buf).unwrap();

    let module = parse_wgsl(&buf).unwrap();

    let exprs: Vec<(&naga::Function, naga::Handle<naga::Expression>)> =
        if let Some(mode) = opt.dump_conditions {
            match mode {
                ConditionMode::All => iter_if_conditions(&module)
                    .chain(iter_loop_conditions(&module))
                    .collect(),
                ConditionMode::If => iter_if_conditions(&module).collect(),
                ConditionMode::Loop => iter_loop_conditions(&module).collect(),
            }
        } else if opt.dump_indices {
            iter_access_expr(&module).collect()
        } else {
            panic!("No mode selected.");
        };

    let mut out = opt.output.create().unwrap();

    for (fun, expr) in exprs {
        writeln!(out, "{}", dump::write_expression(&buf, fun, &expr))
            .expect("Failed to write data");
    }
}
