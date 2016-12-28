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
pub struct CreateTableRequest {
    // message fields
    table: ::protobuf::SingularPtrField<TableProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CreateTableRequest {}

impl CreateTableRequest {
    pub fn new() -> CreateTableRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CreateTableRequest {
        static mut instance: ::protobuf::lazy::Lazy<CreateTableRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CreateTableRequest,
        };
        unsafe {
            instance.get(|| {
                CreateTableRequest {
                    table: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .TableProto table = 1;

    pub fn clear_table(&mut self) {
        self.table.clear();
    }

    pub fn has_table(&self) -> bool {
        self.table.is_some()
    }

    // Param is passed by value, moved
    pub fn set_table(&mut self, v: TableProto) {
        self.table = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_table(&mut self) -> &mut TableProto {
        if self.table.is_none() {
            self.table.set_default();
        };
        self.table.as_mut().unwrap()
    }

    // Take field
    pub fn take_table(&mut self) -> TableProto {
        self.table.take().unwrap_or_else(|| TableProto::new())
    }

    pub fn get_table(&self) -> &TableProto {
        self.table.as_ref().unwrap_or_else(|| TableProto::default_instance())
    }
}

impl ::protobuf::Message for CreateTableRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.table));
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
        for value in &self.table {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.table.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
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
        ::std::any::TypeId::of::<CreateTableRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CreateTableRequest {
    fn new() -> CreateTableRequest {
        CreateTableRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<CreateTableRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "table",
                    CreateTableRequest::has_table,
                    CreateTableRequest::get_table,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CreateTableRequest>(
                    "CreateTableRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CreateTableRequest {
    fn clear(&mut self) {
        self.clear_table();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CreateTableRequest {
    fn eq(&self, other: &CreateTableRequest) -> bool {
        self.table == other.table &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CreateTableRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CreateTableResponse {
    // message fields
    success: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CreateTableResponse {}

impl CreateTableResponse {
    pub fn new() -> CreateTableResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CreateTableResponse {
        static mut instance: ::protobuf::lazy::Lazy<CreateTableResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CreateTableResponse,
        };
        unsafe {
            instance.get(|| {
                CreateTableResponse {
                    success: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional bool success = 1;

    pub fn clear_success(&mut self) {
        self.success = ::std::option::Option::None;
    }

    pub fn has_success(&self) -> bool {
        self.success.is_some()
    }

    // Param is passed by value, moved
    pub fn set_success(&mut self, v: bool) {
        self.success = ::std::option::Option::Some(v);
    }

    pub fn get_success(&self) -> bool {
        self.success.unwrap_or(false)
    }
}

impl ::protobuf::Message for CreateTableResponse {
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
                    self.success = ::std::option::Option::Some(tmp);
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
        if self.success.is_some() {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.success {
            try!(os.write_bool(1, v));
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
        ::std::any::TypeId::of::<CreateTableResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CreateTableResponse {
    fn new() -> CreateTableResponse {
        CreateTableResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CreateTableResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "success",
                    CreateTableResponse::has_success,
                    CreateTableResponse::get_success,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CreateTableResponse>(
                    "CreateTableResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CreateTableResponse {
    fn clear(&mut self) {
        self.clear_success();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CreateTableResponse {
    fn eq(&self, other: &CreateTableResponse) -> bool {
        self.success == other.success &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CreateTableResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct TableProto {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    partitions: ::protobuf::RepeatedField<PartitionProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TableProto {}

impl TableProto {
    pub fn new() -> TableProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TableProto {
        static mut instance: ::protobuf::lazy::Lazy<TableProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TableProto,
        };
        unsafe {
            instance.get(|| {
                TableProto {
                    name: ::protobuf::SingularField::none(),
                    partitions: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        };
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // repeated .PartitionProto partitions = 2;

    pub fn clear_partitions(&mut self) {
        self.partitions.clear();
    }

    // Param is passed by value, moved
    pub fn set_partitions(&mut self, v: ::protobuf::RepeatedField<PartitionProto>) {
        self.partitions = v;
    }

    // Mutable pointer to the field.
    pub fn mut_partitions(&mut self) -> &mut ::protobuf::RepeatedField<PartitionProto> {
        &mut self.partitions
    }

    // Take field
    pub fn take_partitions(&mut self) -> ::protobuf::RepeatedField<PartitionProto> {
        ::std::mem::replace(&mut self.partitions, ::protobuf::RepeatedField::new())
    }

    pub fn get_partitions(&self) -> &[PartitionProto] {
        &self.partitions
    }
}

impl ::protobuf::Message for TableProto {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name));
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.partitions));
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
        for value in &self.name {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in &self.partitions {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.name.as_ref() {
            try!(os.write_string(1, &v));
        };
        for v in &self.partitions {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
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
        ::std::any::TypeId::of::<TableProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TableProto {
    fn new() -> TableProto {
        TableProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<TableProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "name",
                    TableProto::has_name,
                    TableProto::get_name,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "partitions",
                    TableProto::get_partitions,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TableProto>(
                    "TableProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TableProto {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_partitions();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for TableProto {
    fn eq(&self, other: &TableProto) -> bool {
        self.name == other.name &&
        self.partitions == other.partitions &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for TableProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct PartitionProto {
    // message fields
    id: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    key_range: ::protobuf::SingularPtrField<KeyRangeProto>,
    partition_location: ::protobuf::SingularPtrField<PartitionLocationProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PartitionProto {}

impl PartitionProto {
    pub fn new() -> PartitionProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PartitionProto {
        static mut instance: ::protobuf::lazy::Lazy<PartitionProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PartitionProto,
        };
        unsafe {
            instance.get(|| {
                PartitionProto {
                    id: ::protobuf::SingularField::none(),
                    key_range: ::protobuf::SingularPtrField::none(),
                    partition_location: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional bytes id = 1;

    pub fn clear_id(&mut self) {
        self.id.clear();
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: ::std::vec::Vec<u8>) {
        self.id = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_id(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.id.is_none() {
            self.id.set_default();
        };
        self.id.as_mut().unwrap()
    }

    // Take field
    pub fn take_id(&mut self) -> ::std::vec::Vec<u8> {
        self.id.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_id(&self) -> &[u8] {
        match self.id.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional .KeyRangeProto key_range = 2;

    pub fn clear_key_range(&mut self) {
        self.key_range.clear();
    }

    pub fn has_key_range(&self) -> bool {
        self.key_range.is_some()
    }

    // Param is passed by value, moved
    pub fn set_key_range(&mut self, v: KeyRangeProto) {
        self.key_range = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key_range(&mut self) -> &mut KeyRangeProto {
        if self.key_range.is_none() {
            self.key_range.set_default();
        };
        self.key_range.as_mut().unwrap()
    }

    // Take field
    pub fn take_key_range(&mut self) -> KeyRangeProto {
        self.key_range.take().unwrap_or_else(|| KeyRangeProto::new())
    }

    pub fn get_key_range(&self) -> &KeyRangeProto {
        self.key_range.as_ref().unwrap_or_else(|| KeyRangeProto::default_instance())
    }

    // optional .PartitionLocationProto partition_location = 3;

    pub fn clear_partition_location(&mut self) {
        self.partition_location.clear();
    }

    pub fn has_partition_location(&self) -> bool {
        self.partition_location.is_some()
    }

    // Param is passed by value, moved
    pub fn set_partition_location(&mut self, v: PartitionLocationProto) {
        self.partition_location = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_partition_location(&mut self) -> &mut PartitionLocationProto {
        if self.partition_location.is_none() {
            self.partition_location.set_default();
        };
        self.partition_location.as_mut().unwrap()
    }

    // Take field
    pub fn take_partition_location(&mut self) -> PartitionLocationProto {
        self.partition_location.take().unwrap_or_else(|| PartitionLocationProto::new())
    }

    pub fn get_partition_location(&self) -> &PartitionLocationProto {
        self.partition_location.as_ref().unwrap_or_else(|| PartitionLocationProto::default_instance())
    }
}

impl ::protobuf::Message for PartitionProto {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.id));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.key_range));
                },
                3 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.partition_location));
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
        for value in &self.id {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        for value in &self.key_range {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.partition_location {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.id.as_ref() {
            try!(os.write_bytes(1, &v));
        };
        if let Some(v) = self.key_range.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.partition_location.as_ref() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
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
        ::std::any::TypeId::of::<PartitionProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for PartitionProto {
    fn new() -> PartitionProto {
        PartitionProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<PartitionProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "id",
                    PartitionProto::has_id,
                    PartitionProto::get_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "key_range",
                    PartitionProto::has_key_range,
                    PartitionProto::get_key_range,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "partition_location",
                    PartitionProto::has_partition_location,
                    PartitionProto::get_partition_location,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PartitionProto>(
                    "PartitionProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PartitionProto {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_key_range();
        self.clear_partition_location();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for PartitionProto {
    fn eq(&self, other: &PartitionProto) -> bool {
        self.id == other.id &&
        self.key_range == other.key_range &&
        self.partition_location == other.partition_location &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for PartitionProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct KeyRangeProto {
    // message fields
    start_key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    end_key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for KeyRangeProto {}

impl KeyRangeProto {
    pub fn new() -> KeyRangeProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static KeyRangeProto {
        static mut instance: ::protobuf::lazy::Lazy<KeyRangeProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const KeyRangeProto,
        };
        unsafe {
            instance.get(|| {
                KeyRangeProto {
                    start_key: ::protobuf::SingularField::none(),
                    end_key: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional bytes start_key = 1;

    pub fn clear_start_key(&mut self) {
        self.start_key.clear();
    }

    pub fn has_start_key(&self) -> bool {
        self.start_key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_start_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.start_key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_start_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.start_key.is_none() {
            self.start_key.set_default();
        };
        self.start_key.as_mut().unwrap()
    }

    // Take field
    pub fn take_start_key(&mut self) -> ::std::vec::Vec<u8> {
        self.start_key.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_start_key(&self) -> &[u8] {
        match self.start_key.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional bytes end_key = 2;

    pub fn clear_end_key(&mut self) {
        self.end_key.clear();
    }

    pub fn has_end_key(&self) -> bool {
        self.end_key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_end_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.end_key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_end_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.end_key.is_none() {
            self.end_key.set_default();
        };
        self.end_key.as_mut().unwrap()
    }

    // Take field
    pub fn take_end_key(&mut self) -> ::std::vec::Vec<u8> {
        self.end_key.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_end_key(&self) -> &[u8] {
        match self.end_key.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::protobuf::Message for KeyRangeProto {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.start_key));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.end_key));
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
        for value in &self.start_key {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        for value in &self.end_key {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.start_key.as_ref() {
            try!(os.write_bytes(1, &v));
        };
        if let Some(v) = self.end_key.as_ref() {
            try!(os.write_bytes(2, &v));
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
        ::std::any::TypeId::of::<KeyRangeProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for KeyRangeProto {
    fn new() -> KeyRangeProto {
        KeyRangeProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<KeyRangeProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "start_key",
                    KeyRangeProto::has_start_key,
                    KeyRangeProto::get_start_key,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "end_key",
                    KeyRangeProto::has_end_key,
                    KeyRangeProto::get_end_key,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<KeyRangeProto>(
                    "KeyRangeProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for KeyRangeProto {
    fn clear(&mut self) {
        self.clear_start_key();
        self.clear_end_key();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for KeyRangeProto {
    fn eq(&self, other: &KeyRangeProto) -> bool {
        self.start_key == other.start_key &&
        self.end_key == other.end_key &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for KeyRangeProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct PartitionLocationProto {
    // message fields
    hostname: ::protobuf::SingularField<::std::string::String>,
    port: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PartitionLocationProto {}

impl PartitionLocationProto {
    pub fn new() -> PartitionLocationProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PartitionLocationProto {
        static mut instance: ::protobuf::lazy::Lazy<PartitionLocationProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PartitionLocationProto,
        };
        unsafe {
            instance.get(|| {
                PartitionLocationProto {
                    hostname: ::protobuf::SingularField::none(),
                    port: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string hostname = 1;

    pub fn clear_hostname(&mut self) {
        self.hostname.clear();
    }

    pub fn has_hostname(&self) -> bool {
        self.hostname.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hostname(&mut self, v: ::std::string::String) {
        self.hostname = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_hostname(&mut self) -> &mut ::std::string::String {
        if self.hostname.is_none() {
            self.hostname.set_default();
        };
        self.hostname.as_mut().unwrap()
    }

    // Take field
    pub fn take_hostname(&mut self) -> ::std::string::String {
        self.hostname.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_hostname(&self) -> &str {
        match self.hostname.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional uint32 port = 2;

    pub fn clear_port(&mut self) {
        self.port = ::std::option::Option::None;
    }

    pub fn has_port(&self) -> bool {
        self.port.is_some()
    }

    // Param is passed by value, moved
    pub fn set_port(&mut self, v: u32) {
        self.port = ::std::option::Option::Some(v);
    }

    pub fn get_port(&self) -> u32 {
        self.port.unwrap_or(0)
    }
}

impl ::protobuf::Message for PartitionLocationProto {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.hostname));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.port = ::std::option::Option::Some(tmp);
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
        for value in &self.hostname {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in &self.port {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.hostname.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.port {
            try!(os.write_uint32(2, v));
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
        ::std::any::TypeId::of::<PartitionLocationProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for PartitionLocationProto {
    fn new() -> PartitionLocationProto {
        PartitionLocationProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<PartitionLocationProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "hostname",
                    PartitionLocationProto::has_hostname,
                    PartitionLocationProto::get_hostname,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "port",
                    PartitionLocationProto::has_port,
                    PartitionLocationProto::get_port,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PartitionLocationProto>(
                    "PartitionLocationProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PartitionLocationProto {
    fn clear(&mut self) {
        self.clear_hostname();
        self.clear_port();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for PartitionLocationProto {
    fn eq(&self, other: &PartitionLocationProto) -> bool {
        self.hostname == other.hostname &&
        self.port == other.port &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for PartitionLocationProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x25, 0x73, 0x72, 0x63, 0x2f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x2f, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x73, 0x2f, 0x63, 0x6f, 0x6f, 0x72, 0x64, 0x69, 0x6e, 0x61, 0x74, 0x6f,
    0x72, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x37, 0x0a, 0x12, 0x43, 0x72, 0x65, 0x61, 0x74,
    0x65, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x21, 0x0a,
    0x05, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0b, 0x2e, 0x54,
    0x61, 0x62, 0x6c, 0x65, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x52, 0x05, 0x74, 0x61, 0x62, 0x6c, 0x65,
    0x22, 0x2f, 0x0a, 0x13, 0x43, 0x72, 0x65, 0x61, 0x74, 0x65, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x52,
    0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x18, 0x0a, 0x07, 0x73, 0x75, 0x63, 0x63, 0x65,
    0x73, 0x73, 0x18, 0x01, 0x20, 0x01, 0x28, 0x08, 0x52, 0x07, 0x73, 0x75, 0x63, 0x63, 0x65, 0x73,
    0x73, 0x22, 0x51, 0x0a, 0x0a, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12,
    0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x6e,
    0x61, 0x6d, 0x65, 0x12, 0x2f, 0x0a, 0x0a, 0x70, 0x61, 0x72, 0x74, 0x69, 0x74, 0x69, 0x6f, 0x6e,
    0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x0f, 0x2e, 0x50, 0x61, 0x72, 0x74, 0x69, 0x74,
    0x69, 0x6f, 0x6e, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x52, 0x0a, 0x70, 0x61, 0x72, 0x74, 0x69, 0x74,
    0x69, 0x6f, 0x6e, 0x73, 0x22, 0x95, 0x01, 0x0a, 0x0e, 0x50, 0x61, 0x72, 0x74, 0x69, 0x74, 0x69,
    0x6f, 0x6e, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0e, 0x0a, 0x02, 0x69, 0x64, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x0c, 0x52, 0x02, 0x69, 0x64, 0x12, 0x2b, 0x0a, 0x09, 0x6b, 0x65, 0x79, 0x5f, 0x72,
    0x61, 0x6e, 0x67, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0e, 0x2e, 0x4b, 0x65, 0x79,
    0x52, 0x61, 0x6e, 0x67, 0x65, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x52, 0x08, 0x6b, 0x65, 0x79, 0x52,
    0x61, 0x6e, 0x67, 0x65, 0x12, 0x46, 0x0a, 0x12, 0x70, 0x61, 0x72, 0x74, 0x69, 0x74, 0x69, 0x6f,
    0x6e, 0x5f, 0x6c, 0x6f, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x17, 0x2e, 0x50, 0x61, 0x72, 0x74, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x4c, 0x6f, 0x63, 0x61,
    0x74, 0x69, 0x6f, 0x6e, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x52, 0x11, 0x70, 0x61, 0x72, 0x74, 0x69,
    0x74, 0x69, 0x6f, 0x6e, 0x4c, 0x6f, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x22, 0x45, 0x0a, 0x0d,
    0x4b, 0x65, 0x79, 0x52, 0x61, 0x6e, 0x67, 0x65, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x1b, 0x0a,
    0x09, 0x73, 0x74, 0x61, 0x72, 0x74, 0x5f, 0x6b, 0x65, 0x79, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0c,
    0x52, 0x08, 0x73, 0x74, 0x61, 0x72, 0x74, 0x4b, 0x65, 0x79, 0x12, 0x17, 0x0a, 0x07, 0x65, 0x6e,
    0x64, 0x5f, 0x6b, 0x65, 0x79, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x06, 0x65, 0x6e, 0x64,
    0x4b, 0x65, 0x79, 0x22, 0x48, 0x0a, 0x16, 0x50, 0x61, 0x72, 0x74, 0x69, 0x74, 0x69, 0x6f, 0x6e,
    0x4c, 0x6f, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x1a, 0x0a,
    0x08, 0x68, 0x6f, 0x73, 0x74, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52,
    0x08, 0x68, 0x6f, 0x73, 0x74, 0x6e, 0x61, 0x6d, 0x65, 0x12, 0x12, 0x0a, 0x04, 0x70, 0x6f, 0x72,
    0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x04, 0x70, 0x6f, 0x72, 0x74, 0x32, 0x87, 0x01,
    0x0a, 0x0b, 0x43, 0x6f, 0x6f, 0x72, 0x64, 0x69, 0x6e, 0x61, 0x74, 0x6f, 0x72, 0x12, 0x3b, 0x0a,
    0x0c, 0x63, 0x72, 0x65, 0x61, 0x74, 0x65, 0x5f, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x12, 0x13, 0x2e,
    0x43, 0x72, 0x65, 0x61, 0x74, 0x65, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65,
    0x73, 0x74, 0x1a, 0x14, 0x2e, 0x43, 0x72, 0x65, 0x61, 0x74, 0x65, 0x54, 0x61, 0x62, 0x6c, 0x65,
    0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22, 0x00, 0x12, 0x3b, 0x0a, 0x0c, 0x64, 0x65,
    0x6c, 0x65, 0x74, 0x65, 0x5f, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x12, 0x13, 0x2e, 0x43, 0x72, 0x65,
    0x61, 0x74, 0x65, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a,
    0x14, 0x2e, 0x43, 0x72, 0x65, 0x61, 0x74, 0x65, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x52, 0x65, 0x73,
    0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22, 0x00, 0x4a, 0xa9, 0x08, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00,
    0x22, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x0a, 0x0a, 0x02,
    0x06, 0x00, 0x12, 0x04, 0x02, 0x00, 0x05, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x06, 0x00, 0x01, 0x12,
    0x03, 0x02, 0x08, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x00, 0x12, 0x03, 0x03, 0x04,
    0x4a, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x03, 0x08, 0x14, 0x0a,
    0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x03, 0x16, 0x28, 0x0a, 0x0c, 0x0a,
    0x05, 0x06, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x03, 0x33, 0x46, 0x0a, 0x0b, 0x0a, 0x04, 0x06,
    0x00, 0x02, 0x01, 0x12, 0x03, 0x04, 0x04, 0x4a, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x01,
    0x01, 0x12, 0x03, 0x04, 0x08, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x01, 0x02, 0x12,
    0x03, 0x04, 0x16, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x04,
    0x33, 0x46, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x07, 0x00, 0x09, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x07, 0x08, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x00, 0x12, 0x03, 0x08, 0x04, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04,
    0x12, 0x04, 0x08, 0x04, 0x07, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x06, 0x12,
    0x03, 0x08, 0x04, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x08,
    0x0f, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x08, 0x17, 0x18,
    0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x0b, 0x00, 0x0d, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x01, 0x01, 0x12, 0x03, 0x0b, 0x08, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00,
    0x12, 0x03, 0x0c, 0x04, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x04,
    0x0c, 0x04, 0x0b, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x0c,
    0x04, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0c, 0x09, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0c, 0x13, 0x14, 0x0a, 0x0a,
    0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x0f, 0x00, 0x12, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02,
    0x01, 0x12, 0x03, 0x0f, 0x08, 0x12, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03,
    0x10, 0x04, 0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x04, 0x10, 0x04,
    0x0f, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x05, 0x12, 0x03, 0x10, 0x04, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x10, 0x0b, 0x0f, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x10, 0x12, 0x13, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x11, 0x04, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x01, 0x04, 0x12, 0x03, 0x11, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x06,
    0x12, 0x03, 0x11, 0x0d, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x11, 0x1c, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x03, 0x12, 0x03, 0x11, 0x29,
    0x2a, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x14, 0x00, 0x18, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x14, 0x08, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02,
    0x00, 0x12, 0x03, 0x15, 0x04, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x04, 0x12,
    0x04, 0x15, 0x04, 0x14, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x05, 0x12, 0x03,
    0x15, 0x04, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x15, 0x0a,
    0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03, 0x15, 0x0f, 0x10, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x01, 0x12, 0x03, 0x16, 0x04, 0x20, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x01, 0x04, 0x12, 0x04, 0x16, 0x04, 0x15, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x01, 0x06, 0x12, 0x03, 0x16, 0x04, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x16, 0x12, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x03,
    0x12, 0x03, 0x16, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x02, 0x12, 0x03, 0x17,
    0x04, 0x32, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x04, 0x12, 0x04, 0x17, 0x04, 0x16,
    0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x06, 0x12, 0x03, 0x17, 0x04, 0x1a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x01, 0x12, 0x03, 0x17, 0x1b, 0x2d, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x02, 0x03, 0x12, 0x03, 0x17, 0x30, 0x31, 0x0a, 0x0a, 0x0a, 0x02, 0x04,
    0x04, 0x12, 0x04, 0x1a, 0x00, 0x1d, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01, 0x12, 0x03,
    0x1a, 0x08, 0x15, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x00, 0x12, 0x03, 0x1b, 0x04, 0x18,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x04, 0x12, 0x04, 0x1b, 0x04, 0x1a, 0x17, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x05, 0x12, 0x03, 0x1b, 0x04, 0x09, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1b, 0x0a, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x00, 0x03, 0x12, 0x03, 0x1b, 0x16, 0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02,
    0x01, 0x12, 0x03, 0x1c, 0x04, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x04, 0x12,
    0x04, 0x1c, 0x04, 0x1b, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x05, 0x12, 0x03,
    0x1c, 0x04, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x01, 0x12, 0x03, 0x1c, 0x0a,
    0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x03, 0x12, 0x03, 0x1c, 0x14, 0x15, 0x0a,
    0x0a, 0x0a, 0x02, 0x04, 0x05, 0x12, 0x04, 0x1f, 0x00, 0x22, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x05, 0x01, 0x12, 0x03, 0x1f, 0x08, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x00, 0x12,
    0x03, 0x20, 0x04, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x04, 0x12, 0x04, 0x20,
    0x04, 0x1f, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x05, 0x12, 0x03, 0x20, 0x04,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x01, 0x12, 0x03, 0x20, 0x0b, 0x13, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x03, 0x12, 0x03, 0x20, 0x16, 0x17, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x05, 0x02, 0x01, 0x12, 0x03, 0x21, 0x04, 0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05,
    0x02, 0x01, 0x04, 0x12, 0x04, 0x21, 0x04, 0x20, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x01, 0x05, 0x12, 0x03, 0x21, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x21, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x03, 0x12, 0x03,
    0x21, 0x12, 0x13, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
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
