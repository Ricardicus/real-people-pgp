// This file is generated by rust-protobuf 2.18.2. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `proto/poh.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_18_2;

#[derive(PartialEq,Clone,Default)]
pub struct VerifyAttachedSignatureRequest {
    // message fields
    pub file_attached_signature: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a VerifyAttachedSignatureRequest {
    fn default() -> &'a VerifyAttachedSignatureRequest {
        <VerifyAttachedSignatureRequest as ::protobuf::Message>::default_instance()
    }
}

impl VerifyAttachedSignatureRequest {
    pub fn new() -> VerifyAttachedSignatureRequest {
        ::std::default::Default::default()
    }

    // string file_attached_signature = 1;


    pub fn get_file_attached_signature(&self) -> &str {
        &self.file_attached_signature
    }
    pub fn clear_file_attached_signature(&mut self) {
        self.file_attached_signature.clear();
    }

    // Param is passed by value, moved
    pub fn set_file_attached_signature(&mut self, v: ::std::string::String) {
        self.file_attached_signature = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_file_attached_signature(&mut self) -> &mut ::std::string::String {
        &mut self.file_attached_signature
    }

    // Take field
    pub fn take_file_attached_signature(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.file_attached_signature, ::std::string::String::new())
    }
}

impl ::protobuf::Message for VerifyAttachedSignatureRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.file_attached_signature)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.file_attached_signature.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.file_attached_signature);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.file_attached_signature.is_empty() {
            os.write_string(1, &self.file_attached_signature)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> VerifyAttachedSignatureRequest {
        VerifyAttachedSignatureRequest::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "file_attached_signature",
                |m: &VerifyAttachedSignatureRequest| { &m.file_attached_signature },
                |m: &mut VerifyAttachedSignatureRequest| { &mut m.file_attached_signature },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<VerifyAttachedSignatureRequest>(
                "VerifyAttachedSignatureRequest",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static VerifyAttachedSignatureRequest {
        static instance: ::protobuf::rt::LazyV2<VerifyAttachedSignatureRequest> = ::protobuf::rt::LazyV2::INIT;
        instance.get(VerifyAttachedSignatureRequest::new)
    }
}

impl ::protobuf::Clear for VerifyAttachedSignatureRequest {
    fn clear(&mut self) {
        self.file_attached_signature.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for VerifyAttachedSignatureRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for VerifyAttachedSignatureRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct VerifyDetachedSignatureRequest {
    // message fields
    pub detached_signature: ::std::string::String,
    pub file_contents: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a VerifyDetachedSignatureRequest {
    fn default() -> &'a VerifyDetachedSignatureRequest {
        <VerifyDetachedSignatureRequest as ::protobuf::Message>::default_instance()
    }
}

impl VerifyDetachedSignatureRequest {
    pub fn new() -> VerifyDetachedSignatureRequest {
        ::std::default::Default::default()
    }

    // string detached_signature = 1;


    pub fn get_detached_signature(&self) -> &str {
        &self.detached_signature
    }
    pub fn clear_detached_signature(&mut self) {
        self.detached_signature.clear();
    }

    // Param is passed by value, moved
    pub fn set_detached_signature(&mut self, v: ::std::string::String) {
        self.detached_signature = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_detached_signature(&mut self) -> &mut ::std::string::String {
        &mut self.detached_signature
    }

    // Take field
    pub fn take_detached_signature(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.detached_signature, ::std::string::String::new())
    }

    // string file_contents = 2;


    pub fn get_file_contents(&self) -> &str {
        &self.file_contents
    }
    pub fn clear_file_contents(&mut self) {
        self.file_contents.clear();
    }

    // Param is passed by value, moved
    pub fn set_file_contents(&mut self, v: ::std::string::String) {
        self.file_contents = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_file_contents(&mut self) -> &mut ::std::string::String {
        &mut self.file_contents
    }

    // Take field
    pub fn take_file_contents(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.file_contents, ::std::string::String::new())
    }
}

impl ::protobuf::Message for VerifyDetachedSignatureRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.detached_signature)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.file_contents)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.detached_signature.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.detached_signature);
        }
        if !self.file_contents.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.file_contents);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.detached_signature.is_empty() {
            os.write_string(1, &self.detached_signature)?;
        }
        if !self.file_contents.is_empty() {
            os.write_string(2, &self.file_contents)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> VerifyDetachedSignatureRequest {
        VerifyDetachedSignatureRequest::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "detached_signature",
                |m: &VerifyDetachedSignatureRequest| { &m.detached_signature },
                |m: &mut VerifyDetachedSignatureRequest| { &mut m.detached_signature },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "file_contents",
                |m: &VerifyDetachedSignatureRequest| { &m.file_contents },
                |m: &mut VerifyDetachedSignatureRequest| { &mut m.file_contents },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<VerifyDetachedSignatureRequest>(
                "VerifyDetachedSignatureRequest",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static VerifyDetachedSignatureRequest {
        static instance: ::protobuf::rt::LazyV2<VerifyDetachedSignatureRequest> = ::protobuf::rt::LazyV2::INIT;
        instance.get(VerifyDetachedSignatureRequest::new)
    }
}

impl ::protobuf::Clear for VerifyDetachedSignatureRequest {
    fn clear(&mut self) {
        self.detached_signature.clear();
        self.file_contents.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for VerifyDetachedSignatureRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for VerifyDetachedSignatureRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct VerifyResponse {
    // message fields
    pub valid: bool,
    pub info: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a VerifyResponse {
    fn default() -> &'a VerifyResponse {
        <VerifyResponse as ::protobuf::Message>::default_instance()
    }
}

impl VerifyResponse {
    pub fn new() -> VerifyResponse {
        ::std::default::Default::default()
    }

    // bool valid = 1;


    pub fn get_valid(&self) -> bool {
        self.valid
    }
    pub fn clear_valid(&mut self) {
        self.valid = false;
    }

    // Param is passed by value, moved
    pub fn set_valid(&mut self, v: bool) {
        self.valid = v;
    }

    // string info = 2;


    pub fn get_info(&self) -> &str {
        &self.info
    }
    pub fn clear_info(&mut self) {
        self.info.clear();
    }

    // Param is passed by value, moved
    pub fn set_info(&mut self, v: ::std::string::String) {
        self.info = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_info(&mut self) -> &mut ::std::string::String {
        &mut self.info
    }

    // Take field
    pub fn take_info(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.info, ::std::string::String::new())
    }
}

impl ::protobuf::Message for VerifyResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.valid = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.info)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.valid != false {
            my_size += 2;
        }
        if !self.info.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.info);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if self.valid != false {
            os.write_bool(1, self.valid)?;
        }
        if !self.info.is_empty() {
            os.write_string(2, &self.info)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> VerifyResponse {
        VerifyResponse::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                "valid",
                |m: &VerifyResponse| { &m.valid },
                |m: &mut VerifyResponse| { &mut m.valid },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "info",
                |m: &VerifyResponse| { &m.info },
                |m: &mut VerifyResponse| { &mut m.info },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<VerifyResponse>(
                "VerifyResponse",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static VerifyResponse {
        static instance: ::protobuf::rt::LazyV2<VerifyResponse> = ::protobuf::rt::LazyV2::INIT;
        instance.get(VerifyResponse::new)
    }
}

impl ::protobuf::Clear for VerifyResponse {
    fn clear(&mut self) {
        self.valid = false;
        self.info.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for VerifyResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for VerifyResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0fproto/poh.proto\x12\x03poh\"X\n\x1eVerifyAttachedSignatureRequest\
    \x126\n\x17file_attached_signature\x18\x01\x20\x01(\tR\x15fileAttachedSi\
    gnature\"t\n\x1eVerifyDetachedSignatureRequest\x12-\n\x12detached_signat\
    ure\x18\x01\x20\x01(\tR\x11detachedSignature\x12#\n\rfile_contents\x18\
    \x02\x20\x01(\tR\x0cfileContents\":\n\x0eVerifyResponse\x12\x14\n\x05val\
    id\x18\x01\x20\x01(\x08R\x05valid\x12\x12\n\x04info\x18\x02\x20\x01(\tR\
    \x04info2\xb3\x01\n\x03PoH\x12U\n\x19verify_attached_signature\x12#.poh.\
    VerifyAttachedSignatureRequest\x1a\x13.poh.VerifyResponse\x12U\n\x19veri\
    fy_detached_signature\x12#.poh.VerifyDetachedSignatureRequest\x1a\x13.po\
    h.VerifyResponseb\x06proto3\
";

static file_descriptor_proto_lazy: ::protobuf::rt::LazyV2<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::LazyV2::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}
