Conway's Game of Life in Rust
=============================

Goal
----

The goal of this is to show possible implementations of the famous Game of Life in Rust in various degrees of optimization. This is mainly a project for me to learn the language and might be useful to others.


Requirements
------------

The branches `optimize` and `reference` need `gperftools` to be installed on the system.

ToDo
----

* Switch from cpuprofiler/gperftools to [criterion](https://github.com/bheisler/criterion.rs) for benchmarking?
* Use one branch with e.g. one folder/module per optimized implementation
* Possible optimization approaches to implement
  * Change List
  * Quad Tree + Hash Table (Hashlife) [Explanation](http://www.drdobbs.com/jvm/an-algorithm-for-compressing-space-and-t/184406478), maybe use [PH-Tree (faster?)](https://github.com/tzaeschke/phtree), or [kd-trees (faster than quad, slower than ph?)](https://stackoverflow.com/questions/13487953/difference-between-quadtree-and-kd-tree), or [r-tree?](https://stackoverflow.com/questions/4326332/what-is-the-difference-between-a-kd-tree-and-a-r-tree), CoverTree?
  * Sparse Matrix
  * qp trie ?
  * [Chapters 17/18 of the Black Book](http://www.jagregory.com/abrash-black-book/#chapter-17-the-game-of-life)
  * [List Life, Parallelize and other speedups](http://dotat.at/prog/life/life.html)
  * [XLife](http://www.conwaylife.com/wiki/Xlife)
  * Run on GPU with e.g. [ArrayFire](https://github.com/arrayfire/arrayfire) or [RustaCUDA](https://github.com/bheisler/RustaCUDA)
  * Take inspirations from [Golly](https://sourceforge.net/projects/golly/)
  * Read in of [.lif/.life files among others](http://www.mirekw.com/ca/ca_files_formats.html)