use criterion::{criterion_group, criterion_main, Criterion}; //black_box
use new_rust_simulation::{tracking_changes};

extern crate rand;
extern crate file;
extern crate crossbeam_channel;
extern crate indexed_line_reader;
extern crate scheduled_thread_pool;

use std::time::{SystemTime};
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;
use crossbeam_channel::{bounded};
//use std::time::{Instant};

fn criterion_benchmark(c: &mut Criterion) {    
    c.bench_function("tracking_changes", move |b| {       
         b.iter_custom(|_iters| {   
            
            const SIM_SIZE: usize = 25;                                                                                                
            let (sender, receiver) = bounded(SIM_SIZE);                
            
            let stocks_track: [Arc<Mutex<VecDeque<f64>>>; SIM_SIZE] 
                            = Default::default();        
            let stocks_owned: [Arc<Mutex<f64>>; SIM_SIZE] 
                            = Default::default();    
                                  
            let start_time = SystemTime::now();    
            tracking_changes(sender, receiver, stocks_track, 
                            stocks_owned, SIM_SIZE);                                                                  
            SystemTime::now().duration_since(start_time).unwrap()                  
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);