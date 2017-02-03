
#[macro_export]
macro_rules! shape {
    ( ) => {{
        use $crate::primitives::Shape;
        Shape::scalar_shape()
    }};
    ( $dim0: expr ) => {{
        use $crate::primitives::Shape;
        Shape::vector_shape($dim0.into())
    }};
    ( $dim0: expr, $dim1: expr ) => {{
        use $crate::primitives::Shape;
        Shape::matrix_shape($dim0.into(), $dim1.into())
    }};
    ( $dim0: expr, $dim1: expr, $dim2: expr ) => {{
        use $crate::primitives::Shape;
        Shape::tensor3_shape($dim0.into(), $dim1.into(), $dim2.into())
    }};
    ( $dim0: expr, $dim1: expr, $dim2: expr, $dim3: expr ) => {{
        use $crate::primitives::Shape;
        Shape::tensor4_shape($dim0.into(), $dim1.into(), $dim2.into(), $dim3.into())
    }};
}

#[macro_export]
macro_rules! param {
    ( $graph: expr, $type_: expr, ( $( $dim: expr ),* ), $name: expr ) => {{
        $graph.parameter($type_, shape!( $($dim),* ), $name.into())
    }};
}

#[macro_export]
macro_rules! b_param {
    ( $graph: expr, ( $( $dim: expr ),* ), $name: expr ) => {{
        use $crate::primitives::FundamentalType;
        $graph.parameter(FundamentalType::Boolean, shape!( $($dim),* ), $name.into())
    }};
}

#[macro_export]
macro_rules! u_param {
    ( $graph: expr, ( $( $dim: expr ),* ), $name: expr ) => {{
        use $crate::primitives::FundamentalType;
        param!($graph, FundamentalType::UnsignedInt, ( $($dim),* ), $name)
    }};
}

#[macro_export]
macro_rules! i_param {
    ( $graph: expr, ( $( $dim: expr ),* ), $name: expr ) => {{
        use $crate::primitives::FundamentalType;
        param!($graph, FundamentalType::SignedInt, ( $($dim),* ), $name)
    }};
}

#[macro_export]
macro_rules! f_param {
    ( $graph: expr, ( $( $dim: expr ),* ), $name: expr ) => {{
        use $crate::primitives::FundamentalType;
        param!($graph, FundamentalType::Float, ( $($dim),* ), $name)
    }};
}

#[macro_export]
macro_rules! c_param {
    ( $graph: expr, ( $( $dim: expr ),* ), $name: expr ) => {{
        use $crate::primitives::FundamentalType;
        param!($graph, FundamentalType::Complex, ( $($dim),* ), $name)
    }};
}

#[macro_export]
macro_rules! var {
    ( $graph: expr, $type_: expr, ( $( $dim: expr ),* )) => {{
        $graph.input($type_, shape!( $($dim),* ), None)
    }};
    ( $graph: expr, $type_: expr, ( $( $dim: expr ),* ), $name: expr ) => {{
        $graph.input($type_, shape!( $($dim),* ), Some($name.into()))
    }};
}

#[macro_export]
macro_rules! b_var {
    ( $graph: expr, ( $( $dim:expr ),* ) ) => {{
        use $crate::primitives::FundamentalType;
        var!($graph, FundamentalType::Boolean, ( $($dim),* ))
    }};
    ( $graph: expr, ( $( $dim: expr ),* ), $name: expr ) => {{
        use $crate::primitives::FundamentalType;
        var!($graph, FundamentalType::Boolean, ( $($dim),* ), $name)
    }};
}

#[macro_export]
macro_rules! u_var {
    ( $graph: expr, ( $( $dim:expr ),* ) ) => {{
        use $crate::primitives::FundamentalType;
        var!($graph, FundamentalType::UnsignedInt, ( $($dim),* ))
    }};
    ( $graph: expr, ( $( $dim: expr ),* ), $name: expr ) => {{
        use $crate::primitives::FundamentalType;
        var!($graph, FundamentalType::UnsignedInt, ( $($dim),* ), $name)
    }};
}

#[macro_export]
macro_rules! i_var {
    ( $graph: expr, ( $( $dim:expr ),* ) ) => {{
        use $crate::primitives::FundamentalType;
        var!($graph, FundamentalType::SignedInt, ( $($dim),* ))
    }};
    ( $graph: expr, ( $( $dim: expr ),* ), $name: expr ) => {{
        use $crate::primitives::FundamentalType;
        var!($graph, FundamentalType::SignedInt, ( $($dim),* ), $name)
    }};
}

#[macro_export]
macro_rules! f_var {
    ( $graph: expr, ( $( $dim:expr ),* ) ) => {{
        use $crate::primitives::FundamentalType;
        var!($graph, FundamentalType::Float, ( $($dim),* ))
    }};
    ( $graph: expr, ( $( $dim: expr ),* ), $name: expr ) => {{
        use $crate::primitives::FundamentalType;
        var!($graph, FundamentalType::Float, ( $($dim),* ), $name)
    }};
}

#[macro_export]
macro_rules! c_var {
    ( $graph: expr, ( $( $dim:expr ),* ) ) => {{
        use $crate::primitives::FundamentalType;
        var!($graph, FundamentalType::Complex, ( $($dim),* ))
    }};
    ( $graph: expr, ( $( $dim: expr ),* ), $name: expr ) => {{
        use $crate::primitives::FundamentalType;
        var!($graph, FundamentalType::Complex, ( $($dim),* ), $name)
    }};
}