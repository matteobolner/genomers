# genomers
genomers is a rust package for downloading genome assembly data and metadata from NCBI



### Changelog
- v0.1.5 Added GFF genome annotation to download options


### Features

* Download the assembly report and assembly fasta sequence


## How to use ##

The binaries can be downloaded from the 'release' section of the repository (https://github.com/matteobolner/genomers/releases/download/{release})
or with
```shell
wget https://github.com/matteobolner/genomers/releases/download/first-release/genomers
```


## Why genomers? ##

I wanted to have an easy way to download genome assembly data and metadata from NCBI, and since I'm learning rust I decided to try doing it in rust.

## How to use ##

Two parameters are required:  
the assembly accession and assembly name.

running "genomers -h" will print all the necessary information.

Example:

```shell
# download the assembly report for the E. coli genome

./genomers -a GCA_000005845.2 -n ASM584v2 -r > report.txt

# now the full genome seqeuence

./genomers -a GCA_000005845.2 -n ASM584v2 -g > genome.fasta.gz

```

The output is directed to stdout and can be piped:

```shell
./genomers -a GCA_000005845.2 -n ASM584v2 -r | grep "Chromosome"

# unzip the file
./genomers -a GCA_000005845.2 -n ASM584v2 -g | gunzip > genome.fasta

```
