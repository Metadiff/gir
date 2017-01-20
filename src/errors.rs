//use primitives::*;

error_chain!{
        errors {
            /// Error when attempting to use together expressions from different graphs
            NotFromTheSameGraph {
                description("Trying to use expressions which are not from the same graph.")
                display("Trying to use expressions which are not from the same graph.")
            }

            /// Error when trying trying to call `Expr::get()`, but the index is invalid.
            InvalidExprAccess(index: usize) {
                description("Trying to access an invalid expression index.")
                display("Trying to access the invalid expression index {}.", index)
            }

            /// Error when calling forward or reverse mode differentiation with wrong number
            /// of projection tensors.
            InvalidNumberOfProjectionTensors(expected: usize, actual: usize) {
                description("Invalid number of projection tensors.")
                display("Invalid number of projection tensors - expected {}, actual - {}.",
                expected, actual)
            }

            /// Error when attempting to perform an operation on tensors whose shapes are
            /// not compatible (none of them is broadcastable to the other)
            InvalidShapes(shape1: String, shape2: String) {
                description("The shapes given are incompatible.")
                display("The shapes {} and {} are incompatible", shape1, shape2)
            }

            /// Error when requesting a derivative, but the functions do not depend on one of the
            /// parameters
            IndependentDerivative(index: usize) {
                description("The functions 'f' are independent of the tensor.")
                display("The functions 'f' are independent of the tensor 'x' at index {}.", index)
            }

            InvalidArguments(a: usize) {
                description("invalid args")
                display("invalid args")
            }
        }
//        foreign_links {
//            LibUsb(::libusb::Error);
//            Io(::std::io::Error);
//        }
    }

//#[derive(Debug, Clone)]
//pub enum OperatorError {
//    InvalidArguments,
//}

//#[derive(Debug, Clone)]
//pub enum DerivativeError {
//    NoFunctionsProvided,
//    InvalidNumberOfProjectionTensors,
//    InvalidShapeOfProjectionTensor(usize),
//    BadMe
//}

//#[derive(Debug, Clone)]
//pub enum GraphError {
//    InvalidAccess(usize),
//    Internal,
//    IncompatibleNodes(String, String),
//}

