mod acmgtranscript;
mod altallele;
mod altalleleolder;
mod annotation;
mod annotationolder;
mod args;
mod coordinate;
mod coordinateolder;
mod coordinatevariant;
mod coordinatevariantolder;
mod fastanalyzer;
mod genomeanalyzer;
mod genomedownload;
mod gtfanalyzer;
mod pathogenicity;
mod pathogenicityolder;
mod populationvariant;
mod populationvariantolder;
mod refallele;
mod refalleleolder;
mod sequenceprofile;
mod structfile;
mod transcript;
mod transcriptolder;
mod variantaccumulate;
mod variantdatabase;
mod variantfilter;
use crate::acmgtranscript::acmgannotate;
use crate::altallele::altrefallelesearch;
use crate::altalleleolder::altrefalleleoldersearch;
use crate::annotation::annotationsearch;
use crate::annotationolder::annotationsearcholder;
use crate::args::CommandParse;
use crate::args::Commands;
use crate::coordinate::coordinatesearch;
use crate::coordinateolder::coordinatesearcholder;
use crate::coordinatevariant::coordinatevariantsearch;
use crate::coordinatevariantolder::coordinatevariantsearcholder;
use crate::fastanalyzer::fastagtf;
use crate::genomeanalyzer::genomemap;
use crate::genomedownload::downloadgenome;
use crate::gtfanalyzer::analyzegtf;
use crate::pathogenicity::pathogenicityscore;
use crate::pathogenicityolder::pathogenicityscoreolder;
use crate::populationvariant::population;
use crate::populationvariantolder::populationolder;
use crate::refallele::altallelesearch;
use crate::refalleleolder::altalleleoldersearch;
use crate::sequenceprofile::sequence;
use crate::transcript::transcriptsearch;
use crate::transcriptolder::transcriptsearcholder;
use crate::variantaccumulate::variantaccumulateplot;
use crate::variantdatabase::variantdatabase;
use crate::variantfilter::variantanalyzer;
use clap::Parser;
use figlet_rs::FIGfont;

/*
Author Gaurav Sablok,
Email: codeprog@icloud.com
Date: 2025-3-12
*/

fn main() {
    let standard_font = FIGfont::standard().unwrap();
    let figure = standard_font.convert("eVaiUtilities");
    assert!(figure.is_some());
    println!("{}", figure.unwrap());
    let argsparse = CommandParse::parse();
    match &argsparse.command {
        Commands::VariantAnalyzer {
            acmgfile,
            tsvfile,
            threads,
        } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(threads.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let command = genomemap(acmgfile, tsvfile).unwrap();
                println!("The file has been converted: {:?}", command);
            })
        }
        Commands::VariantFilter {
            acmgfile,
            tsvfile,
            variant,
            threads,
        } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(threads.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let command = variantanalyzer(acmgfile, tsvfile, variant).unwrap();
                println!("The filtered variant file has been written:{}", command);
            });
        }
        Commands::VariantDatabase {
            acmgfile,
            tsvfile,
            databaseurl,
        } => {
            let command = variantdatabase(acmgfile, tsvfile, databaseurl).unwrap();
            println!("The variant database has been created:{}", command);
        }
        Commands::GTFAnalyze { gtffile, threads } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(threads.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let command = analyzegtf(gtffile).unwrap();
                println!("The gtf has been analyzed:{:?}", command);
            })
        }
        Commands::VariantSeq {
            acmgfile,
            fastafile,
            threads,
        } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(threads.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let command = fastagtf(acmgfile, fastafile).unwrap();
                println!("The sequences have been written:{:?}", command);
            })
        }
        Commands::DownloadGenome { input } => {
            let command = downloadgenome(input).unwrap();
            println!("The genome has been downloaded:{:?}", command);
        }
        Commands::ACMGTranscript { acmgfile, threads } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(threads.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let command = acmgannotate(acmgfile).unwrap();
                println!("The transcript ids have been written:{:?}", command);
            });
        }
        Commands::SequenceProfile {
            acmgfile,
            fastafile,
            upstream,
            downstream,
            variant,
            threads,
        } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(threads.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let command =
                    sequence(acmgfile, fastafile, *upstream, *downstream, variant).unwrap();
                println!("The sequences have been written:{:?}", command);
            });
        }
        Commands::PopulationVariantSearch {
            acmgdir,
            variant,
            name,
            threads,
        } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(threads.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let command = population(acmgdir, variant, name.to_string()).unwrap();
                println!("The command has finished:{}", command);
            });
        }
        Commands::CoordinateSearch {
            acmgdir,
            start,
            end,
            name,
            threads,
        } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(threads.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let command = coordinatesearch(acmgdir, *start, *end, name).unwrap();
                println!("The command has been finished:{}", command);
            });
        }
        Commands::CoordinateSearchVariant {
            acmgdir,
            start,
            end,
            variant,
            name,
            threads,
        } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(threads.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let command =
                    coordinatevariantsearch(acmgdir, *start, *end, variant, name).unwrap();
                println!("The command has finished:{}", command);
            });
        }
        Commands::AnnotationSearch {
            acmgdir,
            genename,
            name,
            threads,
        } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(threads.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let command = annotationsearch(acmgdir, genename, name).unwrap();
                println!("The command has been finished:{}", command);
            });
        }
        Commands::PathogenicityFilter {
            acmgdir,
            start,
            end,
            name,
            threads,
        } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(threads.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let command = pathogenicityscore(acmgdir, *start, *end, name).unwrap();
                println!("The command has completed:{}", command);
            });
        }
        Commands::PopulationVariantSearcholder {
            acmgdir,
            variant,
            name,
            threads,
        } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(threads.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let command = populationolder(acmgdir, variant, name.to_string()).unwrap();
                println!("The command has finished:{}", command);
            });
        }
        Commands::CoordinateSearcholder {
            acmgdir,
            start,
            end,
            name,
            threads,
        } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(threads.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let command = coordinatesearcholder(acmgdir, *start, *end, name).unwrap();
                println!("The command has been finished:{}", command);
            });
        }
        Commands::CoordinateSearcVariantholder {
            acmgdir,
            start,
            end,
            variant,
            name,
            threads,
        } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(threads.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let command =
                    coordinatevariantsearcholder(acmgdir, *start, *end, variant, name).unwrap();
                println!("The command has finished:{}", command);
            });
        }
        Commands::AnnotationSearcholder {
            acmgdir,
            genename,
            name,
            threads,
        } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(threads.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let command = annotationsearcholder(acmgdir, genename, name).unwrap();
                println!("The command has been finished:{}", command);
            });
        }
        Commands::PathogenicityFilterolder {
            acmgdir,
            start,
            end,
            name,
            threads,
        } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(threads.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let command = pathogenicityscoreolder(acmgdir, *start, *end, name).unwrap();
                println!("The command has completed:{}", command);
            });
        }
        Commands::TranscriptSearch {
            acmgdir,
            transcript,
            name,
            threads,
        } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(threads.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let command = transcriptsearch(acmgdir, transcript, name).unwrap();
                println!("The command has been completed:{}", command);
            });
        }
        Commands::TranscriptSearcholder {
            acmgdir,
            transcript,
            name,
            threads,
        } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(threads.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let command = transcriptsearcholder(acmgdir, transcript, name).unwrap();
                println!(
                    "The command has finished and the files have been written:{}",
                    command
                );
            });
        }
        Commands::AltAllele {
            acmgdir,
            refallele,
            name,
            threads,
        } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(threads.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let command = altallelesearch(acmgdir, refallele, name.to_string()).unwrap();
                println!("The command has finished:{}", command);
            });
        }
        Commands::AltRefAllele {
            acmgdir,
            refallele,
            altallele,
            name,
            threads,
        } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(threads.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let command =
                    altrefallelesearch(acmgdir, refallele, altallele, name.to_string()).unwrap();
                println!("The command has finished:{}", command);
            });
        }
        Commands::AltAlleleOlder {
            acmgdir,
            refallele,
            name,
            threads,
        } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(threads.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let command = altalleleoldersearch(acmgdir, refallele, name.to_string()).unwrap();
                println!("The command has finished:{}", command);
            });
        }
        Commands::AltRefAlleleOlder {
            acmgdir,
            refallele,
            altallele,
            name,
            threads,
        } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(threads.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let command =
                    altrefalleleoldersearch(acmgdir, refallele, altallele, name.to_string())
                        .unwrap();
                println!("The command has finished:{}", command);
            })
        }
        Commands::VariantPlotter {
            pathfolder,
            threads,
        } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(threads.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let command = variantaccumulateplot(pathfolder).unwrap();
                println!("The command has finished: {}", command);
            });
        }
    }
}
