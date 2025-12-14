# eVaiutilities:

- Analyzing and identifying genomic variants is particularly helpful in case of linking the disease onset to the genomic predictions. Genomic variants play a key role in the identification and characterization of the disease onset and also allow for interlinking genomic variability and hypothesis testing. eVai, which is a variant annotation platform provides annotation of the genomic variants using several backhand open source databases and tools. eVaiutilities proposed in this paper, is a data management software for the analysis of the eVai output files. It allows the analysis of the genomic variants further such as analyzing the multiple genomic annotated variants, reference and alternate allele, enabling coordinate search, coordinate search with specified variants and annotation search across a large number of population. The availability of the command line parameters allows for a large scale analysis across the several population files.

# Role of eVaiutilities:
- eVaiutilities is used for the population scale analysis the eVai results. It supports all versions of eVai and uses RUST version rustc 1.89.0 (29483883e 2025-08-04).
- It analyzes the eVai output files for variant annotation to filtering, preparating user reports and database and annotator and more. See the example folder for the older and the newer version files.
- It provides the population scale variant searching given a folder according to the variant, annotation, transcript, gene, and coordinates.
- It uses this version of the human genome from [GENCODE](https://www.gencodegenes.org/human/)
- It summaries several stats and plots figures for the variant analysis.

[![DOI](https://zenodo.org/badge/DOI/10.5281/zenodo.17083829.svg)](https://doi.org/10.5281/zenodo.17083829)

- [Scientific Rust](https://www.youtube.com/watch?app=desktop&v=dru-2Cn-RTQ)

- In this new version, it plots the variants accumulated also.

<img src="https://github.com/genomicssport/eVaiutilities/blob/main/testfiles/barplot.svg" >

```
 cargo build
```

```
___  \ \   / /   __ _  (_) | | | | | |_  (_) | | (_) | |_  (_)   ___   ___
/ _ \  \ \ / /   / _` | | | | | | | | __| | | | | | | | __| | |  / _ \ / __|
|  __/   \ V /   | (_| | | | | |_| | | |_  | | | | | | | |_  | | |  __/ \__ \
\___|    \_/     \__,_| |_|  \___/   \__| |_| |_| |_|  \__| |_|  \___| |___/


annotating and analyzing eVai results.

   ************************************************
   Gaurav Sablok,
   Email: codeprog@icloud.com
   ************************************************

Usage: eVaiutilities <COMMAND>

Commands:
variant-analyzer                analyzer for the variants
variant-filter                  filter the variants
variant-database                create variant database
gtf-analyze                     analyze the corresponding gtf
variant-seq                     prepare the variant seq annotation
download-genome                 download the human genome
acmg-transcript                 variant-transcriptids
sequence-profile                sequence profiling
population-variant-search       search for the variant across population
coordinate-search               search according to coordinates
coordinate-search-variant       search according to coordinates and variant
annotation-search               search according to annotation
pathogenicity-filter            pathogenicity filter
population-variant-searcholder  search for the variant across population older version
coordinate-searcholder          search according to coordinates older version
coordinate-searc-variantholder  search according to coordinates and variant older version
annotation-searcholder          search according to annotation older version
pathogenicity-filterolder       pathogenicity filter older version
transcript-search               search by the specific transcript
transcript-searcholder          search by the specific transcript older
alt-allele                      search for the ref allele
alt-ref-allele                  search according to ref allele and alt allele
alt-allele-older                search for the ref allele older version
alt-ref-allele-older            search according to ref allele and alt allele older version
variant-plotter                 accumulate all variants for the plots
help                            Print this message or the help of the given subcommand(s)

Options:
-h, --help     Print help
-V, --version  Print version

```
- All files are present in the testfiles.

```
 variant-analyzer sample1.tsv sampletest.tsv 4
 variant-filter sample1.tsv sampletest.tsv TTT 4
 gtf-analyze samplegtf.gtf 4
 variant-seq sample1.tsv sample.fasta 4
 download-genome yes 
 acmg-transcript sample1.tsv 4
 sequence-profile sample1.tsv sample.fasta 10 10 4
 population-variant-search newversion-population G samplename 4
 coordinate-search newversion-population 14464 16495 samplename 4
 annotation-search newversion-population AL645608.6 samplename 4
 pathogenicity-filter newversion-population 0.0 1.5 samplename 4
 population-variant-searcholder oldversion-population G samplename 4
 coordinate-searcholder oldversion-population 14464 16495 samplename 4
 annotation-searcholder oldversion-population WASH7P samplename 4
 pathogenicity-filterolder oldversion-population 0 1.5 samplename 4
 transcript-search newversion-population ENST00000635509.2 samplename 4
 alt-allele newversion-population T samplename 4
 alt-allele-older oldversion-population T samplename 4
 alt-ref-allele newversion-population A T samplename 4
 alt-ref-allele-older oldversion-population A T samplename 4
 variant-plotter variant-accumulation 4
```

- Below you will find a detailed usage of the eVaiutilities.

- Variant Analyzer: The variant analyzer allows you to merge all the information present in the ACMG file and the TSV file for the annotate variants into a single file and removes the redundant information.This allows for a single file analysis.
- Variant Filter: This option allows for the filtering of the variants according to a defined variant and allows for the filtering of the variant from a single annotated variant. It takes a ACMG and TSV file and a variant which needs to be filtered.
- GTF-analyze: This option allows for the analysis of the corresponding GTF and gives you the corresponding annotated exons of the annotated genes.It collates all the exons present in the gene.
- Variant-seq: This gives you the annotated sequence associated with the variants given the fasta sequence.
prepare the variant seq annotation.
- Download-genome:This option allows you to download the latest version of the genome from the GENCODE.
- ACMG-transcript: This option gives you the associated transcript ids with the variants in the ACMG transcripts.
- Sequence-profile: This allows you to search for the variant and then extract the upstream and downstream information associated with the particular variant.
- Population-Variant-Search: This allows you to search for a particular variant across a population. This takes a directory containing all the ACMG files correponding to a particular population.
- Coordinate-search: This option allows you to search for a variant between the coordinates. In the above example, we provide a directory containing the ACMG files and then search for the variant between these coordinates across all those files and output a filename with the same samplename.
- Annotation-Search: This allows you to search for a variant with the specificed directory containing all the ACMG files and then associated with the specific transcript id across all those files.
- Pathogenicity-Filter:This allows you to filter a directory with the ACMG files and you to have provide the range of the pathogenicity filter with the start and the end value and the sample name as the file to be written.
- Population-Variant-SearchHolder:This allows you to search for a particular variant across a population. This takes a directory containing all the ACMG files correponding to a particular population. This option is for the older version of the eVai.
- Coordinate-Search-Holder: This option allows you to search for a variant between the coordinates. In the above example, we provide a directory containing the ACMG files and then search for the variant between these coordinates across all those files and output a filename with the same samplename. This supports the older version of the eVai.
- Annotation-Search-Holder:This allows you to search for a variant with the specificed directory containing all the ACMG files and then associated with the specific transcript id across all those files. This supports the older version of the eVai.
- Pathogenicity-Filter-Older:This allows you to filter a directory with the ACMG files and you to have provide the range of the pathogenicity filter with the start and the end value and the sample name as the file to be written. This supports the older version.
- Transcript-Search:This option allows you to search for a specific transcript across the entire population and takes a ACMG folder and the transcript name and a ouptufile name.
- Alt-Allel:This option allows for the filtering of the entire population study and takes a directory containing ACMG files and a ref allele to look for and a output file name.
- Alt-Allele-Older: This option allows for the filtering of the entire population study and takes a directory containing ACMG files and a ref allele to look for and a output file name. This supports the older version of the eVai.
- Alt-Ref-Allel:This option allows you to search for a ref allele that has a specific alternate allele across a entire population and takes a directory containing ACMG files and the ref and the alternate allele to search for and an output filename to which the information should be written.
- Alt-ref-Allele-Older: This option allows you to search for a ref allele that has a specific alternate allele across a entire population and takes a directory containing ACMG files and the ref and the alternate allele to search for and an output filename to which the information should be written. This supports the older version.
- Variant-Plotter:This option allows you to plot all the variants across the entire population. It takes a folder containing the folder containing the annotated variants and then accumulate all the variants across that study and plots a bar plot to show the frequency of the variants across that study.

- To install windows version:

```
rustup component add llvm-tools
rustup target add x86_64-pc-windows-msvc
git clone https://github.com/IBCHgenomic/eVaiutilities.git
cd ensemblcov
cargo xwin build --target x86_64-pc-windows-msvc
```

 Gaurav Sablok \
 codeprog@icloud.com
