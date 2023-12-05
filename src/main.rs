mod macros;
mod traits;

use crate::traits::{Mappable, Mapper, NuType};
use nom::bytes::complete::{tag, tag_no_case, take_until, take_while};
use nom::character::complete::line_ending;
use nom::combinator::{map, map_res, opt};
use nom::multi::many1;
use nom::sequence::{preceded, terminated};
use nom::IResult;
use rayon::prelude::*;
use std::fs;

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
struct Seed(u64);
impl_nu_type!(Seed);
#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
struct Soil(u64);
impl_nu_type!(Soil);
generate_mapper!(SoilMapper, Soil, Seed);
#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
struct Fertilizer(u64);
impl_nu_type!(Fertilizer);
generate_mapper!(FertilizerMapper, Fertilizer, Soil);
#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
struct Water(u64);
impl_nu_type!(Water);
generate_mapper!(WaterMapper, Water, Fertilizer);
#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
struct Light(u64);
impl_nu_type!(Light);
generate_mapper!(LightMapper, Light, Water);
#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
struct Temperature(u64);
impl_nu_type!(Temperature);
generate_mapper!(TemperatureMapper, Temperature, Light);
#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
struct Humidity(u64);
impl_nu_type!(Humidity);
generate_mapper!(HumidityMapper, Humidity, Temperature);
#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
struct Location(u64);
impl_nu_type!(Location);
generate_mapper!(LocationMapper, Location, Humidity);

fn parse_list_numbers(input: &str) -> IResult<&str, u64> {
    terminated(
        map_res(take_while(|c: char| c.is_ascii_digit()), |s: &str| {
            s.parse::<u64>()
        }),
        opt(tag(" ")),
    )(input)
}

fn parse_seed(input: &str) -> IResult<&str, Vec<Seed>> {
    map(
        preceded(
            preceded(tag_no_case("seeds:"), opt(tag(" "))),
            many1(parse_list_numbers),
        ),
        |s| s.iter().map(|s| Seed::new(*s)).collect::<Vec<_>>(),
    )(input)
}

fn parse_maps<M: Mapper>(input: &str) -> IResult<&str, Vec<M>> {
    many1(map(
        terminated(many1(parse_list_numbers), opt(line_ending)),
        |list| {
            M::new(
                *list.first().unwrap(),
                *list.get(1).unwrap(),
                *list.get(2).unwrap(),
            )
        },
    ))(input)
}

fn general_map_parser<'a, T: Mapper>(input: &'a str, until: &str) -> IResult<&'a str, Vec<T>> {
    let (advance_until_map, _) = take_until(until)(input)?;
    let (rest, _) = preceded(tag(until), opt(line_ending))(advance_until_map)?;
    parse_maps::<T>(rest)
}

fn decode_input(input: &String) -> InputDecoded {
    let (rest, seeds) = parse_seed(&input).unwrap();
    let (rest, soil_mappers) = general_map_parser::<SoilMapper>(rest, "seed-to-soil map:").unwrap();
    let (rest, fertilizer_mappers) =
        general_map_parser::<FertilizerMapper>(rest, "soil-to-fertilizer map:").unwrap();
    let (rest, water_mappers) =
        general_map_parser::<WaterMapper>(rest, "fertilizer-to-water map:").unwrap();
    let (rest, light_mappers) =
        general_map_parser::<LightMapper>(rest, "water-to-light map:").unwrap();
    let (rest, temperature_mappers) =
        general_map_parser::<TemperatureMapper>(rest, "light-to-temperature map:").unwrap();
    let (rest, humidity_mappers) =
        general_map_parser::<HumidityMapper>(rest, "temperature-to-humidity map:").unwrap();
    let (_, location_mappers) =
        general_map_parser::<LocationMapper>(rest, "humidity-to-location map:").unwrap();
    let seeds_pairs = seeds
        .clone()
        .chunks_exact(2)
        .map(|chunk| (chunk[0], chunk[1]))
        .collect::<Vec<_>>();
    InputDecoded {
        soil_mappers,
        fertilizer_mappers,
        water_mappers,
        light_mappers,
        temperature_mappers,
        humidity_mappers,
        location_mappers,
        seeds,
        seeds_pairs,
    }
}

#[derive(Debug, Clone)]
struct DecodedInfomationFase1 {
    seed: Seed,
    soil: Soil,
    fertilizer: Fertilizer,
    water: Water,
    light: Light,
    temperature: Temperature,
    humidity: Humidity,
    location: Location,
}

fn calculate(input: &InputDecoded) -> Vec<DecodedInfomationFase1> {
    input
        .seeds
        .clone()
        .into_iter()
        .map(|seed| {
            let soil = SoilMapper::map(&input.soil_mappers, &seed);
            let fertilizer = FertilizerMapper::map(&input.fertilizer_mappers, &soil);
            let water = WaterMapper::map(&input.water_mappers, &fertilizer);
            let light = LightMapper::map(&input.light_mappers, &water);
            let temperature = TemperatureMapper::map(&input.temperature_mappers, &light);
            let humidity = HumidityMapper::map(&input.humidity_mappers, &temperature);
            let location = LocationMapper::map(&input.location_mappers, &humidity);
            DecodedInfomationFase1 {
                seed,
                soil,
                fertilizer,
                water,
                light,
                temperature,
                humidity,
                location,
            }
        })
        .collect::<Vec<_>>()
}

fn calculate_ver2(input: &InputDecoded) -> Vec<DecodedInfomationFase1> {
    input
        .seeds_pairs
        .clone()
        .into_par_iter()
        .map(|(start, range)| {
            let mut min = DecodedInfomationFase1 {
                seed: Seed(0),
                soil: Soil(0),
                fertilizer: Fertilizer(0),
                water: Water(0),
                light: Light(0),
                temperature: Temperature(0),
                humidity: Humidity(0),
                location: Location(u64::MAX),
            };
            for seed in start.0..start.0 + range.0 {
                let soil = SoilMapper::map(&input.soil_mappers, &Seed(seed));
                let fertilizer = FertilizerMapper::map(&input.fertilizer_mappers, &soil);
                let water = WaterMapper::map(&input.water_mappers, &fertilizer);
                let light = LightMapper::map(&input.light_mappers, &water);
                let temperature = TemperatureMapper::map(&input.temperature_mappers, &light);
                let humidity = HumidityMapper::map(&input.humidity_mappers, &temperature);
                let location = LocationMapper::map(&input.location_mappers, &humidity);
                if location < min.location {
                    min = DecodedInfomationFase1 {
                        seed: Seed(seed),
                        soil,
                        fertilizer,
                        water,
                        light,
                        temperature,
                        humidity,
                        location,
                    };
                }
            }
            min
        })
        .collect::<Vec<_>>()
}

#[derive(Debug, Clone)]
struct InputDecoded {
    soil_mappers: Vec<SoilMapper>,
    fertilizer_mappers: Vec<FertilizerMapper>,
    water_mappers: Vec<WaterMapper>,
    light_mappers: Vec<LightMapper>,
    temperature_mappers: Vec<TemperatureMapper>,
    humidity_mappers: Vec<HumidityMapper>,
    location_mappers: Vec<LocationMapper>,
    seeds: Vec<Seed>,
    seeds_pairs: Vec<(Seed, Seed)>,
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    let input = decode_input(&input);
    let result = calculate(&input);
    let min = result.iter().map(|a| a.location).min().unwrap();
    println!("result: {:#?}", min);
    let fase2 = calculate_ver2(&input);
    let min = fase2.iter().map(|a| a.location).min().unwrap();
    println!("result: {:#?}", min);
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_map() {
        let maps: Vec<SoilMapper> = vec![SoilMapper::new(50, 98, 2), SoilMapper::new(52, 50, 48)];
        assert_eq!(SoilMapper::map(&maps, &Seed(0)), Soil(0));
        assert_eq!(SoilMapper::map(&maps, &Seed(49)), Soil(49));
        assert_eq!(SoilMapper::map(&maps, &Seed(50)), Soil(52));
        assert_eq!(SoilMapper::map(&maps, &Seed(51)), Soil(53));
        assert_eq!(SoilMapper::map(&maps, &Seed(96)), Soil(98));
        assert_eq!(SoilMapper::map(&maps, &Seed(97)), Soil(99));
        assert_eq!(SoilMapper::map(&maps, &Seed(98)), Soil(50));
        assert_eq!(SoilMapper::map(&maps, &Seed(99)), Soil(51));
        assert_eq!(SoilMapper::map(&maps, &Seed(79)), Soil(81));
        assert_eq!(SoilMapper::map(&maps, &Seed(14)), Soil(14));
        assert_eq!(SoilMapper::map(&maps, &Seed(55)), Soil(57));
        assert_eq!(SoilMapper::map(&maps, &Seed(13)), Soil(13));
    }

    #[test]
    fn test_parse_seed() {
        let input = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48"#;
        let (rest, seeds) = parse_seed(input).unwrap();
        assert_eq!(seeds, vec![Seed(79), Seed(14), Seed(55), Seed(13)]);
        assert_eq!(rest, "\n\nseed-to-soil map:\n50 98 2\n52 50 48");
    }

    #[test]
    fn test_parse_maps() {
        let input = "50 98 2\n52 50 48";
        assert_eq!(
            parse_maps::<SoilMapper>(input).unwrap(),
            (
                "",
                vec![SoilMapper::new(50, 98, 2), SoilMapper::new(52, 50, 48)]
            )
        );
    }
}
