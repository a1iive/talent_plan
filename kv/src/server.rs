extern crate futures;
extern crate grpcio;
extern crate protos;

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::OpenOptions;
use std::fs::File;
use std::io::Read;
use std::sync::Arc;
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

// static Threshold:u32 = 8_000_000;
static THRESHOLD:i32 = 4;

#[derive(Clone)]
struct MykvService{
    kvpair:BTreeMap<Key,Value>,
    setnum:i32,
}
impl MykvService{
    fn new()->MykvService{
        let file = File::open("kvdata.txt");
        match file {
            Ok(f) =>{
                let mut kv = BTreeMap::new();
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
                MykvService{
                    kvpair:kv,
                    setnum:0,
                }
            }
            Err(_) => 
            MykvService{
                kvpair:BTreeMap::new(),
                setnum:0,
            }
        }
        
    }
}
 
impl Mykv for MykvService{
    fn say(&mut self,ctx:RpcContext,order:Order,sink:UnarySink<Data>){
        println!("Received Order {{ {:?} }}", order);
        let mut data = Data::new();
        match order.get_com() {
            "set" => {
                if self.setnum < THRESHOLD{
                    self.kvpair
                    .insert(Key::from_str(order.get_key()),Value::from_str(order.get_value()));
                    // data.set_key("set success".to_string());
                    // data.set_value("set in memory".to_string());
                    data.set_key(RepeatedField::from_vec(vec!["set success".to_string()]));
                    data.set_value(RepeatedField::from_vec(vec!["set in memory".to_string()]));
                    self.setnum += 1;
                }
                else {
                    let file = String::from("kvdata.txt");
                    // let filenum = self.filenum + 1;
                    // let filenum = filenum.to_string();
                    // let file = format!("{}{}.txt",file,filenum);
                    let file = OpenOptions::new()
                                .read(true)
                                .write(true)
                                .create(true)
                                .append(true)
                                .open(file);
                    match file {
                            Ok(mut stream) => {
                                let mut s = String::new();
                                for (key,value) in self.kvpair.iter(){
                                    // let t = format!("{}\n{}\n{}\n",key.to_string()
                                    //         ,value.tag.to_string(),value.to_string());
                                    let t = format!("{}\n{}\n",key.to_string(),value.to_string());  
                                    s = format!("{}{}",s,t);
                                }
                                match stream.write_all(&*s.as_bytes()) {
                                    Ok(_) => {
                                        // self.kvpair.clear();
                                        self.kvpair
                                            .insert(Key::from_str(order.get_key())
                                            ,Value::from_str(order.get_value()));
                                        // data.set_key("set success".to_string());
                                        // data.set_value("set in disk".to_string());
                                        data.set_key(RepeatedField::from_vec(vec!["set   success".to_string()]));
                                        data.set_value(RepeatedField::from_vec(vec!["set in disk".to_string()]));
                                    }
                                    Err(err) => {
                                        // data.set_key("set failed".to_string());
                                        // data.set_value("set in disk err".to_string());
                                        data.set_key(RepeatedField::from_vec(vec!["set failed".to_string()]));
                                        data.set_value(RepeatedField::from_vec(vec!["set in disk err".to_string()]));
                                        println!("{:?}", err);
                                    }

                                }
                            }
                            Err(err) => {
                                // data.set_key("set failed".to_string());
                                // data.set_value("set in disk err".to_string());
                                data.set_key(RepeatedField::from_vec(
                                    vec!["set failed".to_string()]));
                                data.set_value(RepeatedField::from_vec(
                                    vec!["set in disk err".to_string()]));
                                println!("{:?}", err);
                            }
                    }
                    self.setnum = 0;
                    // self.filenum+=1;
                }
            },
            "get" => {
                let k = Key::from_str(order.get_key());
                match self.kvpair.get(&k){
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
                match self.kvpair.get(&k){
                    Some(_)=>{

                        data.set_key(RepeatedField::from_vec(vec!["del success".to_string()]));
                        }
                    None=>{
                        data.set_key(RepeatedField::from_vec(vec!["del nothing".to_string()]));

                    }
                }
            },
            "scan" => {
                let mut key = Vec::new();
                let mut value = Vec::new();
                let left = Key::from_str(order.get_key());
                let right = Key::from_str(order.get_value());
                // for (k,v) in self.kvpair.iter(){
                //     key.push(k.to_string());
                //     value.push(v.to_string());
                // }
                for(&k,&v) in self.kvpair.range((Included(&left), Included(&right))){
                    key.push(k.to_string());
                    value.push(v.to_string());
                }
                data.set_key(RepeatedField::from_vec(key));
                data.set_value(RepeatedField::from_vec(value));
            },
            "save" =>{
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
                            for (key,value) in self.kvpair.iter(){
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

    fn set(&mut self,ctx:RpcContext,com:Datakey,sink:UnarySink<Resu>){
        println!("Received set {{ {:?} }}", com);
        let mut rep = Resu::new();
        self.kvpair
             .insert(Key::from_str(com.get_key()),Value::from_str(com.get_key()));
        rep.set_re(1);
        rep.set_value("set in memory".to_string());

        let f = sink
            .success(rep.clone())
            .map(move |_| println!("Responded with rep {{ {:?} }}", rep))
            .map_err(move |err| eprintln!("Failed to reply: {:?}", err));
        ctx.spawn(f);
    }

    fn get(&mut self,ctx:RpcContext,com:Datakey,sink:UnarySink<Resu>){
        println!("Received get {{ {:?} }}", com);
        println!("{:?}",self.kvpair.len());
        let k = Key::from_str(com.get_key());
        let mut rep = Resu::new();
        match self.kvpair.get(&k){
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

    fn del(&mut self,ctx:RpcContext,com:Datakey,sink:UnarySink<Resu>){
        println!("Received del {{ {:?} }}", com);
        let k = Key::from_str(com.get_key());
        let mut rep = Resu::new();
        match self.kvpair.remove(&k){
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
    let env = Arc::new(Environment::new(2));
    let instance = MykvService::new();
    let service = mykv_grpc::create_mykv(instance);
    let mut server = ServerBuilder::new(env)
        .register_service(service)
        .bind("127.0.0.1",0)
        .bind("127.0.0.1",0) //add multiple ports. 0 means random
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
