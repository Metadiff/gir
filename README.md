# GIR

Graph Intermediate Representation (GIR) library for ML

## Motivation and goals

There are many packages in existing using a graph abstraction for Machine Learning, 
or equivalently for compilers. The emphasis of GIR is that it will be an independent 
Intermediate Representation with no backend, language or compute engine attached. 
It will allow for a unified representation of abstract mathematical programs over 
large tensors. The paradigm comes from the LLVM compiler, where similarly the backends
are fully separated. This has several benefits, but mainly it allows for reuse of the 
abstract representation and of rapid and independent development of backends. 
As Rust is a very portable language we hope that anyone starting to write their own 
package, would just reuse the GIR and all of its autodiff capabilities, while all 
he needs to do is then implement a frontend and a backend, or alternatively reuse 
one of the existing ones.

## Main distinguishable features
  1. Fully abstract IR for mathematic computation, independent of the backend - 
  towards LLVM paragidm. 
  
  2. Automatic constant and dynamic shape verification at graph construction time.
  This will hopefully make the need of recompilation of graphs, where only a single 
  shape has changed redundant, potentially improving compile times.
  
  3. Ports and interfaces to other languages and easy integration via FFI.
  
  4. Explicit compiler control commands in the graph via auxiliary nodes. This can 
   include commands like fork, join or barriers. 
   
  5. Optimizations based on memory budged - always get the maximum speed for a fixed
  memory limit you have.
  
   
## Current targets

Main focus is on implementing basic operations, their derivatives and bring up the 
Arrayfire backend up to speed, such that we can demonstrate working examples. 
The OpenCL backend is also in progress.
   
## Community/Contact

A group of GIR developers and users can be found in the [Gitter/Rust-ML][gitter] room.

## License

Licensed under either of
  * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
    http://www.apache.org/licenses/LICENSE-2.0)
  * MIT license ([LICENSE-MIT](LICENSE-MIT) or
    http://opensource.org/licenses/MIT) at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you shall be dual licensed as above, without any
additional terms or conditions.

[gitter]: https://gitter.im/rust-ml/Lobby
