extern crate bio;
extern crate clap;
extern crate petgraph;

use bio::alignment::pairwise::Scoring;
use bio::alignment::poa::Poa;
use bio::io::fasta;
use bio::io::fasta::FastaRead;
use std::fs::File;
use std::path::Path;

use clap::{App, Arg};
use petgraph::dot::Dot;
use std::io::Write;
use std::str::FromStr;

use crate::colours::Colours;

pub mod colours;

fn main() {
    let args = App::new("colour-poa")
        .arg(Arg::with_name("INPUT").required(true).index(1))
        .arg(
            Arg::with_name("branch penalty")
                .short("b")
                .value_name("BRANCH")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("mismatch penalty")
                .short("s")
                .value_name("MISMATCH")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("out")
                .short("o")
                .value_name("FILE")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("query")
                .short("q")
                .value_name("QUERY")
                .takes_value(true),
        )
        .get_matches();

    let r = File::open(args.value_of("INPUT").unwrap()).unwrap();

    let mut reader = fasta::Reader::new(r);

    let mismatch_penalty: i32 =
        FromStr::from_str(args.value_of("mismatch penalty").unwrap_or("12")).unwrap();
    let branch_penalty: i32 =
        FromStr::from_str(args.value_of("branch penalty").unwrap_or("3")).unwrap();

    let match_fn = |a: u8, b: u8| -> Colours {
        if a == b {
            Colours { c1: 1i32, c2: 1i32 }
        } else {
            Colours {
                c1: -mismatch_penalty,
                c2: -mismatch_penalty,
            }
        }
    };

    let mut rec_one = fasta::Record::new();
    reader.read(&mut rec_one).expect("could not read file");
    let scoring = Scoring::new(
        Colours {
            c1: -branch_penalty,
            c2: -branch_penalty,
        },
        Colours { c1: 1i32, c2: 1i32 },
        &match_fn,
    );

    let mut poa = Poa::from_string(scoring, &rec_one.seq(), Colours { c1: 1i32, c2: 0i32 });
    for rec in reader.records() {
        if let Ok(s) = rec {
            let aln = poa.global(s.seq()).alignment();
            let c = match s.id() {
                "1" => Colours { c1: 1i32, c2: 0i32 },
                "2" => Colours { c1: 0i32, c2: 1i32 },
                _ => Colours { c1: 0i32, c2: 0i32 },
            };
            println!("score {}", aln.score);
            poa.add_alignment(&aln, s.seq(), c);
        }
    }

    if let Some(q) = args.value_of("query") {
        let tb = poa.global(format!("${}^", q).as_bytes());
        println!("score: {}", tb.alignment().score);
        //tb.print(&poa.graph, format!("${}^", q).as_bytes());
    }
    if let Some(fp) = args.value_of("out") {
        let mut graph_out = File::create(&Path::new(fp)).unwrap();
        let g = &poa.graph.map(|_, nw| *nw as char, |_, ew| ew.label());
        graph_out
            .write_all(Dot::new(g).to_string().replace(r#"\""#, "\"").as_bytes())
            .expect("could not write file");
    }
}
