// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]


// interface

pub trait PartServer {
    fn read(&self, p: super::client::ReadRequest) -> ::grpc::result::GrpcResult<super::client::ReadResponse>;

    fn set(&self, p: super::client::SetRequest) -> ::grpc::result::GrpcResult<super::client::SetResponse>;
}

pub trait PartServerAsync {
    fn read(&self, p: super::client::ReadRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::client::ReadResponse>;

    fn set(&self, p: super::client::SetRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::client::SetResponse>;
}

// sync client

pub struct PartServerClient {
    async_client: PartServerAsyncClient,
}

impl PartServerClient {
    pub fn new(host: &str, port: u16, tls: bool) -> ::grpc::result::GrpcResult<Self> {
        PartServerAsyncClient::new(host, port, tls).map(|c| {
            PartServerClient {
                async_client: c,
            }
        })
    }
}

impl PartServer for PartServerClient {
    fn read(&self, p: super::client::ReadRequest) -> ::grpc::result::GrpcResult<super::client::ReadResponse> {
        ::futures::Future::wait(self.async_client.read(p))
    }

    fn set(&self, p: super::client::SetRequest) -> ::grpc::result::GrpcResult<super::client::SetResponse> {
        ::futures::Future::wait(self.async_client.set(p))
    }
}

// async client

pub struct PartServerAsyncClient {
    grpc_client: ::grpc::client::GrpcClient,
    method_read: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::client::ReadRequest, super::client::ReadResponse>>,
    method_set: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::client::SetRequest, super::client::SetResponse>>,
}

impl PartServerAsyncClient {
    pub fn new(host: &str, port: u16, tls: bool) -> ::grpc::result::GrpcResult<Self> {
        ::grpc::client::GrpcClient::new(host, port, tls).map(|c| {
            PartServerAsyncClient {
                grpc_client: c,
                method_read: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/PartServer/read".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
                method_set: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/PartServer/set".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
            }
        })
    }
}

impl PartServerAsync for PartServerAsyncClient {
    fn read(&self, p: super::client::ReadRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::client::ReadResponse> {
        self.grpc_client.call_unary(p, self.method_read.clone())
    }

    fn set(&self, p: super::client::SetRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::client::SetResponse> {
        self.grpc_client.call_unary(p, self.method_set.clone())
    }
}

// sync server

pub struct PartServerServer {
    async_server: PartServerAsyncServer,
}

struct PartServerServerHandlerToAsync {
    handler: ::std::sync::Arc<PartServer + Send + Sync>,
    cpupool: ::futures_cpupool::CpuPool,
}

impl PartServerAsync for PartServerServerHandlerToAsync {
    fn read(&self, p: super::client::ReadRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::client::ReadResponse> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.read(p)
        })
    }

    fn set(&self, p: super::client::SetRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::client::SetResponse> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.set(p)
        })
    }
}

impl PartServerServer {
    pub fn new<H : PartServer + Send + Sync + 'static>(port: u16, h: H) -> Self {
        let h = PartServerServerHandlerToAsync {
            cpupool: ::futures_cpupool::CpuPool::new_num_cpus(),
            handler: ::std::sync::Arc::new(h),
        };
        PartServerServer {
            async_server: PartServerAsyncServer::new(port, h),
        }
    }
}

// async server

pub struct PartServerAsyncServer {
    grpc_server: ::grpc::server::GrpcServer,
}

impl PartServerAsyncServer {
    pub fn new<H : PartServerAsync + 'static + Sync + Send + 'static>(port: u16, h: H) -> Self {
        let handler_arc = ::std::sync::Arc::new(h);
        let service_definition = ::grpc::server::ServerServiceDefinition::new(
            vec![
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/PartServer/read".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |p| handler_copy.read(p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/PartServer/set".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |p| handler_copy.set(p))
                    },
                ),
            ],
        );
        PartServerAsyncServer {
            grpc_server: ::grpc::server::GrpcServer::new(port, service_definition),
        }
    }
}
