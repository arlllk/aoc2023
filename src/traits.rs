pub trait NuType {
    fn into_inner(self) -> u64;
    fn new(value: u64) -> Self;
}

pub trait Mapper {
    type Source: NuType + Copy;
    type Destination: NuType;
    fn destination_range_start(&self) -> u64;
    fn source_range_start(&self) -> u64;
    fn range_length(&self) -> u64;
    fn new(destination_range_start: u64, source_range_start: u64, range_length: u64) -> Self;
}

pub trait Mappable<S, D> {
    type Mapper: Mapper<Source = S, Destination = D>;
    fn try_map(mapper: &Self::Mapper, source: &S) -> Option<D>;
    fn map(mapper: &[Self::Mapper], source: &S) -> D;
}
impl<T: Mapper> Mappable<T::Source, T::Destination> for T {
    type Mapper = T;
    fn try_map(mapper: &Self::Mapper, source: &T::Source) -> Option<T::Destination> {
        let initial_value = source.into_inner();
        let is_in_range = initial_value >= mapper.source_range_start()
            && initial_value < mapper.source_range_start() + mapper.range_length();
        if is_in_range {
            let added = initial_value - mapper.source_range_start();
            let destination_value = mapper.destination_range_start() + added;
            Some(T::Destination::new(destination_value))
        } else {
            None
        }
    }

    fn map(mapper: &[Self::Mapper], source: &T::Source) -> T::Destination {
        mapper
            .iter()
            .flat_map(|mapper| T::try_map(mapper, source))
            .next()
            .unwrap_or(T::Destination::new(source.into_inner()))
    }
}
