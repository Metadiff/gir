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

   1. Automatic shape verification (even of dynamic shapes) on graph construction time. 
   This feature will be of great benefit for researchers and rapid development. 
   Error messages on invalid shapes are pushed on the exact line where they are violated.
   Additionally, this feature guarantees that there will be no runtime errors during the 
   function execution. This is pretty well aligned in the spirit of Rust.

   2. Full separation of the mathematical graph intermediate representation (GIR) 
    together with the autodiff engine, the compiler optimization procedures and 
    the backend engines. 
    
   3. OpenCL integration. No framework yet supports properly OpenCL. However, if we 
    exclude Nvidia devices, OpenCL runs on anything - Intel or AMD chips, Intel or 
    AMD graphic cards, FPGAs and other embedded devices. Thus we believe that it is 
    a better abstraction than CUDA. 
   
   4. Ports to other languages. The fact that we develop GIR in Rust means that it is 
   can be easily ported to any other languages and be reused anywhere.
   
   5. Explicit compiler control commands in the graph via auxiliary nodes. This can 
   include commands like fork, join or barriers. 
   
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