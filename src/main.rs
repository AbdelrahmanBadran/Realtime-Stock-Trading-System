use new_rust_simulation::{update_stocks, tracking_changes};

extern crate rand;
extern crate file;
extern crate crossbeam_channel;
extern crate indexed_line_reader;
extern crate scheduled_thread_pool;
extern crate  crossbeam;

use std::fs::{OpenOptions};
use std::time::{SystemTime};
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;
use crossbeam_channel::{bounded};

 fn main(){
    let start_time = SystemTime::now();
    const SIM_SIZE: usize = 25;                          
    let stock_path = "./stocks/stocks.txt";
 
    let file = Arc::new(Mutex::new(OpenOptions::new().write(true)
                .truncate(true).create(true).open(stock_path).expect("err: file")));                         
    let stock_values: [Arc<Mutex<f64>>; SIM_SIZE] = Default::default();                   
    let stock_names = ["AAP","MSF","IBM", "ORC","FCB","GOL","XRX","STX","NOR","CAD",
                        "AMD","INT","SAS","ABA","TEN","UBR","GRB", "VER","AEG","AIA", 
                        "AIG","ANT", "ATT", "BTT", "CCC"];                                                   
    update_stocks(stock_names, stock_values, file, start_time, SIM_SIZE);                           
      
    let (sender, receiver) = bounded(SIM_SIZE);    
    let stocks_track: [Arc<Mutex<VecDeque<f64>>>; SIM_SIZE] = Default::default();        
    let stocks_owned: [Arc<Mutex<f64>>; SIM_SIZE] = Default::default();            
    tracking_changes(sender, receiver, stocks_track, stocks_owned, SIM_SIZE);
                                                                                        
    println!("{:?}", SystemTime::now().duration_since(start_time).unwrap());     
}