use symbolic_polynomials::Polynomial;

/// Fundamental mathematical variable types
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum FundamentalType {
    Boolean = 0,
    UnsignedInt = 1,
    SignedInt = 2,
    Float = 3,
    Complex = 4
}

impl ::std::fmt::Display for FundamentalType {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        match *self {
            FundamentalType::Boolean => write!(fmt, "Boolean"),
            FundamentalType::UnsignedInt => write!(fmt, "UnsignedInt"),
            FundamentalType::SignedInt => write!(fmt, "SignedInt"),
            FundamentalType::Float => write!(fmt, "Float"),
            FundamentalType::Complex => write!(fmt, "Complex"),
        }
    }
}

/// Variable storage precisions
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum Precision {
    P8 = 0,
    P16 = 1,
    P32 = 2,
    P64 = 3,
}

impl ::std::fmt::Display for Precision {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        match *self {
            Precision::P8 => write!(fmt, "P8"),
            Precision::P16 => write!(fmt, "P16"),
            Precision::P32 => write!(fmt, "P32"),
            Precision::P64 => write!(fmt, "P64"),
        }
    }
}


/// Operator arity (Number of arguments)
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum Arity {
    Nullary = 0,
    Unary = 1,
    Binary = 2,
    Ternary = 3,
    Nary = 4
}

impl ::std::fmt::Display for Arity {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        match *self {
            Arity::Nullary => write!(fmt, "Nullary"),
            Arity::Unary => write!(fmt, "Unary"),
            Arity::Binary => write!(fmt, "Binary"),
            Arity::Ternary => write!(fmt, "Ternary"),
            Arity::Nary => write!(fmt, "Nary"),
        }
    }
}


/// Constraint for matrix positivity (for square matrices only)
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum MatrixPositivity {
    Indefinite = 0,
    PositiveSemiDefinite = 1,
    PositiveDefinite = 2,
    NegativeDefinite = 3,
    NegativeSemiDefinite = 4
}

impl ::std::fmt::Display for MatrixPositivity {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        match *self {
            MatrixPositivity::Indefinite => write!(fmt, "Indefinite"),
            MatrixPositivity::PositiveSemiDefinite => write!(fmt, "PositiveSemiDefinite"),
            MatrixPositivity::PositiveDefinite => write!(fmt, "PositiveDefinite"),
            MatrixPositivity::NegativeSemiDefinite => write!(fmt, "NegativeSemiDefinite"),
            MatrixPositivity::NegativeDefinite => write!(fmt, "NegativeDefinite"),
        }
    }
}

/// Constraint for matrix symmetry (for square matrices only)
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum MatrixSymmetry {
    NonSymmetric = 0,
    Symmetric = 1,
    SkewSymmetric = 2
}

impl ::std::fmt::Display for MatrixSymmetry {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        match *self {
            MatrixSymmetry::NonSymmetric => write!(fmt, "NonSymmetric"),
            MatrixSymmetry::Symmetric => write!(fmt, "Symmetric"),
            MatrixSymmetry::SkewSymmetric => write!(fmt, "SkewSymmetric"),
        }
    }
}

/// Constraint for matrix fill
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum MatrixFill {
    NonStructuredFill = 0,
    Diagonal = 1,
    TriDiagonal = 2,
    LowerTriangular = 3,
    StrictlyLowerTriangular = 4,
    UpperTriangular = 5,
    StrictlyUpperTriangular = 6
}

impl ::std::fmt::Display for MatrixFill {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        match *self {
            MatrixFill::NonStructuredFill => write!(fmt, "NonStructuredFill"),
            MatrixFill::Diagonal => write!(fmt, "Diagonal"),
            MatrixFill::TriDiagonal => write!(fmt, "TriDiagonal"),
            MatrixFill::LowerTriangular => write!(fmt, "LowerTriangular"),
            MatrixFill::StrictlyLowerTriangular => write!(fmt, "StrictlyLowerTriangular"),
            MatrixFill::UpperTriangular => write!(fmt, "UpperTriangular"),
            MatrixFill::StrictlyUpperTriangular => write!(fmt, "StrictlyUpperTriangular"),
        }
    }
}

/// Symbolic integer used for shapes
pub type SymInt = Polynomial<String, i64, u8>;

/// A tensor shape is just a 4-tuple of SymInt
#[derive(Clone, Debug, PartialEq)]
pub struct Shape(pub SymInt, pub SymInt, pub SymInt, pub SymInt);

impl Shape {
    pub fn order(&self) -> usize {
        if self.3 == 1 {
            if self.2 == 1 {
                if self.1 == 1 {
                    if self.0 == 1 {
                        0
                    } else {
                        1
                    }
                } else {
                    2
                }
            } else {
                3
            }
        } else {
            4
        }
    }

    pub fn scalar_shape() -> Self {
        Shape(1.into(), 1.into(), 1.into(), 1.into())
    }
}

impl ::std::fmt::Display for Shape {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        let f = |x: String| x;
        write!(fmt, "({},{},{},{})",
               self.0.to_code(&f),
               self.1.to_code(&f),
               self.2.to_code(&f),
               self.3.to_code(&f))
    }
}

/// Policy actions for warnings
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum Policy {
    Quite = 0,
    Warn = 1,
    Raise = 2
}

impl ::std::fmt::Display for Policy {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        match *self {
            Policy::Quite => write!(fmt, "Quiet"),
            Policy::Warn => write!(fmt, "Warn"),
            Policy::Raise => write!(fmt, "Raise"),
        }
    }
}

