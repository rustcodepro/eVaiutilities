---
title: "eVaiutilities: Genomics population scale utilities for eVai"
tags:
  - RUST
  - human population genomics
  - variant analysis
  - varaint annotation
  - variant search
authors:
  - name: Gaurav Sablok
    orcid: 0000-0002-4157-9405
    affiliation: 1
affiliations:
  - name: Software Developer, Poznan, Poland
    index: 1
date: 10 April 2025
bibliography: paper.bib
---

# Summary

[eVai](https://www.engenome.com/product/) is a variant annotation platform that leverages multiple open-source databases and tools to annotate genomic variants. It has been widely adopted in both industry and [academic research](https://www.engenome.com/resources/?category=our-publications). However, because eVai is primarily browser-based and lacks native command-line support for high-throughput, large-scale population analyses, its use in population-scale studies can be limited. To address this limitation, we have developed eVaiutilities, a command-line tool implemented in Rust (version 1.89.0) that processes annotations produced by eVai as well as those from other platforms following a compatible format. It enables high-throughput analysis of multiple population files and supports deeper exploration of genomic variation at the population level. This extension allows scalable and reproducible population-level analyses and has the potential to accelerate research on disease-associated variants and support the development of more effective disease-combatting strategies.

# Statement of need

Human genomics remains a compelling area of research, with extensive efforts dedicated to identifying and characterizing genetic variants. In addition to open-source initiatives in variant annotation [@McLaren2016-jo; @Pedersen2023-co], several proprietary platforms have been developed to classify and link variants to genetic and metabolic disorders, spanning a wide spectrum of clinical relevance. One such platform is eVai, a high-throughput variant annotator that provides hierarchical classification of genomic variants through comparative analyses across multiple tools and databases, generating TSV files with mapped annotations. 

While these standard analyses facilitate variant interpretation, large-scale variant searches additionally require high speed and scalability. To meet these needs, we developed eVaiutilities in Rust, a high-performance and memory-safe programming language. eVaiutilities takes eVai output and enables genome-scale analyses of annotated variants, supporting population-wide and variant-level searches across multiple samples and datasets. Key functions include population-wide analysis of specific variants, coordinate-based searches across populations, variant searches within specified genomic coordinates, sequence-specific analyses, and reference and alternate allele queries. The tool also provides summarization and reporting of annotations, transcript sequence spanning around variants, and categorical variant filtering. Together, these features provide a scalable and versatile platform for managing and interrogating eVai outputs.

![Interface of eVaiutilities](eVaiutilities.png)

# Availability

Comprehensive annotation of human genomic variants is essential for biological interpretation and for linking variants to disease phenotypes. Numerous tools have been developed that annotate variants using pre-established databases [@Wang2010-mh; @Obenchain2014-jj]. Alongside significant advances in open-source genomics [@McLaren2016-jo; @Pedersen2023-co], parallel efforts have emerged within industry to support variant identification and annotation. eVaiutilities extends these capabilities by providing a scalable solution for population-level analyses. The tool is freely available on [GitHub](https://github.com/genomicssport/eVaiutilities) and as a Rust [crate](https://crates.io/crates/eVaiutilities).

# Acknowledgements

I thank Julia Romanowska and the reviewers for their work on revising the text, which has significantly improved the paper’s readability.

# References
