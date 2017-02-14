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
    P8 = 1,
    P16 = 2,
    P32 = 4,
    P64 = 8,
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
    Quaternary = 4,
    Quinary = 5,
    Nary = 6
}

impl ::std::fmt::Display for Arity {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        match *self {
            Arity::Nullary => write!(fmt, "Nullary"),
            Arity::Unary => write!(fmt, "Unary"),
            Arity::Binary => write!(fmt, "Binary"),
            Arity::Ternary => write!(fmt, "Ternary"),
            Arity::Quaternary => write!(fmt, "Quaternary"),
            Arity::Quinary => write!(fmt, "Quinary"),
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

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum Axis {
    Axis0 = 0,
    Axis1 = 1,
    Axis2 = 2,
    Axis3 = 3
}

impl Axis {
    pub fn iter() -> ::std::slice::Iter<'static, Axis> {
        static ALL: &'static [Axis] = &[Axis::Axis0, Axis::Axis1, Axis::Axis2, Axis::Axis3];
        ALL.iter()
    }
}

impl ::std::fmt::Display for Axis {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        match *self {
            Axis::Axis0 => write!(fmt, "0"),
            Axis::Axis1 => write!(fmt, "1"),
            Axis::Axis2 => write!(fmt, "2"),
            Axis::Axis3 => write!(fmt, "3"),
        }
    }
}

/// Symbolic integer used for shapes
pub type SymInt = Polynomial<String, i64, u8>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Dim {
    Constant(usize),
    Variable(String)
}

impl<'a> ::std::convert::From<&'a str> for Dim {
    fn from(s: &'a str) -> Self {
        Dim::Variable(format!("{}", s))
    }
}

impl ::std::convert::From<String> for Dim {
    fn from(s: String) -> Self {
        Dim::Variable(s)
    }
}

impl ::std::convert::From<usize> for Dim {
    fn from(v: usize) -> Self {
        Dim::Constant(v)
    }
}

impl ::std::convert::Into<SymInt> for Dim {
    fn into(self) -> SymInt {
        match self {
            Dim::Constant(x) => (x as i64).into(),
            Dim::Variable(id) => ::symbolic_polynomials::variable(id)
        }
    }
}

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

    pub fn elements(&self) -> SymInt {
        &self.0 * &self.1 * &self.2 * &self.3
    }

    pub fn get(&self, axis: Axis) -> &SymInt {
        match axis {
            Axis::Axis0 => &self.0,
            Axis::Axis1 => &self.1,
            Axis::Axis2 => &self.2,
            Axis::Axis3 => &self.3,
        }
    }

    pub fn set(&mut self, axis: Axis, value: SymInt) -> () {
        match axis {
            Axis::Axis0 => {self.0 = value;},
            Axis::Axis1 => {self.1 = value;},
            Axis::Axis2 => {self.2 = value;},
            Axis::Axis3 => {self.3 = value;},
        }
    }

    pub fn scalar_shape() -> Self {
        Shape(1.into(), 1.into(), 1.into(), 1.into())
    }

    pub fn vector_shape(dim0: Dim) -> Self {
        Shape(dim0.into(), 1.into(), 1.into(), 1.into())
    }

    pub fn matrix_shape(dim0: Dim, dim1: Dim) -> Self {
        Shape(dim0.into(), dim1.into(), 1.into(), 1.into())
    }

    pub fn tensor3_shape(dim0: Dim, dim1: Dim, dim2: Dim) -> Self {
        Shape(dim0.into(), dim1.into(), dim2.into(), 1.into())
    }

    pub fn tensor4_shape(dim0: Dim, dim1: Dim, dim2: Dim, dim3: Dim) -> Self {
        Shape(dim0.into(), dim1.into(), dim2.into(), dim3.into())
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

impl Default for Policy {
    fn default() -> Self {
        Policy::Warn
    }
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

//#[derive(Debug, Clone)]
//pub struct ScopedName {
//    pub scoped_name: Vec<String>,
//    pub scope_delimiter: String
//}
//
//impl PartialEq for ScopedName {
//    fn eq(&self, other: &ScopedName) -> bool {
//        if self.scoped_name.len() != other.scoped_name.len() {
//            return false
//        }
//        for (name1, name2) in self.scoped_name.iter().zip(other.scoped_name.iter()) {
//            if name1 != name2 {
//                return false
//            }
//        }
//        return true
//    }
//}
//
//impl Eq for ScopedName {}
//
//impl ::std::hash::Hash for ScopedName {
//    fn hash<H: ::std::hash::Hasher>(&self, state: &mut H) {
//        self.scoped_name.join(&self.scope_delimiter).hash(state);
//    }
//}
//
//impl ::std::fmt::Display for ScopedName {
//    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
//        write!(f, "{}", self.scoped_name.join(&self.scope_delimiter))
//    }
//}


