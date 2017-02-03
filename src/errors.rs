use primitives::*;

error_chain!{
        errors {
            /// Error when trying trying to call `Expr::get()`, but the index is invalid.
            InvalidExprAccess(index: usize) {
                description("Trying to access an invalid expression index.")
                display("Trying to access the invalid expression index {}.", index)
            }

            /// Error when attempting to perform an operation on tensors whose shapes are
            /// not compatible (none of them is broadcastable to the other)
            InvalidShapes(op_name: String, shape1: String, shape2: String) {
                description("The shapes given are incompatible.")
                display("The shapes {} and {} given to operator {} are incompatible",
                shape1, shape2, op_name)
            }

            InvalidArguments(op: String, args: Vec<usize>, msg: String) {
                description("Invalid arguments.")
                display("Invalid arguments: {:?}. '{}' message: {}", args, op, msg)
            }

            Downcast(from: FundamentalType, to: FundamentalType) {
                description("Down casting tensor.")
                display("Down casting tensor from {} to {}.", from, to)
            }
        }
//        foreign_links {
//            LibUsb(::libusb::Error);
//            Io(::std::io::Error);
//        }
    }
