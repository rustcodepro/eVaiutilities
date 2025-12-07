use crate::structfile::GenomeanalyzerOlder;
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

pub fn transcriptsearcholder(
    path1: &str,
    transcriptname: &str,
    analysisname: &str,
) -> Result<String, Box<dyn Error>> {
    let mut filesplit: Vec<GenomeanalyzerOlder> = Vec::new();

    for i in fs::read_dir(path1)? {
        let openfile = i?.path();
        let path_str = openfile.to_str().unwrap();
        let fileopen = File::open(path_str).expect("file not found");
        let fileread = BufReader::new(fileopen);

        for i in fileread.lines() {
            let line = i.expect("line not present");
            if line.starts_with("#") {
                continue;
            }
            if !line.starts_with("#") {
                let linevec = line.split("\t").collect::<Vec<_>>();
                filesplit.push(GenomeanalyzerOlder {
                    chrom: linevec[0].to_string(),
                    start: linevec[1].to_string(),
                    stop: linevec[2].to_string(),
                    generef: linevec[3].to_string(),
                    alt: linevec[4].to_string(),
                    priortranscript: linevec[5].to_string(),
                    hgvsp: linevec[6].to_string(),
                    hgvpc: linevec[7].to_string(),
                    cannonical: linevec[8].to_string(),
                    othertranscript: linevec[9].to_string(),
                    genotype: linevec[10].to_string(),
                    gene: linevec[11].to_string(),
                    phenotype: linevec[12].to_string(),
                    medgencui: linevec[13].to_string(),
                    inheritance: linevec[14].to_string(),
                    finalclass: linevec[15].to_string(),
                    score_pathogen: linevec[16].to_string(),
                    flag: linevec[17].to_string(),
                    note: linevec[18].to_string(),
                    pvs1: linevec[19].to_string(),
                    ps1: linevec[20].to_string(),
                    ps2: linevec[21].to_string(),
                    ps3: linevec[22].to_string(),
                    ps4: linevec[23].to_string(),
                    pm1: linevec[24].to_string(),
                    pm2: linevec[25].to_string(),
                    pm3: linevec[26].to_string(),
                    pm4: linevec[27].to_string(),
                    pm5: linevec[28].to_string(),
                    pm6: linevec[29].to_string(),
                    pp1: linevec[30].to_string(),
                    pp2: linevec[31].to_string(),
                    pp3: linevec[32].to_string(),
                    pp4: linevec[33].to_string(),
                    pp5: linevec[34].to_string(),
                    ba1: linevec[35].to_string(),
                    bs1: linevec[36].to_string(),
                    bs2: linevec[37].to_string(),
                    bs3: linevec[38].to_string(),
                    bs4: linevec[39].to_string(),
                    bp1: linevec[40].to_string(),
                    bp2: linevec[41].to_string(),
                    bp3: linevec[42].to_string(),
                    bp4: linevec[43].to_string(),
                    bp5: linevec[44].to_string(),
                    bp6: linevec[45].to_string(),
                    bp7: linevec[46].to_string(),
                    bp8: linevec[47].to_string(),
                });
            }
        }
    }

    let mut filtervariant: Vec<GenomeanalyzerOlder> = Vec::new();

    for i in filesplit.iter() {
        if i.priortranscript == transcriptname {
            filtervariant.push(GenomeanalyzerOlder {
                chrom: i.chrom.clone(),
                start: i.start.clone(),
                stop: i.stop.clone(),
                generef: i.generef.clone(),
                alt: i.alt.clone(),
                priortranscript: i.priortranscript.clone(),
                hgvsp: i.hgvsp.clone(),
                hgvpc: i.hgvpc.clone(),
                cannonical: i.cannonical.clone(),
                othertranscript: i.othertranscript.clone(),
                genotype: i.genotype.clone(),
                gene: i.gene.clone(),
                phenotype: i.phenotype.clone(),
                medgencui: i.medgencui.clone(),
                inheritance: i.inheritance.clone(),
                finalclass: i.finalclass.clone(),
                score_pathogen: i.score_pathogen.clone(),
                flag: i.flag.clone(),
                note: i.note.clone(),
                pvs1: i.pvs1.clone(),
                ps1: i.ps1.clone(),
                ps2: i.ps2.clone(),
                ps3: i.ps3.clone(),
                ps4: i.ps4.clone(),
                pm1: i.pm1.clone(),
                pm2: i.pm2.clone(),
                pm3: i.pm3.clone(),
                pm4: i.pm4.clone(),
                pm5: i.pm5.clone(),
                pm6: i.pm6.clone(),
                pp1: i.pp1.clone(),
                pp2: i.pp2.clone(),
                pp3: i.pp3.clone(),
                pp4: i.pp4.clone(),
                pp5: i.pp5.clone(),
                ba1: i.ba1.clone(),
                bs1: i.bs1.clone(),
                bs2: i.bs2.clone(),
                bs3: i.bs3.clone(),
                bs4: i.bs4.clone(),
                bp1: i.bp1.clone(),
                bp2: i.bp2.clone(),
                bp3: i.bp3.clone(),
                bp4: i.bp4.clone(),
                bp5: i.bp5.clone(),
                bp6: i.bp6.clone(),
                bp7: i.bp7.clone(),
                bp8: i.bp8.clone(),
            });
        }
    }
    let mut filewrite = File::create(analysisname).expect("file not present");
    writeln!(
            filewrite,
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}", "sample", "chrom", "start", "stop", "generef", "alt", "priortranscript", "hgvsp", "hgvpc", "cannonical", "othertranscript", "genotype", "gene", "phenotype", "medgencui", "inheritance", "finalclass", "score_pathogen", "flag", "note", "pvs1", "ps1", "ps2", "ps3", "ps4", "pm1", "pm2", "pm3", "pm4", "pm5", "pm6", "pp1", "pp2", "pp3", "pp4", "pp5", "ba1", "bs1", "bs2", "bs3", "bs4", "bp1", "bp2", "bp3", "bp4", "bp5", "bp6", "bp7", "bp8").expect("file not present");

    for i in filtervariant.iter() {
        writeln!(
            filewrite,
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            i.chrom,
            i.start,
            i.stop,
            i.generef,
            i.alt,
            i.priortranscript,
            i.hgvsp,
            i.hgvpc,
            i.cannonical,
            i.othertranscript,
            i.genotype,
            i.gene,
            i.phenotype,
            i.medgencui,
            i.inheritance,
            i.finalclass,
            i.score_pathogen,
            i.flag,
            i.note,
            i.pvs1,
            i.ps1,
            i.ps2,
            i.ps3,
            i.ps4,
            i.pm1,
            i.pm2,
            i.pm3,
            i.pm4,
            i.pm5,
            i.pm6,
            i.pp1,
            i.pp2,
            i.pp3,
            i.pp4,
            i.pp5,
            i.ba1,
            i.bs1,
            i.bs2,
            i.bs3,
            i.bs4,
            i.bp1,
            i.bp2,
            i.bp3,
            i.bp4,
            i.bp5,
            i.bp6,
            i.bp7,
            i.bp8
        ).expect("the file not present");
    }

    Ok("The result has been written".to_string())
}
