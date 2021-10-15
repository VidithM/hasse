## Graph visualizer using Rust and FLTK
* After learning about binary relations and partially ordered sets in discrete structures, I was curious about the problem of generating the Hasse diagram of a poset, in particular removing the edges implied by transitivity
* I quickly found out that this is known as the transitive reduction problem
* Solving this is NP-hard for general graphs, but for DAGs it can be solved in near-quadratic complexity
* I am also interested in a similar problem known as transitive closure, which is the converse of what is described above
* Also, I started learning Rust to try something new, so decided to make a graph visualizer that supports transitive reductions and closures of DAGs.
