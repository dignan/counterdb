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

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(Clone,Default)]
pub struct ReadRequest {
    // message fields
    key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ReadRequest {}

impl ReadRequest {
    pub fn new() -> ReadRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ReadRequest {
        static mut instance: ::protobuf::lazy::Lazy<ReadRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ReadRequest,
        };
        unsafe {
            instance.get(|| {
                ReadRequest {
                    key: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional bytes key = 1;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    pub fn has_key(&self) -> bool {
        self.key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.key.is_none() {
            self.key.set_default();
        };
        self.key.as_mut().unwrap()
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::vec::Vec<u8> {
        self.key.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_key(&self) -> &[u8] {
        match self.key.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::protobuf::Message for ReadRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.key));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.key {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.key.as_ref() {
            try!(os.write_bytes(1, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<ReadRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ReadRequest {
    fn new() -> ReadRequest {
        ReadRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<ReadRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "key",
                    ReadRequest::has_key,
                    ReadRequest::get_key,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ReadRequest>(
                    "ReadRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ReadRequest {
    fn clear(&mut self) {
        self.clear_key();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ReadRequest {
    fn eq(&self, other: &ReadRequest) -> bool {
        self.key == other.key &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ReadRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ReadResponse {
    // message fields
    value: ::std::option::Option<i64>,
    is_value: ::std::option::Option<bool>,
    is_error: ::std::option::Option<bool>,
    error_message: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ReadResponse {}

impl ReadResponse {
    pub fn new() -> ReadResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ReadResponse {
        static mut instance: ::protobuf::lazy::Lazy<ReadResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ReadResponse,
        };
        unsafe {
            instance.get(|| {
                ReadResponse {
                    value: ::std::option::Option::None,
                    is_value: ::std::option::Option::None,
                    is_error: ::std::option::Option::None,
                    error_message: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int64 value = 1;

    pub fn clear_value(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: i64) {
        self.value = ::std::option::Option::Some(v);
    }

    pub fn get_value(&self) -> i64 {
        self.value.unwrap_or(0)
    }

    // optional bool is_value = 2;

    pub fn clear_is_value(&mut self) {
        self.is_value = ::std::option::Option::None;
    }

    pub fn has_is_value(&self) -> bool {
        self.is_value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_value(&mut self, v: bool) {
        self.is_value = ::std::option::Option::Some(v);
    }

    pub fn get_is_value(&self) -> bool {
        self.is_value.unwrap_or(false)
    }

    // optional bool is_error = 3;

    pub fn clear_is_error(&mut self) {
        self.is_error = ::std::option::Option::None;
    }

    pub fn has_is_error(&self) -> bool {
        self.is_error.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_error(&mut self, v: bool) {
        self.is_error = ::std::option::Option::Some(v);
    }

    pub fn get_is_error(&self) -> bool {
        self.is_error.unwrap_or(false)
    }

    // optional string error_message = 4;

    pub fn clear_error_message(&mut self) {
        self.error_message.clear();
    }

    pub fn has_error_message(&self) -> bool {
        self.error_message.is_some()
    }

    // Param is passed by value, moved
    pub fn set_error_message(&mut self, v: ::std::string::String) {
        self.error_message = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error_message(&mut self) -> &mut ::std::string::String {
        if self.error_message.is_none() {
            self.error_message.set_default();
        };
        self.error_message.as_mut().unwrap()
    }

    // Take field
    pub fn take_error_message(&mut self) -> ::std::string::String {
        self.error_message.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_error_message(&self) -> &str {
        match self.error_message.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for ReadResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int64());
                    self.value = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.is_value = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.is_error = ::std::option::Option::Some(tmp);
                },
                4 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.error_message));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.value {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.is_value.is_some() {
            my_size += 2;
        };
        if self.is_error.is_some() {
            my_size += 2;
        };
        for value in &self.error_message {
            my_size += ::protobuf::rt::string_size(4, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.value {
            try!(os.write_int64(1, v));
        };
        if let Some(v) = self.is_value {
            try!(os.write_bool(2, v));
        };
        if let Some(v) = self.is_error {
            try!(os.write_bool(3, v));
        };
        if let Some(v) = self.error_message.as_ref() {
            try!(os.write_string(4, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<ReadResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ReadResponse {
    fn new() -> ReadResponse {
        ReadResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<ReadResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "value",
                    ReadResponse::has_value,
                    ReadResponse::get_value,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "is_value",
                    ReadResponse::has_is_value,
                    ReadResponse::get_is_value,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "is_error",
                    ReadResponse::has_is_error,
                    ReadResponse::get_is_error,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "error_message",
                    ReadResponse::has_error_message,
                    ReadResponse::get_error_message,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ReadResponse>(
                    "ReadResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ReadResponse {
    fn clear(&mut self) {
        self.clear_value();
        self.clear_is_value();
        self.clear_is_error();
        self.clear_error_message();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ReadResponse {
    fn eq(&self, other: &ReadResponse) -> bool {
        self.value == other.value &&
        self.is_value == other.is_value &&
        self.is_error == other.is_error &&
        self.error_message == other.error_message &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ReadResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct SetRequest {
    // message fields
    key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    value: ::std::option::Option<i64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SetRequest {}

impl SetRequest {
    pub fn new() -> SetRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SetRequest {
        static mut instance: ::protobuf::lazy::Lazy<SetRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SetRequest,
        };
        unsafe {
            instance.get(|| {
                SetRequest {
                    key: ::protobuf::SingularField::none(),
                    value: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional bytes key = 1;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    pub fn has_key(&self) -> bool {
        self.key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.key.is_none() {
            self.key.set_default();
        };
        self.key.as_mut().unwrap()
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::vec::Vec<u8> {
        self.key.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_key(&self) -> &[u8] {
        match self.key.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional int64 value = 2;

    pub fn clear_value(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: i64) {
        self.value = ::std::option::Option::Some(v);
    }

    pub fn get_value(&self) -> i64 {
        self.value.unwrap_or(0)
    }
}

impl ::protobuf::Message for SetRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.key));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int64());
                    self.value = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.key {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        for value in &self.value {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.key.as_ref() {
            try!(os.write_bytes(1, &v));
        };
        if let Some(v) = self.value {
            try!(os.write_int64(2, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<SetRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for SetRequest {
    fn new() -> SetRequest {
        SetRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<SetRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "key",
                    SetRequest::has_key,
                    SetRequest::get_key,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "value",
                    SetRequest::has_value,
                    SetRequest::get_value,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SetRequest>(
                    "SetRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SetRequest {
    fn clear(&mut self) {
        self.clear_key();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for SetRequest {
    fn eq(&self, other: &SetRequest) -> bool {
        self.key == other.key &&
        self.value == other.value &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for SetRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct SetResponse {
    // message fields
    is_error: ::std::option::Option<bool>,
    error_message: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SetResponse {}

impl SetResponse {
    pub fn new() -> SetResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SetResponse {
        static mut instance: ::protobuf::lazy::Lazy<SetResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SetResponse,
        };
        unsafe {
            instance.get(|| {
                SetResponse {
                    is_error: ::std::option::Option::None,
                    error_message: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional bool is_error = 1;

    pub fn clear_is_error(&mut self) {
        self.is_error = ::std::option::Option::None;
    }

    pub fn has_is_error(&self) -> bool {
        self.is_error.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_error(&mut self, v: bool) {
        self.is_error = ::std::option::Option::Some(v);
    }

    pub fn get_is_error(&self) -> bool {
        self.is_error.unwrap_or(false)
    }

    // optional string error_message = 4;

    pub fn clear_error_message(&mut self) {
        self.error_message.clear();
    }

    pub fn has_error_message(&self) -> bool {
        self.error_message.is_some()
    }

    // Param is passed by value, moved
    pub fn set_error_message(&mut self, v: ::std::string::String) {
        self.error_message = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error_message(&mut self) -> &mut ::std::string::String {
        if self.error_message.is_none() {
            self.error_message.set_default();
        };
        self.error_message.as_mut().unwrap()
    }

    // Take field
    pub fn take_error_message(&mut self) -> ::std::string::String {
        self.error_message.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_error_message(&self) -> &str {
        match self.error_message.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for SetResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.is_error = ::std::option::Option::Some(tmp);
                },
                4 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.error_message));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.is_error.is_some() {
            my_size += 2;
        };
        for value in &self.error_message {
            my_size += ::protobuf::rt::string_size(4, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.is_error {
            try!(os.write_bool(1, v));
        };
        if let Some(v) = self.error_message.as_ref() {
            try!(os.write_string(4, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<SetResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for SetResponse {
    fn new() -> SetResponse {
        SetResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<SetResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "is_error",
                    SetResponse::has_is_error,
                    SetResponse::get_is_error,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "error_message",
                    SetResponse::has_error_message,
                    SetResponse::get_error_message,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SetResponse>(
                    "SetResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SetResponse {
    fn clear(&mut self) {
        self.clear_is_error();
        self.clear_error_message();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for SetResponse {
    fn eq(&self, other: &SetResponse) -> bool {
        self.is_error == other.is_error &&
        self.error_message == other.error_message &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for SetResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x20, 0x73, 0x72, 0x63, 0x2f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x2f, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2f, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x2e, 0x70, 0x72, 0x6f,
    0x74, 0x6f, 0x22, 0x1f, 0x0a, 0x0b, 0x52, 0x65, 0x61, 0x64, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73,
    0x74, 0x12, 0x10, 0x0a, 0x03, 0x6b, 0x65, 0x79, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x03,
    0x6b, 0x65, 0x79, 0x22, 0x7f, 0x0a, 0x0c, 0x52, 0x65, 0x61, 0x64, 0x52, 0x65, 0x73, 0x70, 0x6f,
    0x6e, 0x73, 0x65, 0x12, 0x14, 0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x03, 0x52, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x12, 0x19, 0x0a, 0x08, 0x69, 0x73, 0x5f,
    0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x08, 0x52, 0x07, 0x69, 0x73, 0x56,
    0x61, 0x6c, 0x75, 0x65, 0x12, 0x19, 0x0a, 0x08, 0x69, 0x73, 0x5f, 0x65, 0x72, 0x72, 0x6f, 0x72,
    0x18, 0x03, 0x20, 0x01, 0x28, 0x08, 0x52, 0x07, 0x69, 0x73, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x12,
    0x23, 0x0a, 0x0d, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x5f, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65,
    0x18, 0x04, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0c, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x4d, 0x65, 0x73,
    0x73, 0x61, 0x67, 0x65, 0x22, 0x34, 0x0a, 0x0a, 0x53, 0x65, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65,
    0x73, 0x74, 0x12, 0x10, 0x0a, 0x03, 0x6b, 0x65, 0x79, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0c, 0x52,
    0x03, 0x6b, 0x65, 0x79, 0x12, 0x14, 0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x02, 0x20,
    0x01, 0x28, 0x03, 0x52, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x22, 0x4d, 0x0a, 0x0b, 0x53, 0x65,
    0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x19, 0x0a, 0x08, 0x69, 0x73, 0x5f,
    0x65, 0x72, 0x72, 0x6f, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x08, 0x52, 0x07, 0x69, 0x73, 0x45,
    0x72, 0x72, 0x6f, 0x72, 0x12, 0x23, 0x0a, 0x0d, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x5f, 0x6d, 0x65,
    0x73, 0x73, 0x61, 0x67, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0c, 0x65, 0x72, 0x72,
    0x6f, 0x72, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x32, 0x57, 0x0a, 0x0a, 0x50, 0x61, 0x72,
    0x74, 0x53, 0x65, 0x72, 0x76, 0x65, 0x72, 0x12, 0x25, 0x0a, 0x04, 0x72, 0x65, 0x61, 0x64, 0x12,
    0x0c, 0x2e, 0x52, 0x65, 0x61, 0x64, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x0d, 0x2e,
    0x52, 0x65, 0x61, 0x64, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22, 0x00, 0x12, 0x22,
    0x0a, 0x03, 0x73, 0x65, 0x74, 0x12, 0x0b, 0x2e, 0x53, 0x65, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65,
    0x73, 0x74, 0x1a, 0x0c, 0x2e, 0x53, 0x65, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65,
    0x22, 0x00, 0x4a, 0xee, 0x06, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x1a, 0x01, 0x0a, 0x08, 0x0a,
    0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x0a, 0x0a, 0x02, 0x06, 0x00, 0x12, 0x04, 0x02,
    0x00, 0x05, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x06, 0x00, 0x01, 0x12, 0x03, 0x02, 0x08, 0x12, 0x0a,
    0x0b, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x00, 0x12, 0x03, 0x03, 0x02, 0x32, 0x0a, 0x0c, 0x0a, 0x05,
    0x06, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x03, 0x06, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00,
    0x02, 0x00, 0x02, 0x12, 0x03, 0x03, 0x0c, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x03, 0x22, 0x2e, 0x0a, 0x0b, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x01, 0x12, 0x03,
    0x04, 0x02, 0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x04, 0x06,
    0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x04, 0x0b, 0x15, 0x0a,
    0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x04, 0x20, 0x2b, 0x0a, 0x0a, 0x0a,
    0x02, 0x04, 0x00, 0x12, 0x04, 0x07, 0x00, 0x09, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01,
    0x12, 0x03, 0x07, 0x08, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x08,
    0x02, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x04, 0x08, 0x02, 0x07,
    0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x08, 0x02, 0x07, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x08, 0x08, 0x0b, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x08, 0x0e, 0x0f, 0x0a, 0x0a, 0x0a, 0x02, 0x04,
    0x01, 0x12, 0x04, 0x0b, 0x00, 0x10, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03,
    0x0b, 0x08, 0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x0c, 0x02, 0x12,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x04, 0x0c, 0x02, 0x0b, 0x16, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x0c, 0x02, 0x07, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0c, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0c, 0x10, 0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02,
    0x01, 0x12, 0x03, 0x0d, 0x02, 0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x04, 0x12,
    0x04, 0x0d, 0x02, 0x0c, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x05, 0x12, 0x03,
    0x0d, 0x02, 0x06, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0d, 0x07,
    0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x0d, 0x12, 0x13, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x02, 0x12, 0x03, 0x0e, 0x02, 0x14, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x02, 0x04, 0x12, 0x04, 0x0e, 0x02, 0x0d, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x02, 0x05, 0x12, 0x03, 0x0e, 0x02, 0x06, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x02, 0x01, 0x12, 0x03, 0x0e, 0x07, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x03,
    0x12, 0x03, 0x0e, 0x12, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x03, 0x12, 0x03, 0x0f,
    0x02, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x04, 0x12, 0x04, 0x0f, 0x02, 0x0e,
    0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x05, 0x12, 0x03, 0x0f, 0x02, 0x08, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x01, 0x12, 0x03, 0x0f, 0x09, 0x16, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x03, 0x03, 0x12, 0x03, 0x0f, 0x19, 0x1a, 0x0a, 0x0a, 0x0a, 0x02, 0x04,
    0x02, 0x12, 0x04, 0x12, 0x00, 0x15, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03,
    0x12, 0x08, 0x12, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x13, 0x02, 0x10,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x04, 0x13, 0x02, 0x12, 0x14, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x05, 0x12, 0x03, 0x13, 0x02, 0x07, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x13, 0x08, 0x0b, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x13, 0x0e, 0x0f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02,
    0x01, 0x12, 0x03, 0x14, 0x02, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x04, 0x12,
    0x04, 0x14, 0x02, 0x13, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x05, 0x12, 0x03,
    0x14, 0x02, 0x07, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x01, 0x12, 0x03, 0x14, 0x08,
    0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x03, 0x12, 0x03, 0x14, 0x10, 0x11, 0x0a,
    0x0a, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x17, 0x00, 0x1a, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x03, 0x01, 0x12, 0x03, 0x17, 0x08, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12,
    0x03, 0x18, 0x02, 0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x04, 0x12, 0x04, 0x18,
    0x02, 0x17, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x05, 0x12, 0x03, 0x18, 0x02,
    0x06, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x18, 0x07, 0x0f, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03, 0x18, 0x12, 0x13, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x03, 0x02, 0x01, 0x12, 0x03, 0x19, 0x02, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x01, 0x04, 0x12, 0x04, 0x19, 0x02, 0x18, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x01, 0x05, 0x12, 0x03, 0x19, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x19, 0x09, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x03, 0x12, 0x03,
    0x19, 0x19, 0x1a, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
