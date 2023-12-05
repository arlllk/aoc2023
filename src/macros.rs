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
    };
}

#[macro_export]
macro_rules! impl_mappeable {
    ( $s:ident, $d:ident ) => {
        impl Mappable for $s {
            type Source = $s;
            type Destination = $d;
        }
    };
}

#[macro_export]
macro_rules! generate_mapper {
    ($name:ident, $destination:ident, $source:ident) => {
        #[derive(PartialEq, Ord, PartialOrd, Eq, Copy, Clone, Debug)]
        struct $name {
            destination_range_start: u64,
            source_range_start: u64,
            range_length: u64,
        }

        impl $crate::traits::Mapper for $name {
            type Source = $source;
            type Destination = $destination;
            fn destination_range_start(&self) -> u64 {
                self.destination_range_start
            }
            fn source_range_start(&self) -> u64 {
                self.source_range_start
            }
            fn range_length(&self) -> u64 {
                self.range_length
            }

            fn new(
                destination_range_start: u64,
                source_range_start: u64,
                range_length: u64,
            ) -> Self {
                Self {
                    destination_range_start,
                    source_range_start,
                    range_length,
                }
            }
        }
    };
}
