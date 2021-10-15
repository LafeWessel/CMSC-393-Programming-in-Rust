
use std::fs::{File};
use std::io::{BufReader, BufRead};
use std::path::{Path};

pub struct AirportSimulation {
    arrival_q : Vec<Plane>,
    depart_q : Vec<Plane>,
    timestamp : u32,
    simulation_file : String,
    config : SimulationConfig,
    summary : SimulationSummary,
}

impl AirportSimulation {
    pub fn new(simulation_file : &str) -> Self{
        let config = AirportSimulation::read_config(&simulation_file );
        AirportSimulation{
            arrival_q : Vec::new(),
            depart_q : Vec::new(),
            timestamp : 0,
            simulation_file: simulation_file.parse().unwrap(),
            config,
            summary : SimulationSummary {
                count_departed : 0,
                ave_departure_wait: 0.0,
                count_landed: 0,
                ave_land_wait: 0.0,
                count_crashed: 0
            }
        }
    }

    fn read_config(path : &str) -> SimulationConfig {
        let file = std::fs::read_to_string(path).expect(&format!("Unable to open file {:?}",path));
        let lines : Vec<&str> = file.lines().collect();

        //println!("Lines: {:?}", lines);

        SimulationConfig{
            takeoff_duration : lines[0].parse().unwrap(),
            landing_duration: lines[1].parse().unwrap(),
            departure_rate: lines[2].parse().unwrap(),
            arrival_rate: lines[3].parse().unwrap(),
            reserve_fuel: lines[4].parse().unwrap(),
            simulation_time: lines[5].parse().unwrap(),
            description: lines[6].parse().unwrap(),
        }
    }

}





struct Plane {
    enter_q : u32,
    exit_q : u32,
    in_q : bool,
}

struct SimulationConfig {
    takeoff_duration : u32,
    landing_duration : u32,
    departure_rate : f32,
    arrival_rate : f32,
    reserve_fuel : u32,
    simulation_time : u32,
    description : String,
}

struct SimulationSummary {
    count_departed : u32,
    ave_departure_wait : f32,
    count_landed : u32,
    ave_land_wait : f32,
    count_crashed : u32,
}