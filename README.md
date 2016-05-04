# Endo - ICFP 2007

An implementation and walkthrough of the [ICFP 2007 Programming Contest](http://save-endo.cs.uu.nl/) in Rust.
The task is described in [Endo.pdf](Endo.pdf).

## Components

### DNA -> RNA

In `src\dna.rs` there is an implementation of the DNA->RNA processor.

This uses a rope data structure from the `xi-editor` project.  A copy of the crate is in `rope`.

### RNA -> Image

In `src\rna.rs` there is an implementation of the RNA renderer.

This generates a full `.png` by default, but can render intermediate steps in the rendering with the `-i` flag.

## Walkthrough

First, make sure things build.

```bash
cargo build --release
```

Render the original Endo image with no prefix. 

```bash
cargo run --release
```

Render the self-check based on the hint in Endo.pdf page 21.

```bash
cargo run --release -- IIPIFFCPICICIICPIICIPPPICIIC
```
Render all of the intermediate results - the first 13 build another prefix. 

```bash
cargo run --release -- -i IIPIFFCPICICIICPIICIPPPICIIC
```

Render the Fuun Field Repair Guide based on the prefix from previous step.

```bash
cargo run --release -- IIPIFFCPICFPPICIICCIICIPPPFIIC
```

Rotate the sun per the Fuun Field Repair Guide.

```bash
cargo run --release -- IIPIFFCPICPCIICICIICIPPPPIIC
```

