use rand::Rng;
use std::{format};
use std::time::{Duration, SystemTime};

use std::collections::VecDeque;
use std::sync::{Arc, Mutex, MutexGuard};
use crossbeam_channel::{Receiver,Sender};
use scheduled_thread_pool::ScheduledThreadPool;

use std::io::{BufRead, BufReader, Seek, SeekFrom, BufWriter, Write};
use std::fs::{File, OpenOptions};
use std::string::String;
use crossbeam::thread;

pub fn tracking_changes(sender:Sender<String>, receiver:Receiver<String>, 
                         stocks_track:[Arc<Mutex<VecDeque<f64>>>; 25], stocks_owned:[Arc<Mutex<f64>>; 25], _sim_size:usize){        
    
    let stock_path = "./stocks/stocks.txt"; 
    let stock_file = OpenOptions::new().read(true).open(stock_path).expect("err: opening file");                
                     
    thread::scope(|scope| {                
        scope.spawn(move |_| loop{
            let mut buffer = BufReader::new(&stock_file);                            
            buffer.seek(SeekFrom::Current(0)).unwrap();
            for line in buffer.lines(){  //comment for bench
                if let Ok(update) = line {                     
                     sender.send(update).expect("err: sender");                    
                }
            }
//             drop(sender);
        });               
                
     const TRCK_SIZE: usize = 10;   
     for _ in 0..5{
        let rxc = receiver.clone();                                                       
        let track_stocks = stocks_track.clone();                         
        let owned_stocks = stocks_owned.clone();                                    
    
        scope.spawn(move |_| {            
            for stock_update in &rxc {        
                                                                                                          
//                println!("{:?}", stock_update);
                                        
                //Split updated stock 
                let split_stock: Vec<&str> = stock_update.split(' ').collect();
                let stock_i: usize = split_stock[0].parse().unwrap();
                let stock_n: String = split_stock[1].parse().unwrap();
                let stock_v: f64 = split_stock[2].parse().unwrap();   
                
                //Save stock updates
                let mut track_stock = track_stocks[stock_i].lock().unwrap();       
                track_stock.push_back(stock_v);   
                            
                //Pass to Buy/Sell                 
                if track_stock.len() > TRCK_SIZE { 
                    track_stock.pop_front();                                
                    let owned_stocks = owned_stocks[stock_i].lock().unwrap();                         
                    buy_sell_algorithm(stock_n, stock_v, track_stock, owned_stocks); 
                }                                                                                                                
            }
        });
    }
        
    }).unwrap();                            
}

pub fn buy_sell_algorithm(stock_n:String, stock_v:f64, 
    track_stock:MutexGuard<VecDeque<f64>>,mut owned_stock:MutexGuard<f64>) {
                
    let stock_vsum: f64 = track_stock.iter().sum();       

    //Calculate average of updated Stock
    let stock_avg = stock_vsum /  track_stock.len() as f64;                                            
                           
    //Buy and Sell based on average and if I owned
    if *owned_stock == 0.0 {
        if stock_avg < -1.0 && stock_avg > -2.0{             
            println!( "   BUYS: {:?}     U: {:.3}   A: {:.3}", stock_n, stock_v, stock_avg); 
            *owned_stock = stock_v; 
        }
    }    
    else if (stock_avg - *owned_stock) > 3.0 {         
        println!("   SELL: {:?}     U: {:.3}   A: {:.3}     H: {:.3}", stock_n, stock_v, stock_avg, *owned_stock); 
        *owned_stock = 0.0; 
    }                                                                  
} 

 pub fn update_stocks(stock_names:[&'static str;25], stock_values:[Arc<Mutex<f64>>; 25],
                   file: Arc<Mutex<File>>, start_time:SystemTime, sim_size:usize){       
    
     let pool = ScheduledThreadPool::new(sim_size);
    
     for _ in 0..sim_size {               
          let stock_values = stock_values.clone();       
          let stock_file = file.clone();       
           
         pool.execute_at_fixed_rate(Duration::from_millis(0), Duration::from_millis(1), move || loop{                        
              let mut rng = rand::thread_rng();                            
                                       
              //Lock & Update a random stock value 
              let stock_num = rng.gen_range(0, 25);                          
              let mut stock_value = stock_values[stock_num].lock().unwrap();                                                                                                                         
              *stock_value += rng.gen_range(-1.0, 1.0);             
             
              //Store stock update
              let mut stock_file_c = stock_file.lock().unwrap();              
              let time_since_start = SystemTime::now().duration_since(start_time).unwrap();        
              let data = format!("{} {} {:?} {:?}", stock_num, stock_names[stock_num], &stock_value, time_since_start);                                         
              save_stock_update(data, &mut stock_file_c).expect("err: updating stock");                                                                                
          });                         
      }        
 }

 pub fn save_stock_update(stock_update:String, 
        file: &mut File) -> file::Result<()> {            
            
     let mut f = BufWriter::new(file);    
     writeln!(f, "{}", stock_update)?;     
     f.flush()?;
     Ok(())
 }