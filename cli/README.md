midas CLI
=========

[![](http://meritbadge.herokuapp.com/midas_cli)](https://crates.io/crates/midas_cli)

Rust implementation of https://github.com/bhatiasiddharth/MIDAS

Installable via

```sh
cargo install midas_cli
```

Given a CSV file with three whole number columns (`source_node`,
`destination_node`, and `time` (where `time` is monotonically
increasing)) like the following

`example.csv`
```csv
2,3,1
2,3,1
3,4,2
3,4,2
5,9,2
5,9,3
7,73,3
11,74,5
```

run via

```sh
midas_cli --directed example.csv
```

For information on the algorithm or how to interpret the output see the
link above and the corresponding paper.
