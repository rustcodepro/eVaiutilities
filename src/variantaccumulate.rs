use crate::structfile::VariantAccumulate;
use plotpy::{Barplot, Plot};
use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;

/*
Gaurav Sablok,
codeprog@icloud.com
*/

pub fn variantaccumulateplot(path1: &str) -> Result<String, Box<dyn Error>> {
    let mut filesplit: Vec<VariantAccumulate> = Vec::new();
    let mut fileversion: HashSet<String> = HashSet::new();
    for i in fs::read_dir(path1)? {
        let openfile = i?.path();
        let path_str = openfile.to_str().unwrap();
        let fileopen = File::open(path_str).expect("file not found");
        let _filerename = path_str.split("/").collect::<Vec<_>>()[1]
            .split(".")
            .collect::<Vec<_>>()[0]
            .to_string();
        let fileread = BufReader::new(fileopen);
        for i in fileread.lines() {
            let line = i.expect("line not present");
            if line.starts_with("#") && line.contains("eVAI-version") {
                let version = line.replace("#", "");
                fileversion.insert(version);
            }
            if !line.starts_with("#") {
                let linevec = line.split("\t").collect::<Vec<_>>();
                filesplit.push(VariantAccumulate {
                    refallele: linevec[3].to_string().clone(),
                    altallele: linevec[4].to_string().clone(),
                    effect: linevec[5].to_string().clone(),
                    gene: linevec[6].to_string().clone(),
                });
            }
        }
    }
    let mut effect: HashSet<_> = HashSet::new();
    for i in filesplit.iter() {
        effect.insert(i.effect.clone());
    }

    let mut countmatrix: Vec<(String, usize)> = Vec::new();
    for i in effect.iter() {
        let mut count = 0usize;
        for j in filesplit.iter() {
            if j.effect == *i {
                count += 1;
            }
        }
        let countinsert: (String, usize) = (i.clone(), count);
        countmatrix.push(countinsert);
    }

    let mut filecount = File::create("variantplot.txt").expect("file not present");
    for i in countmatrix.iter() {
        if i.0 == "EFFECT" {
            continue;
        } else {
            writeln!(filecount, "{}\t{}", i.0, i.1).expect("file not found");
        }
    }

    let mut namevector: Vec<_> = Vec::new();
    let mut namedata: Vec<usize> = Vec::new();

    for i in countmatrix.iter() {
        if i.0 == "EFFECT" {
            continue;
        } else {
            namevector.push(i.0.as_str());
            namedata.push(i.1)
        }
    }

    let mut bar = Barplot::new();
    bar.set_horizontal(true)
        .set_with_text("edge")
        .draw_with_str(namevector.as_slice(), &namedata);

    let mut plot = Plot::new();
    plot.set_inv_y()
        .add(&bar)
        .set_title("Annotate Variant Frequency")
        .set_label_x("Variant Count");

    plot.save("barplot.svg")?;

    Ok("The file has been written to string".to_string())
}
