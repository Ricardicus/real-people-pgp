// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

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


// server interface

pub trait PoH {
    fn verify_file_attached_signature(&self, o: ::grpc::ServerHandlerContext, req: ::grpc::ServerRequestSingle<super::poh::VerifyAttachedSignatureRequest>, resp: ::grpc::ServerResponseUnarySink<super::poh::VerifyResponse>) -> ::grpc::Result<()>;

    fn verify_detached_signature(&self, o: ::grpc::ServerHandlerContext, req: ::grpc::ServerRequestSingle<super::poh::VerifyDetachedSignatureRequest>, resp: ::grpc::ServerResponseUnarySink<super::poh::VerifyResponse>) -> ::grpc::Result<()>;
}

// client

pub struct PoHClient {
    grpc_client: ::std::sync::Arc<::grpc::Client>,
}

impl ::grpc::ClientStub for PoHClient {
    fn with_client(grpc_client: ::std::sync::Arc<::grpc::Client>) -> Self {
        PoHClient {
            grpc_client: grpc_client,
        }
    }
}

impl PoHClient {
    pub fn verify_file_attached_signature(&self, o: ::grpc::RequestOptions, req: super::poh::VerifyAttachedSignatureRequest) -> ::grpc::SingleResponse<super::poh::VerifyResponse> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/poh.PoH/verify_file_attached_signature"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn verify_detached_signature(&self, o: ::grpc::RequestOptions, req: super::poh::VerifyDetachedSignatureRequest) -> ::grpc::SingleResponse<super::poh::VerifyResponse> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/poh.PoH/verify_detached_signature"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }
}

// server

pub struct PoHServer;


impl PoHServer {
    pub fn new_service_def<H : PoH + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new("/poh.PoH",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/poh.PoH/verify_file_attached_signature"),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| (*handler_copy).verify_file_attached_signature(ctx, req, resp))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/poh.PoH/verify_detached_signature"),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| (*handler_copy).verify_detached_signature(ctx, req, resp))
                    },
                ),
            ],
        )
    }
}
