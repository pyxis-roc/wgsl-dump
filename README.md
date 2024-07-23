<!--
SPDX-FileCopyrightText: 2024 University of Rochester

SPDX-License-Identifier: CC-BY-SA-4.0
-->

# wgsl-dump

wgsl-dump is a small command line utility for dumping selected parts of a wgsl module.

### Installation

Install wgsl-dumper with cargo by providing 
`cargo install --git https://github.com/pyxis-roc/wgsl-dump`

### Usage

To use wgsl-dump, invoke it from the command line, providing the path to the wgsl file you want to parse:
``wgsl-dump -i /path/to/input -o /path/to/output <--dump-conditions|--dump-indices>``

For full usage information see ``wgsl-dump --help``
