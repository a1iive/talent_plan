extern crate futures;
extern crate grpcio;
extern crate protos;

use std::io::prelude::*;
use std::io::BufReader;
use std::fs;
use std::fs::OpenOptions;
use std::fs::File;
use std::io::Read;
use std::sync::{Arc,RwLock};
use std::{io, thread};
use std::collections::BTreeMap;
use std::ops::Bound::Included;

use futures::sync::oneshot;
use futures::Future;
use grpcio::*;

use protobuf::RepeatedField;
use protos::mykv::{Data, Resu,Order,Empty,Datakey};
use protos::mykv_grpc::{self, Mykv};
use protos::lib::*;

static THRESHOLD:usize = 1;

#[derive(Clone)]
struct MykvService{
    kvpair:Arc<RwLock<BTreeMap<Key,Value>>>,
    log:Arc<RwLock<Vec<(String,Key,Value)>>>,
}
impl MykvService{
    fn new()->MykvService{
        let file = File::open("kvdata.txt");
        let logfile = File::open("log.log");
        let mut kv = BTreeMap::new();
        match file {
            Ok(f) =>{
                let f = BufReader::new(f);
                let mut key = String::new();
                let mut value = String::new();
                let mut l=0;
                for line in f.lines() {
                    if let Ok(line) = line {
                        if l==0 {
                            key = line;
                        }
                        else {
                            value = line;
                            kv.insert(Key::from_str(&*key),Value::from_str(&*value));
                        }
                    }
                    l = (l+1)%2;
                }
            }
            Err(_) => {
            } 
        }
        match logfile {
            Ok(f) =>{
                let f = BufReader::new(f);
                let mut op = String::new();
                let mut key = String::new();
                let mut value = String::new();
                let mut l=0;
                for line in f.lines() {
                    if let Ok(line) = line {
                        if l==0 {
                            op = line;
                        }
                        else if l==1{
                            key = line;
                        }
                        else {
                            value = line;
                            if op.trim() == "set"{
                                kv.insert(Key::from_str(&*key),Value::from_str(&*value));
                            }
                            else {
                                kv.remove(&Key::from_str(&*key));
                            }
                        }
                    }
                    l = (l+1)%3;
                }
            }
            Err(_) =>{

            }
        }
        // fs::remove_file("log.log").unwrap_or_else(|why| {
        //     println!("! {:?}", why.kind());
        // });
        MykvService{
            kvpair:Arc::new(RwLock::new(kv)),
            log:Arc::new(RwLock::new(Vec::new())),
        }
    }
}

impl Mykv for MykvService{
    fn say(&mut self,ctx:RpcContext,order:Order,sink:UnarySink<Data>){
        println!("Received Order {{ {:?} }}", order);
        let mut data = Data::new();
        match order.get_com() {
            "set" => {
                let mut kvpair = self.kvpair.write().unwrap();
                let mut log = self.log.write().unwrap();
                let ret = kvpair
                    .insert(Key::from_str(order.get_key()),Value::from_str(order.get_value()));
                match ret {
                    Some(_) => {
                        data.set_key(RepeatedField::from_vec(vec!["overwrite set".to_string()]));
                    }
                    None =>{
                        data.set_key(RepeatedField::from_vec(vec!["normal set".to_string()]));
                    }
                }
                data.set_value(RepeatedField::from_vec(vec!["set in memory".to_string()]));
                // data.set_key("set success".to_string());
                // data.set_value("set in memory".to_string());
                log.push((String::from("set"),Key::from_str(order.get_key()),Value::from_str(order.get_value())));
                let th = THRESHOLD;
                if log.len()>=th{
                    let file = String::from("log.log");
                    let file = OpenOptions::new()
                                .read(true)
                                .write(true)
                                .create(true)
                                .append(true)
                                .open(file);
                    match file {
                        Ok(mut stream) =>{
                            let mut s = String::new();
                            for (op,k,v) in log.iter() {
                                let t = format!("{}\n{}\n{}\n",op,k.to_string(),v.to_string());  
                                s = format!("{}{}",s,t);
                            }
                            match stream.write_all(&*s.as_bytes()) {
                                Ok(_) => {
                                    println!("log saved ok");
                                    log.clear();
                                }
                                Err(err) => {
                                    println!("log save error:{:?}", err);
                                }
                            }
                        }
                        Err(err) =>{
                            println!("log save error:{:?}",err );
                        }
                    }
                } 
                
            },
            "get" => {
                let k = Key::from_str(order.get_key());
                let kvpair = self.kvpair.read().unwrap();
                match kvpair.get(&k){
                    Some(v)=>{
                            data.set_key(RepeatedField::from_vec(vec!["get success".to_string()]));
                            data.set_value(RepeatedField::from_vec(vec![v.to_string()]));
                        }
                    None=>{
                        data.set_key(RepeatedField::from_vec(vec!["get nothing".to_string()]));
                    }
                }
            },
            "del" => {
                let k = Key::from_str(order.get_key());
                let mut log = self.log.write().unwrap();
                let mut kvpair = self.kvpair.write().unwrap();
                match kvpair.remove(&k){
                    Some(_)=>{
                        data.set_key(RepeatedField::from_vec(vec!["del success".to_string()]));
                        }
                    None=>{
                        data.set_key(RepeatedField::from_vec(vec!["del nothing".to_string()]));

                    }
                }
                log.push((String::from("del"),Key::from_str(order.get_key()),Value::from_str(order.get_value())));
                let th = THRESHOLD;
                if log.len()>=th{
                    let file = String::from("log.log");
                    let file = OpenOptions::new()
                                .read(true)
                                .write(true)
                                .create(true)
                                .append(true)
                                .open(file);
                    match file {
                        Ok(mut stream) =>{
                            let mut s = String::new();
                            for (op,k,v) in log.iter() {
                                let t = format!("{}\n{}\n{}\n",op,k.to_string(),v.to_string());  
                                s = format!("{}{}",s,t);
                            }
                            match stream.write_all(&*s.as_bytes()) {
                                Ok(_) => {
                                    println!("log saved ok");
                                    log.clear();
                                }
                                Err(err) => {
                                    println!("log save error:{:?}", err);
                                }
                            }
                        }
                        Err(err) =>{
                            println!("log save error:{:?}",err );
                        }
                    }
                } 
                
            },
            "scan" => {
                let mut key = Vec::new();
                let mut value = Vec::new();
                let kvpair = self.kvpair.read().unwrap();
                let left = Key::from_str(order.get_key());
                let right = Key::from_str(order.get_value());
                // for (k,v) in self.kvpair.iter(){
                //     key.push(k.to_string());
                //     value.push(v.to_string());
                // }
                for(&k,&v) in kvpair.range((Included(&left), Included(&right))){
                    key.push(k.to_string());
                    value.push(v.to_string());
                }
                data.set_key(RepeatedField::from_vec(key));
                data.set_value(RepeatedField::from_vec(value));
            },
            "save" =>{
                let kvpair = self.kvpair.write().unwrap();
                fs::copy("log.log","log_bak.log").unwrap();
                fs::remove_file("log.log").unwrap_or_else(|why| {
                    println!("! {:?}", why.kind());
                });
                let file = String::from("kvdata.txt");
                let file = OpenOptions::new()
                            .read(true)
                            .write(true)
                            .create(true)
                            .append(true)
                            .open(file);
                match file {
                        Ok(mut stream) => {

                            let mut s = String::new();
                            for (key,value) in kvpair.iter(){
                                let t = format!("{}\n{}\n",key.to_string(),value.to_string());  
                                s = format!("{}{}",s,t);
                            }
                            match stream.write_all(&*s.as_bytes()) {
                                Ok(_) => {
                                    data.set_key(RepeatedField::from_vec(vec!["save   success".to_string()]));
                                }
                                Err(err) => {
                                    data.set_key(RepeatedField::from_vec(vec!["save failed".to_string()]));
                                    println!("{:?}", err);
                                }

                            }
                        }
                        Err(err) => {
                            data.set_key(RepeatedField::from_vec(
                                vec!["save failed".to_string()]));
                            println!("{:?}", err);
                        }
                }
            },
            _  => {

            }
        }
        let f = sink
            .success(data.clone())
            .map(move |_| println!("Responded with Data {{ {:?} }}", data))
            .map_err(move |err| eprintln!("Failed to reply: {:?}", err));
        ctx.spawn(f);
    }
    // no use
    fn set(&mut self,ctx:RpcContext,com:Datakey,sink:UnarySink<Resu>){
        println!("Received set {{ {:?} }}", com);
        let mut rep = Resu::new();
        let mut kvpair = self.kvpair.write().unwrap();
        kvpair.insert(Key::from_str(com.get_key()),Value::from_str(com.get_key()));
        rep.set_re(1);
        rep.set_value("set in memory".to_string());

        let f = sink
            .success(rep.clone())
            .map(move |_| println!("Responded with rep {{ {:?} }}", rep))
            .map_err(move |err| eprintln!("Failed to reply: {:?}", err));
        ctx.spawn(f);
    }
    //no use
    fn get(&mut self,ctx:RpcContext,com:Datakey,sink:UnarySink<Resu>){
        println!("Received get {{ {:?} }}", com);
        let kvpair = self.kvpair.write().unwrap();
        let k = Key::from_str(com.get_key());
        let mut rep = Resu::new();
        match kvpair.get(&k){
            Some(v)=>{
                rep.set_re(1);
                rep.set_value(v.to_string());
                println!("get success");
                }
            None=>{
                rep.set_re(1);
                rep.set_value("none".to_string());
                println!("get nothing");
            }
        }
        let f = sink
            .success(rep.clone())
            .map(move |_| println!("Responded with rep {{ {:?} }}", rep))
            .map_err(move |err| eprintln!("Failed to reply: {:?}", err));
        ctx.spawn(f);
    }
    // no use
    fn del(&mut self,ctx:RpcContext,com:Datakey,sink:UnarySink<Resu>){
        println!("Received del {{ {:?} }}", com);
        let mut kvpair = self.kvpair.write().unwrap();
        let k = Key::from_str(com.get_key());
        let mut rep = Resu::new();
        match kvpair.remove(&k){
            Some(v)=>{
                rep.set_re(1);
                rep.set_value(v.to_string());
                }
            None=>{
                rep.set_re(0);
                rep.set_value("none".to_string());
            }
        }
        let f = sink
            .success(rep.clone())
            .map(move |_| println!("Responded with rep {{ {:?} }}", rep))
            .map_err(move |err| eprintln!("Failed to reply: {:?}", err));
        ctx.spawn(f);
    }

    fn scan(&mut self,_ctx:RpcContext,_com:Empty,_sink:ServerStreamingSink<Data>){

    }

}


fn main() {
    println!("kv server !");
    let env = Arc::new(Environment::new(1));
    let instance = MykvService::new();
    let service = mykv_grpc::create_mykv(instance);
    let mut server = ServerBuilder::new(env)
        .register_service(service)
        .bind("127.0.0.1",45251)
        .build()
        .unwrap();
    server.start();
    for &(ref host,port) in server.bind_addrs(){
        println!("listening on {}:{}", host, port);
    }
    let (tx, rx) = oneshot::channel();
    thread::spawn(move || {
        println!("Press ENTER to exit...");
        let _ = io::stdin().read(&mut [0]).unwrap();
        tx.send(())
    });
    let _ = rx.wait();
    let _ = server.shutdown().wait();
}
