use std::fs::File;
use std::io::{self, Write};
use std::{env, error, vec};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        writeln!(io::stderr(), "Usage: generate_ast <output directory>").unwrap();
        return;
    }
    let output_dir = &args[1];
    let mut input = Vec::new();
    input.push(String::from(
        "Binary   : Expr left, Token operator, Expr right",
    ));
    input.push(String::from("Grouping : Expr expression"));
    input.push(String::from("Literal  : String value"));
    input.push(String::from("Unary    : Token operator, Expr right"));
    define_ast(output_dir, String::from("Expr"), input).unwrap();
}

fn define_ast(
    output_dir: &String,
    base_name: String,
    input: Vec<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let path = format!("{}/{}", output_dir, base_name);
    let mut data_file = File::create(path).expect("creation failed");

    let mut types = Vec::new();

    for x in &input {
        types.push(format!("{}", x.split(':').next().unwrap().trim()))
    }

    // Imports
    data_file.write(b"use crate::token::Token;\n\n")?;

    // Visitor trait
    data_file.write(b"pub trait Visitor<T> {\n")?;
    for t in &types {
        data_file.write(
            format!(
                "\tfn visit_{}(&mut self, {}: &{}) -> T;\n",
                t.to_ascii_lowercase(),
                t.to_ascii_lowercase(),
                t
            )
            .as_bytes(),
        )?;
    }
    data_file.write(b"}\n\n")?;

    // Expression enum
    data_file.write(b"\npub enum Expr {\n")?;
    for t in types.iter() {
        data_file.write(format!("\t{}(Box<{}>),\n", t, t).as_bytes())?;
    }
    data_file.write(b"}\n")?;

    // Defeinig structs
    for x in &input {
        let strs: Vec<&str> = x.split(':').collect();
        data_file.write(format!("struct {} {}\n", strs[0].trim(), '{').as_bytes())?;

        let fields: Vec<&str> = strs[1].trim().split(',').collect();
        for field in &fields {
            let f: Vec<&str> = field.trim().split(' ').collect();
            match f[0] {
                "Expr" => {
                    data_file.write(format!("\tpub {}: Box<{}>,\n", f[1], f[0]).as_bytes())?;
                }
                _ => {
                    data_file.write(format!("\tpub {}: {},\n", f[1], f[0]).as_bytes())?;
                }
            }
        }
        data_file.write(b"}\n\n")?;

        // Impls
        data_file.write(format!("impl {} {}\n", strs[0].trim(), '{').as_bytes())?;

        data_file.write(b"\tfn new(")?;

        for field in &fields {
            let f: Vec<&str> = field.trim().split(' ').collect();
            data_file.write(format!("{}: {}, ", f[1], f[0]).as_bytes())?;
        }
        data_file.write(format!(") -> {} {}\n", strs[0].trim(), '{').as_bytes())?;

        data_file.write(format!("\t\t{} {}\n", strs[0].trim(), '{').as_bytes())?;

        for field in &fields {
            let f: Vec<&str> = field.trim().split(' ').collect();
            match f[0] {
                "Expr" => {
                    data_file.write(format!("\t\t\t{}: Box::new({}),\n", f[1], f[1]).as_bytes())?;
                }
                _ => {
                    data_file.write(format!("\t\t\t{},\n", f[1]).as_bytes())?;
                }
            }
        }

        data_file.write(b"\t\t};\n")?;

        data_file.write(b"\t}\n")?;

        data_file.write(b"}\n\n")?;
    }

    Ok(())
}
