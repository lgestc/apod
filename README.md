# Astronomy Picture of the Day in your terminal

Downloads and writes the Astronomy Picture of the Day to standard output

## Why
Just to get into Rust smoothly after a break.

## Installation

Prerequisites: **rust toolchain installed**

It is only possible to install from the source ATM. Clone the repository and run
`cargo install --path .`.

## Usage

Either redirect the output to a file with `>` operator or pipe it to something
like `display` (from `imagemagick` package).
