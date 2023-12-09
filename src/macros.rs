#[macro_export]
macro_rules! impl_nu_type {
    ( $x:ident  ) => {
        impl $crate::traits::NuType for $x {
            fn into_inner(self) -> u64 {
                self.0
            }
            fn new(value: u64) -> Self {
                Self(value)
            }
        }

        impl Display for $x {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.into_inner())
            }
        }

        impl std::ops::Add<$x> for $x {
            type Output = $x;

            fn add(self, rhs: $x) -> Self::Output {
                $x::new(self.into_inner() + rhs.into_inner())
            }
        }

        impl std::ops::Sub<$x> for $x {
            type Output = $x;

            fn sub(self, rhs: $x) -> Self::Output {
                $x::new(self.into_inner() - rhs.into_inner())
            }
        }

        impl std::ops::Sub<$x> for &$x {
            type Output = $x;

            fn sub(self, rhs: $x) -> Self::Output {
                $x::new(self.into_inner() - rhs.into_inner())
            }
        }

        impl std::ops::Sub<&$x> for $x {
            type Output = $x;

            fn sub(self, rhs: &$x) -> Self::Output {
                $x::new(self.into_inner() - rhs.into_inner())
            }
        }
    };
}
