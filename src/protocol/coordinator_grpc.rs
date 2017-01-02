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

pub trait Coordinator {
    fn create_table(&self, p: super::coordinator::CreateTableRequest) -> ::grpc::result::GrpcResult<super::coordinator::CreateTableResponse>;

    fn delete_table(&self, p: super::coordinator::DeleteTableRequest) -> ::grpc::result::GrpcResult<super::coordinator::DeleteTableResponse>;
}

pub trait CoordinatorAsync {
    fn create_table(&self, p: super::coordinator::CreateTableRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::coordinator::CreateTableResponse>;

    fn delete_table(&self, p: super::coordinator::DeleteTableRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::coordinator::DeleteTableResponse>;
}

// sync client

pub struct CoordinatorClient {
    async_client: CoordinatorAsyncClient,
}

impl CoordinatorClient {
    pub fn new(host: &str, port: u16, tls: bool) -> ::grpc::result::GrpcResult<Self> {
        CoordinatorAsyncClient::new(host, port, tls).map(|c| {
            CoordinatorClient {
                async_client: c,
            }
        })
    }
}

impl Coordinator for CoordinatorClient {
    fn create_table(&self, p: super::coordinator::CreateTableRequest) -> ::grpc::result::GrpcResult<super::coordinator::CreateTableResponse> {
        ::futures::Future::wait(self.async_client.create_table(p))
    }

    fn delete_table(&self, p: super::coordinator::DeleteTableRequest) -> ::grpc::result::GrpcResult<super::coordinator::DeleteTableResponse> {
        ::futures::Future::wait(self.async_client.delete_table(p))
    }
}

// async client

pub struct CoordinatorAsyncClient {
    grpc_client: ::grpc::client::GrpcClient,
    method_create_table: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::coordinator::CreateTableRequest, super::coordinator::CreateTableResponse>>,
    method_delete_table: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::coordinator::DeleteTableRequest, super::coordinator::DeleteTableResponse>>,
}

impl CoordinatorAsyncClient {
    pub fn new(host: &str, port: u16, tls: bool) -> ::grpc::result::GrpcResult<Self> {
        ::grpc::client::GrpcClient::new(host, port, tls).map(|c| {
            CoordinatorAsyncClient {
                grpc_client: c,
                method_create_table: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/Coordinator/create_table".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
                method_delete_table: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/Coordinator/delete_table".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
            }
        })
    }
}

impl CoordinatorAsync for CoordinatorAsyncClient {
    fn create_table(&self, p: super::coordinator::CreateTableRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::coordinator::CreateTableResponse> {
        self.grpc_client.call_unary(p, self.method_create_table.clone())
    }

    fn delete_table(&self, p: super::coordinator::DeleteTableRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::coordinator::DeleteTableResponse> {
        self.grpc_client.call_unary(p, self.method_delete_table.clone())
    }
}

// sync server

pub struct CoordinatorServer {
    async_server: CoordinatorAsyncServer,
}

struct CoordinatorServerHandlerToAsync {
    handler: ::std::sync::Arc<Coordinator + Send + Sync>,
    cpupool: ::futures_cpupool::CpuPool,
}

impl CoordinatorAsync for CoordinatorServerHandlerToAsync {
    fn create_table(&self, p: super::coordinator::CreateTableRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::coordinator::CreateTableResponse> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.create_table(p)
        })
    }

    fn delete_table(&self, p: super::coordinator::DeleteTableRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::coordinator::DeleteTableResponse> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.delete_table(p)
        })
    }
}

impl CoordinatorServer {
    pub fn new<H : Coordinator + Send + Sync + 'static>(port: u16, h: H) -> Self {
        let h = CoordinatorServerHandlerToAsync {
            cpupool: ::futures_cpupool::CpuPool::new_num_cpus(),
            handler: ::std::sync::Arc::new(h),
        };
        CoordinatorServer {
            async_server: CoordinatorAsyncServer::new(port, h),
        }
    }
}

// async server

pub struct CoordinatorAsyncServer {
    grpc_server: ::grpc::server::GrpcServer,
}

impl CoordinatorAsyncServer {
    pub fn new<H : CoordinatorAsync + 'static + Sync + Send + 'static>(port: u16, h: H) -> Self {
        let handler_arc = ::std::sync::Arc::new(h);
        let service_definition = ::grpc::server::ServerServiceDefinition::new(
            vec![
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/Coordinator/create_table".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |p| handler_copy.create_table(p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/Coordinator/delete_table".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |p| handler_copy.delete_table(p))
                    },
                ),
            ],
        );
        CoordinatorAsyncServer {
            grpc_server: ::grpc::server::GrpcServer::new(port, service_definition),
        }
    }
}
