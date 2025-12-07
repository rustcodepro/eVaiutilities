use crate::structfile::Fasta;
use crate::structfile::PriorTranscript;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::process::Command;

/*
Gaurav Sablok,
codeprog@icloud.com
*/

pub fn fastagtf(pathacmg: &str, pathfasta: &str) -> Result<String, Box<dyn Error>> {
    let fastafile = File::open(pathfasta).expect("file not present");
    let fastaread = BufReader::new(fastafile);
    let mut vecfastaid: Vec<String> = Vec::new();
    let mut vecfastaseq: Vec<String> = Vec::new();
    for i in fastaread.lines() {
        let line = i.expect("line not present");
        if line.starts_with(">") {
            let lineselect: Vec<_> = line.split(" ").collect::<Vec<_>>();
            vecfastaid.push(lineselect[0].to_string().replace(">", ""));
        } else if !line.starts_with(">") {
            vecfastaseq.push(line);
        }
    }
    let mut fastacombine: Vec<Fasta> = Vec::new();
    for i in 0..vecfastaid.len() {
        fastacombine.push(Fasta {
            header: vecfastaid[i].clone(),
            sequence: vecfastaseq[i].clone(),
        });
    }

    let acmgopen = File::open(pathacmg).expect("file not present");
    let acmgread = BufReader::new(acmgopen);
    let mut priortranscript: Vec<PriorTranscript> = Vec::new();
    for i in acmgread.lines() {
        let line = i.expect("line not present");
        if line.starts_with("#") {
            continue;
        } else if !line.starts_with("#") {
            let linecut: Vec<_> = line.split("\t").map(|x| x.to_string()).collect::<Vec<_>>();
            let linegenome: Vec<_> = linecut[9]
                .split("|")
                .collect::<Vec<_>>()
                .into_iter()
                .map(|x| x.replace(":null", ""))
                .collect::<Vec<_>>();
            priortranscript.push(PriorTranscript {
                prior: linecut[5].clone(),
                alternate: linegenome,
            });
        }
    }

    let _ = Command::new("wget")
            .arg("https://ftp.ensembl.org/pub/release-113/fasta/homo_sapiens/cds/Homo_sapiens.GRCh38.cds.all.fa.gz")
            .output()
            .expect("result not found");

    let mut priorsequencewrite: Vec<Fasta> = Vec::new();
    for i in priortranscript.iter() {
        for j in fastacombine.iter() {
            if *i.prior == j.header.to_string() {
                priorsequencewrite.push(Fasta {
                    header: i.prior.clone(),
                    sequence: j.sequence.clone(),
                })
            }
        }
    }

    let mut alternatesequencewrite: Vec<Fasta> = Vec::new();
    for i in priortranscript.iter() {
        for j in fastacombine.iter() {
            for val in i.alternate.iter() {
                if *val == j.header.to_string() {
                    alternatesequencewrite.push(Fasta {
                        header: val.to_string(),
                        sequence: j.sequence.clone(),
                    })
                }
            }
        }
    }

    let mut priorsequenceout = File::create("priorsequence.fasta").expect("file not found");
    let mut alternatesequenceout = File::create("alternatesequence.fasta").expect("file not found");
    for i in priorsequencewrite.iter() {
        writeln!(priorsequenceout, ">{}\n{}", i.header, i.sequence).expect("line not present");
    }

    for i in alternatesequencewrite.iter() {
        writeln!(alternatesequenceout, ">{}\n{}", i.header, i.sequence).expect("line not present");
    }

    Ok("The sequences have been written".to_string())
}
