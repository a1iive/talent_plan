// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

const METHOD_MYKV_SAY: ::grpcio::Method<super::mykv::Order, super::mykv::Data> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/kv.Mykv/Say",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_MYKV_SET: ::grpcio::Method<super::mykv::Datakey, super::mykv::Resu> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/kv.Mykv/Set",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_MYKV_GET: ::grpcio::Method<super::mykv::Datakey, super::mykv::Resu> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/kv.Mykv/Get",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_MYKV_DEL: ::grpcio::Method<super::mykv::Datakey, super::mykv::Resu> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/kv.Mykv/Del",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_MYKV_SCAN: ::grpcio::Method<super::mykv::Empty, super::mykv::Data> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/kv.Mykv/Scan",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct MykvClient {
    client: ::grpcio::Client,
}

impl MykvClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        MykvClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn say_opt(&self, req: &super::mykv::Order, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::mykv::Data> {
        self.client.unary_call(&METHOD_MYKV_SAY, req, opt)
    }

    pub fn say(&self, req: &super::mykv::Order) -> ::grpcio::Result<super::mykv::Data> {
        self.say_opt(req, ::grpcio::CallOption::default())
    }

    pub fn say_async_opt(&self, req: &super::mykv::Order, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::mykv::Data>> {
        self.client.unary_call_async(&METHOD_MYKV_SAY, req, opt)
    }

    pub fn say_async(&self, req: &super::mykv::Order) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::mykv::Data>> {
        self.say_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn set_opt(&self, req: &super::mykv::Datakey, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::mykv::Resu> {
        self.client.unary_call(&METHOD_MYKV_SET, req, opt)
    }

    pub fn set(&self, req: &super::mykv::Datakey) -> ::grpcio::Result<super::mykv::Resu> {
        self.set_opt(req, ::grpcio::CallOption::default())
    }

    pub fn set_async_opt(&self, req: &super::mykv::Datakey, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::mykv::Resu>> {
        self.client.unary_call_async(&METHOD_MYKV_SET, req, opt)
    }

    pub fn set_async(&self, req: &super::mykv::Datakey) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::mykv::Resu>> {
        self.set_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_opt(&self, req: &super::mykv::Datakey, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::mykv::Resu> {
        self.client.unary_call(&METHOD_MYKV_GET, req, opt)
    }

    pub fn get(&self, req: &super::mykv::Datakey) -> ::grpcio::Result<super::mykv::Resu> {
        self.get_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_async_opt(&self, req: &super::mykv::Datakey, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::mykv::Resu>> {
        self.client.unary_call_async(&METHOD_MYKV_GET, req, opt)
    }

    pub fn get_async(&self, req: &super::mykv::Datakey) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::mykv::Resu>> {
        self.get_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn del_opt(&self, req: &super::mykv::Datakey, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::mykv::Resu> {
        self.client.unary_call(&METHOD_MYKV_DEL, req, opt)
    }

    pub fn del(&self, req: &super::mykv::Datakey) -> ::grpcio::Result<super::mykv::Resu> {
        self.del_opt(req, ::grpcio::CallOption::default())
    }

    pub fn del_async_opt(&self, req: &super::mykv::Datakey, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::mykv::Resu>> {
        self.client.unary_call_async(&METHOD_MYKV_DEL, req, opt)
    }

    pub fn del_async(&self, req: &super::mykv::Datakey) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::mykv::Resu>> {
        self.del_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn scan_opt(&self, req: &super::mykv::Empty, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::mykv::Data>> {
        self.client.server_streaming(&METHOD_MYKV_SCAN, req, opt)
    }

    pub fn scan(&self, req: &super::mykv::Empty) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::mykv::Data>> {
        self.scan_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait Mykv {
    fn say(&mut self, ctx: ::grpcio::RpcContext, req: super::mykv::Order, sink: ::grpcio::UnarySink<super::mykv::Data>);
    fn set(&mut self, ctx: ::grpcio::RpcContext, req: super::mykv::Datakey, sink: ::grpcio::UnarySink<super::mykv::Resu>);
    fn get(&mut self, ctx: ::grpcio::RpcContext, req: super::mykv::Datakey, sink: ::grpcio::UnarySink<super::mykv::Resu>);
    fn del(&mut self, ctx: ::grpcio::RpcContext, req: super::mykv::Datakey, sink: ::grpcio::UnarySink<super::mykv::Resu>);
    fn scan(&mut self, ctx: ::grpcio::RpcContext, req: super::mykv::Empty, sink: ::grpcio::ServerStreamingSink<super::mykv::Data>);
}

pub fn create_mykv<S: Mykv + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_MYKV_SAY, move |ctx, req, resp| {
        instance.say(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_MYKV_SET, move |ctx, req, resp| {
        instance.set(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_MYKV_GET, move |ctx, req, resp| {
        instance.get(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_MYKV_DEL, move |ctx, req, resp| {
        instance.del(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_server_streaming_handler(&METHOD_MYKV_SCAN, move |ctx, req, resp| {
        instance.scan(ctx, req, resp)
    });
    builder.build()
}
