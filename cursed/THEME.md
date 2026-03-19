In Rust I am creating a CLI tui in rust. Now for that I have to maintain a theme so what's the best way to maintain a theme in Rust for CLI tui?
1. Use rkyv machine formatted theme in binary so we have zero-copy fast themes with ykyb and memmap2 with rayon crate.
2. Use Lua rather than theme.
Please give me the best solution for maintaining the best UI in CLI tui Rust.
