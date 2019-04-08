extern crate grpcio;
extern crate protos;
extern crate rand;
// use std::env;
use std::sync::Arc;
use std::thread;
// use std::io;
use rand::Rng;
use grpcio::{ChannelBuilder, EnvBuilder};

use protos::mykv::{Order,Data};
use protos::mykv_grpc::MykvClient;
// use protos::lib::*;
use std::time::SystemTime;

pub struct Client{
    pub client:MykvClient,
} 
impl Client{
    pub fn new(host:String,port:u16)->Client{
        let env = Arc::new(EnvBuilder::new().build());
        let ch = ChannelBuilder::new(env).connect(format!("{}:{}",host,port).as_str());
        let client = MykvClient::new(ch);
        Client{
            client,
        }
    }
    pub fn set(&self,key:String,value:String)->Option<Data> {
        let mut order = Order::new();
        order.set_com(String::from("set"));
        order.set_key(key);
        order.set_value(value);
        let data = self.client.say(&order);
        match data {
            Ok(ret) =>{
                Some(ret)
            }
            Err(err) =>{
                println!("set error:{:?}", err);
                None
            }
        }
    }
    pub fn get(&self,key:String)->Option<Data> {
        let mut order = Order::new();
        order.set_com(String::from("get"));
        order.set_key(key);
        let data = self.client.say(&order);
        match data {
            Ok(ret) =>{
                Some(ret)
            }
            Err(err) =>{
                println!("get error:{:?}", err);
                None
            }
        }
    }
    pub fn del(&self,key:String)->Option<Data>{
        let mut order = Order::new();
        order.set_com(String::from("del"));
        order.set_key(key);
        let data = self.client.say(&order);
        match data {
            Ok(ret) =>{
                Some(ret)
            }
            Err(err) =>{
                println!("del error:{:?}", err);
                None
            }
        }
    }
    pub fn scan(&self,key_left:String,key_right:String)->Option<Data>{
        let mut order = Order::new();
        order.set_com(String::from("scan"));
        order.set_key(key_left);
        order.set_value(key_right);
        let data = self.client.say(&order);
        match data {
            Ok(ret) =>{
                Some(ret)
            }
            Err(err) =>{
                println!("scan error:{:?}", err);
                None
            }
        }
    }
    pub fn save(&self)->Option<Data>{
        let mut order = Order::new();
        order.set_com(String::from("save"));
        let data = self.client.say(&order);
        match data {
            Ok(ret) =>{
                Some(ret)
            }
            Err(err) =>{
                println!("save error:{:?}", err);
                None
            }
        }
    }
}
static THREAD_NUM:u16 = 500;
static SET_NUM:u16=20;
fn main() {
    let mut handles = Vec::new();
    let start = SystemTime::now();
    for _ in 0..THREAD_NUM{
        let handle = thread::spawn(move || {
            let mut rng = rand::thread_rng();
            let client = Client::new(String::from("localhost"),45251);
            for _ in 0..SET_NUM{
                let ret = client.set(format!("{}",rng.gen::<u64>()),format!("{}",rng.gen::<u64>()));
                match ret{
                        Some(_) => {
                            // println!("reply : {:?}", ret);
                        }
                        None =>{
                        }
                    }
            }
        });
        handles.push(handle);
    }
    for handle in handles{
        handle.join().unwrap();
    }
    let end = SystemTime::now();
    let cost = start.elapsed().unwrap();
    println!("cost time:{:?}", cost);

    // input ord by youself
    // let thread1 = 
    // thread::spawn(move || {
    //     let client = Client::new(String::from("localhost"),45251);
    //     loop {
    //         let mut com = String::new();
    //         let mut key = String::new();
    //         let mut value = String::new();
    //         println!("input command:");
    //         io::stdin().read_line(&mut com)
    //         .expect("Failed to read line");
    //         match com.trim() {
    //             "set" => {
    //                 println!("input key:");
    //                 io::stdin().read_line(&mut key)
    //                 .expect("Failed to read line");
    //                 println!("input value:");
    //                 io::stdin().read_line(&mut value)
    //                 .expect("Failed to read line");
    //                 let ret = client.set(key, value);
    //                 match ret{
    //                     Some(ret) => {
    //                         println!("reply : {:?}", ret);
    //                     }
    //                     None =>{
    //                     }
    //                 }
    //             }
    //             "get" => {
    //                 println!("input key:");
    //                 io::stdin().read_line(&mut key)
    //                 .expect("Failed to read line");
    //                 let ret = client.del(key);
    //                 match ret{
    //                     Some(ret) => {
    //                         println!("reply : {:?}", ret);
    //                     }
    //                     None =>{
    //                     }
    //                 }
    //             }
    //             "del" => {
    //                 println!("input key:");
    //                 io::stdin().read_line(&mut key)
    //                 .expect("Failed to read line");
    //                 let ret = client.del(key);
    //                 match ret{
    //                     Some(ret) => {
    //                         println!("reply : {:?}", ret);
    //                     }
    //                     None =>{
    //                     }
    //                 }
    //             }
    //             "scan" => {
    //                 println!("input key left:");
    //                 io::stdin().read_line(&mut key)
    //                 .expect("Failed to read line");
    //                 println!("input key right:");
    //                 io::stdin().read_line(&mut value)
    //                 .expect("Failed to read line");
    //                 let ret = client.scan(key, value);
    //                 match ret{
    //                     Some(ret) => {
    //                         let len = ret.get_key().len();
    //                         let mut i = 0;
    //                         while i<len {
    //                             println!("key:{}",&ret.get_key()[i]);
    //                             println!("value:{}",&ret.get_value()[i]);
    //                             i +=1;
    //                         }
    //                     }
    //                     None =>{

    //                     }
    //                 }
                    
    //             }
    //             "save" =>{
    //                 let ret = client.save();
    //                 match ret{
    //                     Some(ret) => {
    //                         println!("reply : {:?}", ret);
    //                     }
    //                     None =>{
    //                     }
    //                 }
    //             }
    //             _ => {
    //             }
    //         }    
    //     }
    // });

    

}