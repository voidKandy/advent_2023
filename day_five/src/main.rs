use std::{collections::VecDeque, fs};

fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();
    let map = EntireMap::from(input.as_str());
    println!("{}", map.lowest_location());
}

type AlmanacMap = (u64, u64, u64);

#[derive(Debug, Clone)]
struct AlmanacRanges {
    destination_range: Vec<u64>,
    source_range: Vec<u64>,
}

#[derive(Debug, Clone)]
struct DestinationSourceMap {
    destination: u64,
    source: u64,
}

#[derive(Debug, Clone)]
struct DestinationSourceMapVec(Vec<DestinationSourceMap>);

#[derive(Debug, Clone)]
struct EntireMap {
    seeds: Vec<u64>,
    seed_to_soil: DestinationSourceMapVec,
    soil_to_fertilizer: DestinationSourceMapVec,
    fertilizer_to_water: DestinationSourceMapVec,
    water_to_light: DestinationSourceMapVec,
    light_to_temp: DestinationSourceMapVec,
    temp_to_humidity: DestinationSourceMapVec,
    humidity_to_location: DestinationSourceMapVec,
}

fn try_map_from_str(str: &str) -> Option<AlmanacMap> {
    let vec: Vec<u64> = str
        .split_whitespace()
        .filter_map(|s| {
            if !s.trim().is_empty() {
                s.parse().ok()
            } else {
                None
            }
        })
        .collect();
    if vec.len() == 3 {
        Some((vec[0], vec[1], vec[2]))
    } else {
        None
    }
}

impl From<AlmanacMap> for AlmanacRanges {
    fn from(value: AlmanacMap) -> Self {
        let range_iter = (0..value.2).into_iter();
        let destination_range = range_iter.clone().fold(vec![], |mut range, i| {
            range.push(value.0 + i);
            range
        });
        let source_range = range_iter.fold(vec![], |mut range, i| {
            range.push(value.1 + i);
            range
        });
        AlmanacRanges {
            destination_range,
            source_range,
        }
    }
}

impl TryFrom<Vec<AlmanacRanges>> for DestinationSourceMapVec {
    type Error = Box<dyn std::error::Error>;
    fn try_from(value: Vec<AlmanacRanges>) -> Result<Self, Self::Error> {
        let return_vec = value.into_iter().fold(vec![], |mut outer_mapvec, r| {
            outer_mapvec.append(r.destination_range.into_iter().enumerate().fold(
                &mut vec![],
                |mapvec, (i, d)| {
                    let source_map = DestinationSourceMap {
                        destination: d,
                        source: r.source_range[i],
                    };
                    mapvec.push(source_map);
                    mapvec
                },
            ));
            outer_mapvec
        });
        Ok(Self(return_vec))
    }
}

impl From<&str> for EntireMap {
    fn from(value: &str) -> Self {
        let mut maps: VecDeque<&str> = value.split("\n\n").map(|s| s.trim()).collect();
        let seeds: Vec<u64> = maps
            .pop_front()
            .unwrap()
            .split_once(':')
            .expect("failed to split on :")
            .1
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        println!("{:?}", maps);
        let vec_maps: Vec<Vec<AlmanacMap>> = maps
            .into_iter()
            .map(|m| {
                m.split_once(':')
                    .expect("failed to split on :")
                    .1
                    .lines()
                    .filter_map(|l| try_map_from_str(l))
                    .collect()
            })
            .collect();
        let ranges: VecDeque<Vec<AlmanacRanges>> = vec_maps
            .into_iter()
            .map(|m| {
                m.into_iter()
                    .map(|inner_m| AlmanacRanges::from(inner_m))
                    .collect()
            })
            .collect();
        EntireMap {
            seeds,
            seed_to_soil: DestinationSourceMapVec::try_from(ranges.get(0).unwrap().to_owned())
                .unwrap(),
            soil_to_fertilizer: DestinationSourceMapVec::try_from(
                ranges.get(1).unwrap().to_owned(),
            )
            .unwrap(),
            fertilizer_to_water: DestinationSourceMapVec::try_from(
                ranges.get(2).unwrap().to_owned(),
            )
            .unwrap(),
            water_to_light: DestinationSourceMapVec::try_from(ranges.get(3).unwrap().to_owned())
                .unwrap(),
            light_to_temp: DestinationSourceMapVec::try_from(ranges.get(4).unwrap().to_owned())
                .unwrap(),
            temp_to_humidity: DestinationSourceMapVec::try_from(ranges.get(5).unwrap().to_owned())
                .unwrap(),
            humidity_to_location: DestinationSourceMapVec::try_from(
                ranges.get(6).unwrap().to_owned(),
            )
            .unwrap(),
        }
    }
}

impl EntireMap {
    fn seed_locations(&self) -> Vec<(u64, u64)> {
        self.seeds.iter().fold(vec![], |mut ret_tup, s| {
            let soil = self.seed_to_soil.lookup_source(s.clone());
            let fert = self.soil_to_fertilizer.lookup_source(soil);
            let water = self.fertilizer_to_water.lookup_source(fert);
            let light = self.water_to_light.lookup_source(water);
            let temp = self.light_to_temp.lookup_source(light);
            let hum = self.temp_to_humidity.lookup_source(temp);
            let loc = self.humidity_to_location.lookup_source(hum);
            ret_tup.push((loc, *s));
            ret_tup
        })
    }

    fn lowest_location(&self) -> u64 {
        self.seed_locations()
            .into_iter()
            .min_by_key(|x| x.0)
            .unwrap()
            .0
    }
}

impl DestinationSourceMapVec {
    fn lookup_source(&self, source: u64) -> u64 {
        if let Some(map) = self.0.iter().find(|m| m.source == source) {
            map.destination
        } else {
            source
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::{try_map_from_str, AlmanacMap, AlmanacRanges, DestinationSourceMapVec, EntireMap};

    #[test]
    fn example_seed_to_soil_map() {
        let input = "50 98 2\n52 50 48";
        let maps: Vec<AlmanacMap> = input
            .lines()
            .into_iter()
            .filter_map(|l| try_map_from_str(l))
            .collect();
        let ranges: Vec<AlmanacRanges> = maps.into_iter().map(|m| AlmanacRanges::from(m)).collect();
        assert_eq!(vec![50, 51], ranges[0].destination_range);
        assert_eq!(vec![98, 99], ranges[0].source_range);

        let source_map = DestinationSourceMapVec::try_from(ranges.clone()).unwrap();
        assert_eq!(
            (50, 98),
            (source_map.0[0].destination, source_map.0[0].source)
        );

        let seeds_input = "79 14 55 13";
        let seeds: Vec<u64> = seeds_input
            .split_whitespace()
            .filter_map(|s| {
                if !s.trim().is_empty() {
                    s.parse().ok()
                } else {
                    None
                }
            })
            .collect();

        assert_eq!((79, 81), (seeds[0], source_map.lookup_source(seeds[0])));

        assert_eq!((14, 14), (seeds[1], source_map.lookup_source(seeds[1])));

        assert_eq!((55, 57), (seeds[2], source_map.lookup_source(seeds[2])));

        assert_eq!((13, 13), (seeds[3], source_map.lookup_source(seeds[3])));
    }

    #[test]
    fn example_seed_to_destination_map() {
        let input = "seeds: 79 14 55 13

            seed-to-soil map:
            50 98 2
            52 50 48

            soil-to-fertilizer map:
            0 15 37
            37 52 2
            39 0 15

            fertilizer-to-water map:
            49 53 8
            0 11 42
            42 0 7
            57 7 4

            water-to-light map:
            88 18 7
            18 25 70

            light-to-temperature map:
            45 77 23
            81 45 19
            68 64 13

            temperature-to-humidity map:
            0 69 1
            1 0 69

            humidity-to-location map:
            60 56 37
            56 93 4";

        // let input = std::fs::read_to_string("src/input.txt").unwrap();
        let entire_map: EntireMap = EntireMap::from(input);
        let seed_locs = entire_map.seed_locations();
        assert_eq!(vec![(82, 79), (43, 14), (86, 55), (35, 13)], seed_locs);
        assert_eq!(35, entire_map.lowest_location());
    }
}
