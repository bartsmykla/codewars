#![allow(unused)]
/*
    Kata (8kyu): DNA to RNA Conversion
    Url: https://www.codewars.com/kata/dna-to-rna-conversion/train/rust

    Deoxyribonucleic acid, DNA is the primary information storage molecule
    in biological systems. It is composed of four nucleic acid bases
    Guanine ('G'), Cytosine ('C'), Adenine ('A'), and Thymine ('T').

    Ribonucleic acid, RNA, is the primary messenger molecule in cells.
    RNA differs slightly from DNA its chemical structure and contains
    no Thymine. In RNA Thymine is replaced by another nucleic acid Uracil ('U').

    Create a funciton which translates a given DNA string into RNA.

    For example:

    dna_to_rna("GCAT") //=> "GCAU"
    The input string can be of arbitrary length - in particular,
    it may be empty. All input is guaranteed to be valid,
    i.e. each input string will only ever consist of 'G', 'C', 'A' and/or 'T'.
*/

fn dna_to_rna(dna: &str) -> String {
    dna.chars()
        .map(|c| match c {
            'T' => 'U',
            _ => c,
        })
        .collect()
}

#[test]
fn returns_expected() {
    assert_eq!(dna_to_rna("TTTT"), "UUUU");
    assert_eq!(dna_to_rna("GCAT"), "GCAU");
}
