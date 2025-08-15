# rustviz

Generate GraphViz Dot and C4 diagrams for your Rust projects
## Features

- Auto-detect workspaces
- Pick the output format (GraphViz/C4)


## Usage / Examples

Run the `rustviz` command either in a cargo workspace or in a crate.
`rustviz` will then output a graphviz dot file that you can open in [graphviz-online](https://dreampuf.github.io/GraphvizOnline/).

```bash
  rustviz
```

You can specify a different project path after the invocation:
```bash
  rustviz ./mycoolproject
```

Alternatively, you can force it to output a C4 Diagram in the PlantUML using the option `-f c4`, but that output is experimental since auto layout is not implemented. It is viewable in the 
[MermaidJS Live Editor](https://mermaid.live/edit).

```bash
  rustviz -f c4
```


### Other Available Options / Help

```
Usage: rustviz [OPTIONS] [PROJECT_PATH]

Arguments:
  [PROJECT_PATH]  [default: .]

Options:
  -o, --output-path <OUTPUT_PATH>            [default: -]
  -f, --output-format <OUTPUT_FORMAT>        [default: graphviz] [possible values: c4, graphviz]
  -d, --detect-workspace <DETECT_WORKSPACE>  [default: auto] [possible values: yes, no, auto]
  -h, --help                                 Print help
  -V, --version                              Print version
```


## Installation

Install `rustviz` with brew

```bash
  brew install leopoldlabs/apps/rustviz
```

Or build from source (you need the [rust toolchain](https://rustup.rs/) installed)

```bash
  cargo b --release
  cp target/release/rustviz /whatever/your/bin/folder
```
    
## Feedback

If you have any feedback, please reach out to me at rustviz@leoj.de


## FAQ

#### Who/What is this for?

My seminar paper.

## Related 

Here are some related projects and commands that do similar things

- `cargo tree`
- [regexident / cargo-modules](https://github.com/regexident/cargo-modules)
- [SelamaAshalanore / rudg](https://github.com/SelamaAshalanore/rudg)
- [robinmoussu / cargo-callgraph](https://github.com/robinmoussu/cargo-callgraph)
- [nrc / callgraph.rs](https://github.com/nrc/callgraph.rs)


