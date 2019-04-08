extern crate grpcio;
extern crate protos;

use std::env;
use std::sync::Arc;
use std::io;

use grpcio::{ChannelBuilder, EnvBuilder};

use protos::mykv::{Order};
use protos::mykv_grpc::MykvClient;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        panic!("Expected exactly four argument, the port number.")
    }
    let port = args[1]
        .parse::<u16>()
        .expect(format!("{} is not a valid port number", args[1]).as_str());

    let env = Arc::new(EnvBuilder::new().build());
    let ch = ChannelBuilder::new(env).connect(format!("localhost:{}", port).as_str());
    let client = MykvClient::new(ch);

    let mut order = Order::new();
    loop {
        let mut com = String::new();
        let mut key = String::new();
        let mut value = String::new();
        println!("input command:");
        io::stdin().read_line(&mut com)
        .expect("Failed to read line");
        match com.trim() {
            "set" => {
                println!("input key:");
                io::stdin().read_line(&mut key)
                .expect("Failed to read line");
                println!("input value:");
                io::stdin().read_line(&mut value)
                .expect("Failed to read line");
                order.set_com(com.trim().to_string());
                order.set_key(key.trim().to_string());
                order.set_value(value.trim().to_string());
            }
            "get" => {
                println!("input key:");
                io::stdin().read_line(&mut key)
                .expect("Failed to read line");
                order.set_com(com.trim().to_string());
                order.set_key(key.trim().to_string());
            }
            "del" => {
                println!("input key:");
                io::stdin().read_line(&mut key)
                .expect("Failed to read line");
                order.set_com(com.trim().to_string());
                order.set_key(key.trim().to_string());
            }
            "scan" => {
                println!("input key left:");
                io::stdin().read_line(&mut key)
                .expect("Failed to read line");
                println!("input key right:");
                io::stdin().read_line(&mut value)
                .expect("Failed to read line");
                order.set_com(com.trim().to_string());
                order.set_key(key.trim().to_string());
                order.set_value(value.trim().to_string());
            }
            "save" =>{
                order.set_com(com.trim().to_string());
            }
            _ => {

            }
        }
        
        let data = client.say(&order).expect("RPC Failed!");
        println!("order : {:?}", order);
        if order.get_com() == "scan" {
            let len = data.get_key().len();
            let mut i = 0;
            while i<len {
                println!("key:{}",&data.get_key()[i]);
                println!("value:{}",&data.get_value()[i]);
                i +=1;
            }
        }
        else {
            println!("reply : {:?}", data);
        }
        
    }

    // loop {
    //     let mut com = String::new();
    //     let mut key = String::new();
    //     let mut value = String::new();
    //     println!("input command:");
    //     io::stdin().read_line(&mut com)
    //     .expect("Failed to read line");
    //     match (com.trim()) {
    //         "set" => {
    //             println!("input key:");
    //             io::stdin().read_line(&mut key)
    //             .expect("Failed to read line");
    //             println!("input value:");
    //             io::stdin().read_line(&mut value)
    //             .expect("Failed to read line");
    //             let mut data = Datakey::new();
    //             data.set_key(key.trim().to_string());
    //             let rep = client.set(&data).expect("RPC Failed!");
    //             println!("set : {:?}", data);
    //             println!("reply : {:?}", rep);
    //         }
    //         "get" => {
    //             println!("input key:");
    //             io::stdin().read_line(&mut key)
    //             .expect("Failed to read line");
    //             let mut datakey = Datakey::new();
    //             datakey.set_key(key.trim().to_string());
    //             let rep = client.get(&datakey).expect("RPC Failed!");
    //             println!("get : {:?}", datakey);
    //             println!("reply : {:?}", rep);
    //         }
    //         "del" => {

    //         }
    //         "scan" => {

    //         }
    //         _ =>{

    //         }
    //         }
    //     }
    
}