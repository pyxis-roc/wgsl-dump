<!--
SPDX-FileCopyrightText: 2024 University of Rochester

SPDX-License-Identifier: CC-BY-SA-4.0
-->


# wgsl-dump
[![Build Status]][actions] [![REUSE status]][reuse] ![MIT-License] 

[Build Status]: https://github.com/pyxis-roc/wgsl-dump/actions/workflows/rust.yml/badge.svg
[actions]: https://github.com/pyxis-roc/wgsl-dump/actions/test
[REUSE status]: https://api.reuse.software/badge/github.com/pyxis-roc/wgsl-dump
[reuse]: https://api.reuse.software/info/github.com/pyxis-roc/wgsl-dump
[MIT-License]: https://img.shields.io/badge/License-MIT-blue.svg

wgsl-dump is a small command line utility for dumping selected parts of a wgsl module.

### Installation

Install wgsl-dumper with cargo by providing 
`cargo install --git https://github.com/pyxis-roc/wgsl-dump`

### Usage

To use wgsl-dump, invoke it from the command line, providing the path to the wgsl file you want to parse:
``wgsl-dump -i /path/to/input -o /path/to/output <--dump-conditions|--dump-indices>``

## Licenses

- **Code**: The source code is licensed under the MIT License.
- **Documentation**: The documentation and related materials are licensed under the CC-BY-SA-4.0 License.
