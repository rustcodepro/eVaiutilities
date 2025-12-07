use clap::{Parser, Subcommand};
#[derive(Debug, Parser)]
#[command(
    name = "variantannotator",
    version = "1.0",
    about = "annotating and analyzing eVai results.

      ************************************************
      Gaurav Sablok,
      Email: codeprog@icloud.com
      ************************************************"
)]
pub struct CommandParse {
    /// subcommands for the specific actions
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// analyzer for the variants
    VariantAnalyzer {
        /// provide the path to the acmg file
        acmgfile: String,
        /// provide the path to the tsv file
        tsvfile: String,
        /// threads for the analysis
        threads: String,
    },
    /// filter the variants
    VariantFilter {
        /// provides the path to the acmg file
        acmgfile: String,
        /// provides the path to the tsv file
        tsvfile: String,
        /// provide the variant
        variant: String,
        /// threads for the analysis
        threads: String,
    },
    /// create variant database
    VariantDatabase {
        /// please provide the path to the acmg file
        acmgfile: String,
        /// please provide the path to the tsv file
        tsvfile: String,
        // please provide the database url
        databaseurl: String,
    },
    /// analyze the corresponding gtf
    GTFAnalyze {
        /// path to the gtf file
        gtffile: String,
        /// threads for the analysis
        threads: String,
    },
    /// prepare the variant seq annotation.
    VariantSeq {
        /// provide the ACMG file
        acmgfile: String,
        /// provide fasta file
        fastafile: String,
        /// threads for the analysis
        threads: String,
    },
    /// download the human genome
    DownloadGenome { input: String },
    /// variant-transcriptids
    ACMGTranscript {
        /// provide the ACMG file
        acmgfile: String,
        /// threads for the analysis
        threads: String,
    },
    /// sequence profiling
    SequenceProfile {
        /// provide the acmg file
        acmgfile: String,
        /// provide the fasta file
        fastafile: String,
        /// upstream location
        upstream: usize,
        /// downstream location
        downstream: usize,
        /// variant
        variant: String,
        /// threads for the analysis
        threads: String,
    },
    /// search for the variant across population
    PopulationVariantSearch {
        /// provide the acmg directory
        acmgdir: String,
        /// provide the variant
        variant: String,
        /// analysis name
        name: String,
        /// threads for the analysis
        threads: String,
    },
    /// search according to coordinates
    CoordinateSearch {
        /// provide the acmg directory
        acmgdir: String,
        /// provide the coordinate
        start: usize,
        /// provide the end coordinate
        end: usize,
        /// analysis name
        name: String,
        /// threads for the analysis
        threads: String,
    },
    /// search according to coordinates and variant
    CoordinateSearchVariant {
        /// provide the acmg directory
        acmgdir: String,
        /// provide the coordinate
        start: usize,
        /// provide the end coordinate
        end: usize,
        /// variant type
        variant: String,
        /// analysis name
        name: String,
        /// threads for the analysis
        threads: String,
    },
    /// search according to annotation
    AnnotationSearch {
        /// provide the acmg directory
        acmgdir: String,
        /// search the annotation
        genename: String,
        /// analysis name
        name: String,
        /// threads for the analysis
        threads: String,
    },
    /// pathogenicity filter
    PathogenicityFilter {
        /// provide the acmg dir
        acmgdir: String,
        /// start value
        start: f32,
        /// end value
        end: f32,
        /// analysis name
        name: String,
        /// threads for the analysis
        threads: String,
    },
    /// search for the variant across population older version
    PopulationVariantSearcholder {
        /// provide the acmg directory
        acmgdir: String,
        /// provide the variant
        variant: String,
        /// analysis name
        name: String,
        /// threads for the analysis
        threads: String,
    },
    /// search according to coordinates older version
    CoordinateSearcholder {
        /// provide the acmg directory
        acmgdir: String,
        /// provide the coordinate
        start: usize,
        /// provide the end coordinate
        end: usize,
        /// analysis name
        name: String,
        /// threads for the analysis
        threads: String,
    },
    /// search according to coordinates and variant older version
    CoordinateSearcVariantholder {
        /// provide the acmg directory
        acmgdir: String,
        /// provide the coordinate
        start: usize,
        /// provide the end coordinate
        end: usize,
        /// variant type
        variant: String,
        /// analysis name
        name: String,
        /// threads for the analysis
        threads: String,
    },
    /// search according to annotation older version
    AnnotationSearcholder {
        /// provide the acmg directory
        acmgdir: String,
        /// search the annotation
        genename: String,
        /// analysis name
        name: String,
        /// threads for the analysis
        threads: String,
    },
    /// pathogenicity filter older version
    PathogenicityFilterolder {
        /// provide the acmg dir
        acmgdir: String,
        /// start
        start: f32,
        /// end
        end: f32,
        /// analysis name
        name: String,
        /// threads for the analysis
        threads: String,
    },
    /// search by the specific transcript
    TranscriptSearch {
        /// provide the acmg directory
        acmgdir: String,
        /// provide the transcript name
        transcript: String,
        /// name of the analysis
        name: String,
        /// threads for the analysis
        threads: String,
    },
    /// search by the specific transcript older
    TranscriptSearcholder {
        /// provide the acmg directory
        acmgdir: String,
        /// provide the transcript name
        transcript: String,
        /// name of the analysis
        name: String,
        /// threads for the analysis
        threads: String,
    },
    /// search for the ref allele
    AltAllele {
        /// provide the acmg directory
        acmgdir: String,
        /// provide the alt allele
        refallele: String,
        /// provide the analysis name
        name: String,
        /// threads for the analysis
        threads: String,
    },
    /// search according to ref allele and alt allele
    AltRefAllele {
        /// provide the acmd directory
        acmgdir: String,
        /// provide the ref allele
        refallele: String,
        /// provide the alt allele
        altallele: String,
        /// provide the analysis name
        name: String,
        /// threads for the analysis
        threads: String,
    },
    /// search for the ref allele older version
    AltAlleleOlder {
        /// provide the acmg directory
        acmgdir: String,
        /// provide the alt allele
        refallele: String,
        /// provide the analysis name
        name: String,
        /// threads for the analysis
        threads: String,
    },
    /// search according to ref allele and alt allele older version
    AltRefAlleleOlder {
        /// provide the acmd directory
        acmgdir: String,
        /// provide the ref allele
        refallele: String,
        /// provide the alt allele
        altallele: String,
        /// provide the analysis name
        name: String,
        /// threads for the analysis
        threads: String,
    },
    /// accumulate all variants for the plots
    VariantPlotter {
        /// provide the path to the folder
        pathfolder: String,
        /// threads for the analysis
        threads: String,
    },
}
