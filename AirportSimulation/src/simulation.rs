
use std::fs::{File};
use std::io::{BufReader, BufRead};
use std::path::{Path};
use rand::Rng;
use std::borrow::Borrow;
use std::ops::Deref;

pub struct AirportSimulation {
    arrival_q : Vec<Plane>,
    depart_q : Vec<Plane>,
    timestamp : u32,
    simulation_file : String,
    config : SimulationConfig,
    summary : SimulationSummary,
    runways : Vec<u32>, // 0 = open, >0 = occupied
}

impl AirportSimulation {
    pub fn new(simulation_file : &str, runway_count : usize) -> Self{
        AirportSimulation{
            arrival_q : Vec::new(),
            depart_q : Vec::new(),
            timestamp : 0,
            simulation_file : simulation_file.parse().unwrap(),
            config : AirportSimulation::read_config(&simulation_file ),
            summary : SimulationSummary {
                count_departed : 0,
                ave_departure_wait : 0.0,
                count_landed : 0,
                ave_land_wait : 0.0,
                count_crashed : 0,
                depart_waits : Vec::new(),
                land_waits : Vec::new(),
            },
            runways : vec![0;runway_count],
        }
    }


    /// Read configuration data from `path` and return a new SimulationConfig
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

    /// Run simulation for the number of ticks specified in the simulation configuration
    pub fn run_simulation(&mut self){
        println!("Running simulation for {} with {} runway(s)", self.simulation_file, self.runways.len());
        while self.timestamp < self.config.simulation_time{
            // if self.timestamp % 1000 == 0{
            //     println!("Current timestamp: {}", self.timestamp);
            // }
            self.tick();
            self.timestamp += 1;
        }
        self.summary.print_summary();
    }

    fn tick(&mut self){
        // during each tick, must determine:
        // - if any planes crashed
        self.determine_crashed();
        // - which runways are free
        // - - which planes can land or depart
        self.land_planes();
        self.depart_planes();
        // - whether to add any planes to depart/land queues
        self.add_to_queues();
        self.decrement_runways();
    }

    /// Determine if any planes have crashed, remove them from the arrival_q and document, return local count crashed
    fn determine_crashed(&mut self) -> u32{
        // Iterate through each plane and determine if they crashed
        let mut count : u32 = 0;
        for p in self.arrival_q.iter_mut(){
            if p.check_crash(self.config.reserve_fuel, self.timestamp){
                self.summary.count_crashed += 1;
                count += 1;
            }
        }
        // reassign arrival queue to one without crashed planes
        self.arrival_q = self.arrival_q.iter().filter(|plane| plane.in_q).cloned().collect();
        count
    }

    /// Add Planes to arrival and departure queues based on configuration probabilities, return count added
    fn add_to_queues(&mut self) -> u32{
        let mut count : u32 = 0;
        // Arrivals
        if rand::thread_rng().gen_ratio((self.config.arrival_rate * 1000.0) as u32,1000){
            self.arrival_q.push(Plane{
                enter_q : self.timestamp,
                in_q : true
            });
            count += 1;
        }
        // Departures
        if rand::thread_rng().gen_ratio((self.config.departure_rate * 1000.0) as u32, 1000){
            self.depart_q.push(Plane{
                enter_q : self.timestamp,
                in_q : true
            });
            count += 1;
        }
        count
    }

    /// determine how many planes can be departed based on open runways, return count departed
    fn depart_planes(&mut self) -> u32{
        let mut ct : u32 = 0;
        for r in self.runways.iter_mut(){
            // check if runway is open
            if *r == 0 && self.depart_q.len() > 0{
                // remove plane from arrival queue
                let p: Plane = self.depart_q.remove(0);
                // save wait time
                self.summary.depart_waits.push(self.timestamp - p.enter_q);
                // add to cooldown
                *r = self.config.takeoff_duration;
                // add to summary
                self.summary.count_departed += 1;
                ct += 1;
            }
        }
        ct
    }

    /// determine how many planes can be landed based on open runways, return count landed
    fn land_planes(&mut self) -> u32{
        let mut ct : u32 = 0;
        for r in self.runways.iter_mut(){
            // check if runway is open
            if *r == 0 && self.arrival_q.len() > 0{
                // remove plane from arrival queue
                let p: Plane = self.arrival_q.remove(0);
                // save wait time
                self.summary.land_waits.push(self.timestamp - p.enter_q);
                // add to cooldown
                *r += self.config.landing_duration;
                // add to summary
                self.summary.count_landed += 1;
                ct += 1;
            }
        }
        ct
    }

    /// decrement the cooldown of each runway by one
    fn decrement_runways(&mut self){
        for r in self.runways.iter_mut(){
            if *r > 0{
                *r -= 1;
            }
        }
    }
}


struct Plane {
    enter_q : u32,
    in_q : bool,
}

impl Plane {
    pub fn new() -> Self{
        Plane{
            enter_q: 0,
            in_q: true,
        }
    }

    /// Return whether the plane has been in the queue longer than it has fuel
    pub fn check_crash(&mut self, fuel : u32, timestamp: u32) -> bool{
        if (timestamp - self.enter_q) > fuel{
            self.in_q = false;
            return true;
        }
        false
    }
}

impl Clone for Plane{
    fn clone(&self) -> Self {
        Plane{
            ..*self
        }
    }
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
    depart_waits : Vec<u32>,
    land_waits : Vec<u32>,
}

impl SimulationSummary{
    /// Calculate averages for departure and arrival waits
    fn calculate_summary(&mut self){
        self.ave_departure_wait = (self.land_waits.iter().sum::<u32>() / (self.land_waits.len() as u32)) as f32;
        self.ave_land_wait = (self.depart_waits.iter().sum::<u32>() / (self.depart_waits.len() as u32)) as f32;
    }

    /// Print simulation summary details
    pub fn print_summary(&mut self){
        self.calculate_summary();
        println!("--------------------------------");
        println!("Number of planes departed: {}", self.count_departed);
        println!("Average wait time for departing planes: {}", self.ave_departure_wait);
        println!("Number of planes landed: {}", self.count_landed);
        println!("Average wait time for landing planes {}", self.ave_land_wait);
        println!("Number of planes crashed: {}", self.count_crashed);
        println!("--------------------------------");

    }
}