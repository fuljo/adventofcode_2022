use std::{
    fs::File,
    io::{BufRead, BufReader},
    ops::RangeInclusive,
};

use scanf::sscanf;

// const LINE: isize = 10;
const LINE: isize = 2000000;
// const BOUND: RangeInclusive<isize> = RangeInclusive::new(0, 20);
const BOUND: RangeInclusive<isize> = RangeInclusive::new(0, 4000000);

struct Sensor {
    sensor: (isize, isize),
    beacon: (isize, isize),
    radius: usize,
}

impl Sensor {
    fn new(sensor: (isize, isize), beacon: (isize, isize)) -> Self {
        let radius = sensor.0.abs_diff(beacon.0) + sensor.1.abs_diff(beacon.1);
        Self {
            sensor,
            beacon,
            radius,
        }
    }

    fn outer_circle(&self) -> SensorOuterIter {
        SensorOuterIter::new(self)
    }
}

// iterate over the exterior border of the sensor (radius + 1)
struct SensorOuterIter {
    center: (isize, isize),
    pos: (isize, isize),
}

impl SensorOuterIter {
    fn new(sensor: &Sensor) -> Self {
        Self {
            center: sensor.sensor,
            pos: (
                sensor.sensor.0 + sensor.radius as isize + 1,
                sensor.sensor.1,
            ),
        }
    }
}

impl Iterator for SensorOuterIter {
    type Item = (isize, isize);

    fn next(&mut self) -> Option<Self::Item> {
        let (x, y) = self.pos;
        let (cx, cy) = self.center;

        if x > cx && y <= cy {
            // 1st quadrant
            self.pos = (x - 1, y - 1);
        } else if x <= cx && y < cy {
            // 2nd quadrant
            self.pos = (x - 1, y + 1);
        } else if x < cx && y >= cy {
            // 3rd quadrant
            self.pos = (x + 1, y + 1);
        } else if x >= cx && y > cy {
            // 4th quadrand
            self.pos = (x + 1, y - 1);
            if self.pos.1 == cy {
                return None;
            }
        }

        Some(self.pos)
    }
}

fn main() {
    let f = File::open("input/day15.txt").unwrap();
    let read = BufReader::new(f);
    let lines = read.lines();

    let sensors: Vec<Sensor> = lines
        .map(|line| {
            let line = line.expect("error while reading line");
            let (mut psx, mut psy): (isize, isize) = (0, 0); // sensor
            let (mut pbx, mut pby): (isize, isize) = (0, 0); // beacon
            sscanf!(
                &line,
                "Sensor at x={}, y={}: closest beacon is at x={}, y={}",
                psx,
                psy,
                pbx,
                pby
            )
            .expect("failed to parse");

            Sensor::new((psx, psy), (pbx, pby))
        })
        .collect();

    // determine ranges in the radius of the sensors
    let mut marked_ranges: Vec<RangeInclusive<isize>> = Vec::new();
    for s in sensors.iter() {
        let sensor = s.sensor;
        let radius = s.radius;

        println!(
            "Sensor: {:?}, Beacon: {:?}, Radius: {}",
            s.sensor, s.beacon, s.radius,
        );

        let dist = LINE.abs_diff(sensor.1); // distance sensor-line
        let slack = radius as isize - dist as isize;
        if slack < 0 {
            continue; // no intersection
        }

        let mut range = RangeInclusive::new(sensor.0 - slack, sensor.0 + slack);
        let mut contained = false;

        // fuse this range with any other overlapping ranges
        marked_ranges.retain(|other| {
            if other.contains(range.start()) || other.contains(range.end()) {
                if other.contains(range.start()) && other.contains(range.end()) {
                    // this is contained in other => do nothing
                    contained = true;
                    return true;
                } else {
                    // union of other into this
                    range = RangeInclusive::new(
                        *range.start().min(other.start()),
                        *range.end().max(other.end()),
                    );
                    return false;
                }
            } else if range.contains(other.start()) && range.contains(other.end()) {
                // other is contained in this => remove
                return true;
            }
            false
        });

        if !contained {
            marked_ranges.push(range);
        }
    }

    // count the positions with no beacons
    let mut no_beacon_count: usize = 0;
    let beacons_in_line: Vec<isize> = sensors
        .iter()
        .filter_map(|s| {
            if s.beacon.1 == LINE {
                Some(s.beacon.1)
            } else {
                None
            }
        })
        .collect();
    for range in marked_ranges {
        no_beacon_count += range.end().abs_diff(*range.start()) + 1;
        if beacons_in_line.iter().any(|b| range.contains(b)) {
            no_beacon_count -= 1;
        }
    }

    println!("{}", no_beacon_count);

    // part 2

    let res = sensors.iter().enumerate().find_map(|(i, sensor)| {
        println!("Sensor {}", i);
        sensor.outer_circle().find_map(|(x, y)| {
            if !BOUND.contains(&x) || !BOUND.contains(&y) {
                return None;
            }

            let feasible = sensors.iter().all(|other| {
                let dist = x.abs_diff(other.sensor.0) + y.abs_diff(other.sensor.1);
                dist > other.radius
            });
            if feasible {
                println!("Sensor {}: point {:?} feasible", i, (x, y));
                Some((x, y))
            } else {
                None
            }
        })
    });

    let (x, y) = res.unwrap();
    let tuning_freq = x * 4000000 + y;
    println!("{}", tuning_freq);
}
