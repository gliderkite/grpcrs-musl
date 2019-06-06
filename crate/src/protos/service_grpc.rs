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

const METHOD_RUNNER_CD_RUN: ::grpcio::Method<super::service::Empty, super::service::Empty> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/artichok.RunnerCd/Run",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct RunnerCdClient {
    client: ::grpcio::Client,
}

impl RunnerCdClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        RunnerCdClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn run_opt(&self, req: &super::service::Empty, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::Empty> {
        self.client.unary_call(&METHOD_RUNNER_CD_RUN, req, opt)
    }

    pub fn run(&self, req: &super::service::Empty) -> ::grpcio::Result<super::service::Empty> {
        self.run_opt(req, ::grpcio::CallOption::default())
    }

    pub fn run_async_opt(&self, req: &super::service::Empty, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::Empty>> {
        self.client.unary_call_async(&METHOD_RUNNER_CD_RUN, req, opt)
    }

    pub fn run_async(&self, req: &super::service::Empty) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::Empty>> {
        self.run_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait RunnerCd {
    fn run(&mut self, ctx: ::grpcio::RpcContext, req: super::service::Empty, sink: ::grpcio::UnarySink<super::service::Empty>);
}

pub fn create_runner_cd<S: RunnerCd + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_RUNNER_CD_RUN, move |ctx, req, resp| {
        instance.run(ctx, req, resp)
    });
    builder.build()
}
