# Graph Intermediate Representation for Rust

# Potential roadmap

1. Implement all basic operations used in standard 
neural networks which would be needed for comparative bachmarks 
(e.g. Conv2D, MaxPooling, GEMM, elementwise nonlinearities 
and reductions)

2. Implement a simple native backend which can execute a graph function.
there are two possible ways to do this:
    1. Source code generation and dynamic linking 
    2. Runtime evaluation traversing the graph. 
    
    Additionally a memory manager would be required, which to allow for 
    reusing nodes along the computation when they are no longer needed. 
    This can include a runtime memory allocator (like Tensorflow) or a 
    static one (preferably). Linking ot BLAS would be required as well
    as automatically detecting MKL / LAPACK / OpenBLAS installations. 
    The Leaf native backend might be a good starting point. Otherwise 
    for future development we should aim to use things like SIMD / Rayon 
    and futures from tokio for optimal utilization of the CPU cores. 
    
3. Implement a simple GPU backend. Again this can be done either by 
source code generation or traversing the graph. However, this backend 
will require to at least source right and compile the GPU kernels. 
The memory manager from the native backend can be reused just working in
GPU memory. We have crates like [nvptx](https://github.com/japaric/nvptx) 
which can compile Rust to ptx, so we can generate Rust code rather than
C. Or there is [HIP](https://github.com/GPUOpen-ProfessionalCompute-Tools/HIP) 
which can allows us to generate the same kernels for both Nvidia and AMD 
cards by generating C. There is also [ocl](https://github.com/cogciprocate/ocl) 
which is native Rust binding for OpenCL. There needs to be a major 
decision on which way to go if we want to support both. Additionally, 
bindings to libraries like cuDNN are a must as well. 

4. If everything is working we should write a few simple and sensible
 graph optimizations such that we can achieve comparable run times to
 leading frameworks like Theano and Tensorflow.
 
 From there on we can refine and reiterate each one of these 
 semi independently. 
