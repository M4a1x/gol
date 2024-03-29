Conway's Game of Life in Rust

=============================

Goal
----

The goal of this is to show possible implementations of the famous Game of Life in Rust in various degrees of optimization. The idea here is to optimize the approach as in data representation and algorithm before optimizing the performed number of instructions. 

This is mainly a project for me to learn the language and might be useful to others.


Requirements
------------


Optimizations/Ideas of the different approaches
-----------------------------------------------

* Naive
  * Cells represented by enums
  * 2x 1D Array of Cells on heap for the board, swapping every generation
  * Go through every cell, access all neighbours and count alive alive ones
* New Cell Model
  * Based on [Graphics Programming Black Book, Chapter 17](http://www.jagregory.com/abrash-black-book/#chapter-17-the-game-of-life)
  * Save count of alive neighbours in each cell, because cell states change rather infrequently
  * Update all neighbours when cell is born or dies
* Change List
  * Based on [Graphics Programming Black Book, Chapter 18](http://www.jagregory.com/abrash-black-book/#chapter-18-its-a-plain-wonderful-life)
  * In addition to the grid, have a change list, that contains all cells that might change next generation and loop over this instead of the whole grid
* Table
  * Lookup the next state based on current state and neighbours in a table, instead of calculating everything every time


About the Game
--------------

* Cells change state quite infrequently
* Most of the time, the board is mostly empty
* Lots of repeating patterns


Patterns
--------
Example patterns and interesting objects can be found in the `patterns` subdirectory. I am planning to add any patterns there that I can find. Feel free to point me to missing ones! If I missed any attribution please contact me. Most files contain information about the author(s) and I try to mention previous collectors.

Currently includes:
* `patterns/lifep`: [lifep collection](http://www.ibiblio.org/lifepatterns/lifep.zip) by Alan Hensel
* `patterns/jason`: [All collections on this website](http://entropymine.com/jason/life/) by Jason Summers, Dietrich Leithner, Peter Rott, Paul Tooke, Helmut Postl
* `patterns/golly`: [Taken from the Golly 4.1 source distribution](https://sourceforge.net/projects/golly/files/golly/golly-4.1/)
* `patterns/conwaylife-wiki`: [Collection of all patterns used in the cowaylife.com wiki](https://conwaylife.com/wiki/Main_Page)

ToDo
----

* Switch from to [criterion](https://github.com/bheisler/criterion.rs) for benchmarking?
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
  * Take inspirations from [conwaylife.com forum](https://conwaylife.com/forums/viewtopic.php?f=7&t=20&start=50), especially the 2019 ioccc entry by dogon
  * Don't forget rayon
  * Use hasbrown SwissTable instead of std HashTable
* Get more input from [jason's life page](http://entropymine.com/jason/life/)
* Take a look at [Sprout Life](https://github.com/ShprAlex/SproutLife)
* [Example Patterns (all!?) & JS Hashlife Implementation](https://github.com/copy/life)
* [Comprehensive Collection & Explanations, conwaylife.com](https://conwaylife.com/)
* Read in of [.lif/.life files among others](http://www.mirekw.com/ca/ca_files_formats.html)
* Output via tui+crossterm
* CLI parsing with clap

See also
--------

* [ca-rules](https://crates.io/crates/ca-rules) - A parser for rule strings
* [ca-formats](https://crates.io/crates/ca-formats) - Parses RLE, Plaintext, apgcode & Macrocell format
* [game-of-life-parsers](https://crates.io/crates/game-of-life-parsers) - Parses Life 1.05 & Life 1.06
* [rlifesrc](https://crates.io/crates/rlifesrc) - Searches Game of Life Patterns