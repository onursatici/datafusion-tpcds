use std::{collections::HashMap, path::Path};

use clap::Parser;
use datafusion::common::Result;
use regex::Regex;
use tokio::{
    fs::File,
    io::{AsyncBufReadExt, BufReader},
};

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    data_dir: String,

    #[arg(short, long)]
    queries_file: String,
}

#[derive(Debug)]
struct Query {
    number: usize,
    sql: String,
    template_name: String,
}

#[derive(Debug)]
enum ParserState {
    SearchingStart,
    CollectingQuery {
        number: usize,
        template: String,
        content: String,
    },
}

async fn parse_queries(file: &Path) -> Result<Vec<Query>> {
    let file = File::open(file).await?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    // Create regex patterns
    let start_pattern = Regex::new(
        r"--\s*start\s*query\s*(?P<number>\d+)\s*in\s*stream\s*\d+\s*using\s*template\s*(?P<template>\w+\.tpl)").unwrap();
    let end_pattern = Regex::new(r"--\s*end\s*query\s*\d+").unwrap();

    let mut queries = Vec::new();
    let mut state = ParserState::SearchingStart;

    while let Some(line) = lines.next_line().await? {
        match &mut state {
            ParserState::SearchingStart => {
                if let Some(captures) = start_pattern.captures(&line) {
                    let number = captures
                        .name("number")
                        .and_then(|n| n.as_str().parse().ok())
                        .expect("malformed sql");
                    let template = captures
                        .name("template")
                        .map(|t| t.as_str().to_string())
                        .expect("malformed sql");

                    state = ParserState::CollectingQuery {
                        number,
                        template,
                        content: String::new(),
                    };
                }
            }
            ParserState::CollectingQuery {
                number,
                template,
                content,
            } => {
                if end_pattern.is_match(&line) {
                    queries.push(Query {
                        number: *number,
                        template_name: template.clone(),
                        sql: content.trim().to_string(),
                    });
                    state = ParserState::SearchingStart;
                } else {
                    content.push_str(&line);
                    content.push('\n');
                }
            }
        }
    }

    Ok(queries)
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let queries = parse_queries(Path::new(&args.queries_file)).await;
    println!("{:?}", queries.unwrap().len());
    // TODO: execute queries
    Ok(())
}
