
#[macro_export]
macro_rules! f_param {
    ( $graph: expr, $name: expr) => {{
        use $crate::primitives;
        $graph.parameter(primitives::FundamentalType::Float,
            primitives::Shape::scalar_shape(),
            $name.into())
    }};
    ( $graph: expr, ($dim0: expr), $name: expr) => {{
        use $crate::primitives;
        $graph.parameter(primitives::FundamentalType::Float,
            primitives::Shape::vector_shape($dim0.into()),
            $name.into())
    }};
    ( $graph: expr, ($dim0: expr, $dim1: expr), $name: expr) => {{
        use $crate::primitives;
        $graph.parameter(primitives::FundamentalType::Float,
            primitives::Shape::matrix_shape($dim0.into(), $dim1.into()),
            $name.into())
    }};
    ( $graph: expr, ($dim0: expr, $dim1: expr, $dim2: expr), $name: expr) => {{
        use $crate::primitives;
        $graph.parameter(primitives::FundamentalType::Float,
            primitives::Shape::tensor3_shape($dim0.into(), $dim1.into(), $dim2.into()),
            $name.into())
    }};
    ( $graph: expr, ($dim0: expr, $dim1: expr, $dim2: expr, $dim3: expr), $name: expr) => {{
        use $crate::primitives;
        $graph.parameter(primitives::FundamentalType::Float,
            primitives::Shape::tensor4_shape($dim0.into(), $dim1.into(), $dim2.into(), $dim3.into()),
            $name.into())
    }};
}

#[macro_export]
macro_rules! f_var {
    ( $graph: expr ) => {{
        $graph.f_scalar(None)
    }};
    ( $graph: expr, $name: expr) => {{
        $graph.f_scalar(Some($name.into()))
    }};
    ( $graph: expr, ($dim0: expr)) => {{
        $graph.f_vector($dim0.into(), None)
    }};
    ( $graph: expr, ($dim0: expr), $name: expr) => {{
        $graph.f_vector($dim0.into(), Some($name.into()))
    }};
    ( $graph: expr, ($dim0: expr, $dim1: expr)) => {{
        $graph.f_matrix($dim0.into(), $dim1.into(), None)
    }};
    ( $graph: expr, ($dim0: expr, $dim1: expr), $name: expr) => {{
        $graph.f_matrix($dim0.into(), $dim1.into(), Some($name.into()))
    }};
    ( $graph: expr, ($dim0: expr, $dim1: expr, $dim2: expr)) => {{
        $graph.f_tensor3($dim0.into(), $dim1.into(), $dim2.into(), None)
    }};
    ( $graph: expr, ($dim0: expr, $dim1: expr, $dim2: expr), $name: expr) => {{
        $graph.f_tensor3($dim0.into(), $dim1.into(), $dim2.into(), Some($name.into()))
    }};
    ( $graph: expr, ($dim0: expr, $dim1: expr, $dim2: expr, $dim3: expr)) => {{
        $graph.f_tensor4($dim0.into(), $dim1.into(), $dim2.into(), $dim3.into(), None)
    }};
    ( $graph: expr, ($dim0: expr, $dim1: expr, $dim2: expr, $dim3: expr), $name: expr) => {{
        $graph.f_tensor4($dim0.into(), $dim1.into(), $dim2.into(), $dim3.into(), Some($name.into()))
    }};
}