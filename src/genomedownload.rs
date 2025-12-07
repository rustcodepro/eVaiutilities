use cmd_lib::*;
use std::env;
use std::error::Error;
use std::fs;
use std::path::Path;
use std::process::Command;

/*
Gaurav Sablok,
codeprog@icloud.com
*/

pub fn downloadgenome(input: &str) -> Result<String, Box<dyn Error>> {
    if input == "yes" {
        let _ = fs::create_dir("./download").unwrap();
        let newpath = Path::new("./download");
        let _ = env::set_current_dir(newpath);

        let _ = Command::new("wget")
            .arg("-F")
            .arg("https://www.ncbi.nlm.nih.gov/grc/human")
            .output()
            .expect("command to fail");
        let _ = Command::new("wget")
            .arg("https://ftp.ncbi.nlm.nih.gov/genomes/all/GCA/000/001/405/GCA_000001405.15_GRCh38/GCA_000001405.15_GRCh38_genomic.fna.gz")
            .output()
            .expect("command to fail");
        let _ = Command::new("wget")
            .arg("https://ftp.ncbi.nlm.nih.gov/genomes/all/GCA/000/001/405/GCA_000001405.15_GRCh38/ GCA_000001405.15_GRCh38_genomic.gff.gz")
            .output()
            .expect("command to fail");
        let _ = Command::new("wget")
            .arg("https://ftp.ncbi.nlm.nih.gov/genomes/all/GCA/000/001/405/GCA_000001405.15_GRCh38/GCA_000001405.15_GRCh38_genomic.gaps.gz")
            .output()
            .expect("command to fail");
        let _ = Command::new("wget")
            .arg("https://ftp.ncbi.nlm.nih.gov/genomes/all/GCA/000/001/405/GCA_000001405.15_GRCh38/GCA_000001405.15_GRCh38_genomic.gbff.gz")
            .output()
            .expect("command to fail");
        let _ = Command::new("wget")
            .arg("https://ftp.ncbi.nlm.nih.gov/genomes/all/GCA/000/001/405/GCA_000001405.15_GRCh38/GCA_000001405.15_GRCh38_genomic.gff.gz")
            .output()
            .expect("command to fail");
        let _ = Command::new("wget")
            .arg("https://ftp.ncbi.nlm.nih.gov/genomes/all/GCA/000/001/405/GCA_000001405.15_GRCh38/GCA_000001405.15_GRCh38_protein.gpff.gz")
            .output()
            .expect("command to fail");
        let _ = Command::new("wget")
            .arg("https://ftp.ncbi.nlm.nih.gov/genomes/all/GCA/000/001/405/GCA_000001405.29_GRCh38.p14/GCA_000001405.29_GRCh38.p14_feature_count.txt.gz").output()
            .expect("command to fail");
        let _ = Command::new("wget")
            .arg("https://ftp.ncbi.nlm.nih.gov/genomes/all/GCA/000/001/405/GCA_000001405.29_GRCh38.p14/GCA_000001405.29_GRCh38.p14_feature_table.txt.gz").output()
            .expect("command to fail");
        let _ = Command::new("wget")
            .arg("https://ftp.ncbi.nlm.nih.gov/genomes/all/GCA/000/001/405/GCA_000001405.29_GRCh38.p14/GCA_000001405.29_GRCh38.p14_genomic.fna.gz",).output()
            .expect("command to fail");
        let _ = Command::new("wget")
            .arg("https://ftp.ncbi.nlm.nih.gov/genomes/all/GCA/000/001/405/GCA_000001405.29_GRCh38.p14/GCA_000001405.29_GRCh38.p14_genomic.gbff.gz").output()
            .expect("command to fail");
        let _ = Command::new("wget")
            .arg("https://ftp.ncbi.nlm.nih.gov/genomes/all/GCA/000/001/405/GCA_000001405.29_GRCh38.p14/GCA_000001405.29_GRCh38.p14_genomic.gff.gz").output()
            .expect("command to fail");
        let _ = Command::new("wget")
            .arg("https://ftp.ncbi.nlm.nih.gov/genomes/all/GCA/000/001/405/GCA_000001405.29_GRCh38.p14/GCA_000001405.29_GRCh38.p14_genomic.gtf.gz").output()
            .expect("command to fail");
        let _ = Command::new("wget")
            .arg("https://ftp.ncbi.nlm.nih.gov/genomes/all/GCA/000/001/405/GCA_000001405.29_GRCh38.p14/GCA_000001405.29_GRCh38.p14_genomic_gaps.txt.gz").output()
            .expect("command to fail");
        let _ = Command::new("wget")
            .arg("https://ftp.ncbi.nlm.nih.gov/genomes/all/GCA/000/001/405/GCA_000001405.29_GRCh38.p14/GCA_000001405.29_GRCh38.p14_protein.faa.gz",).output()
            .expect("command to fail");
        let _ = Command::new("wget")
            .arg("https://ftp.ncbi.nlm.nih.gov/genomes/all/GCA/000/001/405/GCA_000001405.29_GRCh38.p14/GCA_000001405.29_GRCh38.p14_protein.gpff.gz").output()
            .expect("command to fail");
        let _ = Command::new("wget")
            .arg("https://ftp.ncbi.nlm.nih.gov/genomes/all/GCA/000/001/405/GCA_000001405.29_GRCh38.p14/GCA_000001405.29_GRCh38.p14_rna_from_genomic.fna.gz").output()
            .expect("command to fail");
        let _ = Command::new("wget")
            .arg("https://ftp.ncbi.nlm.nih.gov/genomes/all/GCA/000/001/405/GCA_000001405.29_GRCh38.p14/GCA_000001405.29_GRCh38.p14_translated_cds.faa.gz").output()
            .expect("command to fail");
        let _ = Command::new("wget")
            .arg("https://ftp.ncbi.nlm.nih.gov/genomes/all/GCA/000/001/405/GCA_000001405.29_GRCh38.p14/GCA_000001405.29_GRCh38.p14_cds_from_genomic.fna.gz").output()
            .expect("command to fail");
        let _ = run_cmd!(md5sum GCA_000001405.29_GRCh38.p14_feature_count.txt.gz >> download.txt)
            .expect("run command failed");
        let _ =
            run_cmd!(md5sum GCA_000001405.29_GRCh38.p14_cds_from_genomic.fna.gz >> download.txt)
                .expect("run command failed");
        let _ = run_cmd!(md5sum GCA_000001405.29_GRCh38.p14_feature_table.txt.gz >> download.txt)
            .expect("run command failed");
        let _ = run_cmd!(md5sum GCA_000001405.29_GRCh38.p14_genomic.fna.gz >> download.txt)
            .expect("run command failed");
        let _ = run_cmd!(md5sum GCA_000001405.29_GRCh38.p14_genomic_gaps.txt.gz >> download.txt)
            .expect("run command failed");
        let _ = run_cmd!(md5sum GCA_000001405.29_GRCh38.p14_genomic.gbff.gz >> download.txt)
            .expect("run command failed");
        let _ = run_cmd!(md5sum GCA_000001405.29_GRCh38.p14_genomic.gff.gz >> download.txt)
            .expect("run command failed");
        let _ = run_cmd!(md5sum GCA_000001405.29_GRCh38.p14_genomic.gtf.gz >> download.txt)
            .expect("run command failed");
        let _ = run_cmd!(md5sum GCA_000001405.29_GRCh38.p14_protein.faa.gz >> download.txt)
            .expect("run command failed");
        let _ = run_cmd!(md5sum GCA_000001405.29_GRCh38.p14_protein.gpff.gz >> download.txt)
            .expect("run command failed");
        let _ =
            run_cmd!(md5sum GCA_000001405.29_GRCh38.p14_rna_from_genomic.fna.gz >> download.txt)
                .expect("run command failed");
        let _ = run_cmd!(md5sum GCA_000001405.29_GRCh38.p14_translated_cds.faa.gz >> download.txt)
            .expect("run command failed");
        let _ = fs::create_dir("../gencode-download").unwrap();
        let newpath = Path::new("../gencode-download");
        let _ = env::set_current_dir(newpath);
        let _ = Command::new("wget")
            .arg("https://ftp.ebi.ac.uk/pub/databases/gencode/Gencode_human/release_48/gencode.v48.primary_assembly.annotation.gtf.gz").output().expect("command failed");
        let _ = Command::new("wget").arg("https://ftp.ebi.ac.uk/pub/databases/gencode/Gencode_human/release_48/gencode.v48.primary_assembly.annotation.gff3.gz").output().expect("command failed");
        let _ = Command::new("wget").arg("https://ftp.ebi.ac.uk/pub/databases/gencode/Gencode_human/release_48/GRCh38.primary_assembly.genome.fa.gz").output().expect("command failed");
        let _ = Command::new("wget").arg("https://ftp.ebi.ac.uk/pub/databases/gencode/Gencode_human/release_48/gencode.v48.annotation.gtf.gz").output().expect("command failed");
        let _ = Command::new("wget").arg("https://ftp.ebi.ac.uk/pub/databases/gencode/Gencode_human/release_48/gencode.v48.annotation.gff3.gz").output().expect("command failed");
        let _ = Command::new("wget").arg("https://ftp.ebi.ac.uk/pub/databases/gencode/Gencode_human/release_48/gencode.v48.chr_patch_hapl_scaff.annotation.gtf.gz").output().expect("command failed");
        let _ = Command::new("wget").arg("https://ftp.ebi.ac.uk/pub/databases/gencode/Gencode_human/release_48/gencode.v48.chr_patch_hapl_scaff.annotation.gff3.gz").output().expect("command failed");
        let _ = Command::new("wget").arg("https://ftp.ebi.ac.uk/pub/databases/gencode/Gencode_human/release_48/GRCh38.primary_assembly.genome.fa.gz").output().expect("command failed");
        let _ = Command::new("wget").arg("https://ftp.ebi.ac.uk/pub/databases/gencode/Gencode_human/release_48/gencode.v48.transcripts.fa.gz").output().expect("command failed");
        let _ = Command::new("wget").arg("https://ftp.ebi.ac.uk/pub/databases/gencode/Gencode_human/release_48/gencode.v48.pc_transcripts.fa.gz").output().expect("command failed");
        let _ = Command::new("wget").arg("https://ftp.ebi.ac.uk/pub/databases/gencode/Gencode_human/release_48/GRCh38.p14.genome.fa.gz").output().expect("command failed");
        let _ = run_cmd!(md5sum gencode.v48.primary_assembly.annotation.gtf.gz >> gencode.txt)
            .expect("run command failed");
        let _ = run_cmd!(md5sum gencode.v48.primary_assembly.annotation.gff3.gz >> gencode.txt)
            .expect("run command failed");
        let _ = run_cmd!(md5sum GRCh38.primary_assembly.genome.fa.gz >> gencode.txt)
            .expect("run command failed");
        let _ = run_cmd!(md5sum gencode.v48.annotation.gtf.gz >> gencode.txt)
            .expect("run command failed");
        let _ = run_cmd!(md5sum gencode.v48.annotation.gff3.gz >> gencode.txt)
            .expect("run command failed");
        let _ = run_cmd!(md5sum gencode.v48.chr_patch_hapl_scaff.annotation.gtf.gz >> gencode.txt)
            .expect("run command failed");
        let _ = run_cmd!(md5sum gencode.v48.chr_patch_hapl_scaff.annotation.gff3.gz >> gencode.txt)
            .expect("run command failed");
        let _ = run_cmd!(md5sum GRCh38.primary_assembly.genome.fa.gz >> gencode.txt)
            .expect("run command failed");
        let _ = run_cmd!(md5sum gencode.v48.transcripts.fa.gz >> gencode.txt)
            .expect("run command failed");
        let _ = run_cmd!(md5sum gencode.v48.pc_transcripts.fa.gz >> gencode.txt)
            .expect("run command failed");
        let _ =
            run_cmd!(md5sum GRCh38.p14.genome.fa.gz >> gencode.txt).expect("run command failed");
    }

    Ok("The downloaded version of the human genome has been written to the disc".to_string())
}
