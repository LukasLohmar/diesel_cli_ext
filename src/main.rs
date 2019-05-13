#[macro_use]
extern crate nom;

use std::env;
use std::fs::File;
use std::io::prelude::*;
mod parse;

fn print_normal_dependencies(type_ndt: bool, type_bd: bool, type_ip: bool) {
    if type_ndt {
        println!("use chrono::NaiveDateTime;");
    }
    if type_bd {
        println!("use bigdecimal::BigDecimal;");
    }
    if type_ip {
        println!("use ipnetwork::IpNetwork;");
    }
}
fn print_conversion_dependencies() {
    //todo add selection for ndt and bd
    println!(
        "
use models;
use proto::client_service;
use std::str::FromStr;
use std::convert::From;"
    );
}
fn print_conversion_methods(type_ndt: bool, type_bd: bool) {
    //todo add selection for ndt and bd
    if type_ndt {
        println!(
            "
fn str2ndt(istr: &str) -> NaiveDateTime {{
    NaiveDateTime::parse_from_str(istr, \"%Y-%m-%d %H:%M:%S\").unwrap()
}}"
        );
    }

    if type_bd {
        println!(
            "
fn str2bd(istr: &str) -> BigDecimal{{
    BigDecimal::from_str(istr).unwrap()
}}"
        );
    }
}

fn main() {
    //Read in
    let args: Vec<_> = env::args().collect();
    let action;
    let mut derive: Option<&str> = None;
    let mut class_name = "";
    if args.len() < 2 {
        action = "model";
    } else {
        action = &args[1];
        if action == "into_proto" || action == "from_proto" && args.len() >= 3 {
            class_name = &args[2];
        } else {
            show_help();
        }
    }

    if action == "model" && args.len() == 3 {
        derive = Some(&args[2]);
    }

    let mut f = File::open("schema.rs")
        .expect("File not found. Please run in the directory with schema.rs.");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("Something went wrong reading the file.");

    let (
        str_proto,
        str_request,
        str_rpc,
        str_model,
        str_from_proto,
        str_into_proto,
        type_ndt,
        type_bd,
        type_ip,
    ) = parse::parse(contents, action, derive);
    //Output
    match action {
        "proto" => {
            println!("syntax = \"proto3\";\n\n");
            println!("{}\n", str_proto);
            println!("{}\n", str_request);
            println!("service MessageRpc {{\n{}}}", str_rpc);
        }
        "model" => {
            println!("// Generated by diesel_ext\n");
            println!("#![allow(unused)]");
            println!("#![allow(clippy::all)]\n");
            print_normal_dependencies(type_ndt, type_bd, type_ip);
            println!("{}", str_model);
        }
        "from_proto" => {
            print_conversion_dependencies();
            print_conversion_methods(type_ndt, type_bd);
            println!("{}", str_from_proto.replace("_name_", class_name));
        }
        "into_proto" => {
            print_conversion_dependencies();
            println!("{}", str_into_proto.replace("_name_", class_name));
        }
        _ => {
            show_help();
        }
    }
}

fn show_help() {
    println!(
        "
Diesel CLI extension Help
        
Usage:

    Generate models:
        diesel_ext (default, equals to: 'cargo run model')
        
        diesel_ext model 
        (default, equals to: 'cargo run model \"Debug,Queryable\"')

        diesel_ext model <derives> 
        (e.g. diesel_ext model \"Debug, Queryable, Identifiable, Associations, AsChangeset\")

    Generate protos:
        diesel_ext into_proto <ClassName> (Pick the ClassName you like)
        diesel_ext from_proto <ClassName> (Pick the ClassName you like)
            "
    );
    ::std::process::exit(0);
}
