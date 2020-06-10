// This file is generated by rust-protobuf 2.8.0. Do not edit
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
//! Generated file from `tensorflow/core/framework/resource_handle.proto`

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_8_0;

#[derive(PartialEq, Clone, Default)]
pub struct ResourceHandleProto {
    // message fields
    pub device: ::std::string::String,
    pub container: ::std::string::String,
    pub name: ::std::string::String,
    pub hash_code: u64,
    pub maybe_type_name: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a ResourceHandleProto {
    fn default() -> &'a ResourceHandleProto {
        <ResourceHandleProto as ::protobuf::Message>::default_instance()
    }
}

impl ResourceHandleProto {
    pub fn new() -> ResourceHandleProto {
        ::std::default::Default::default()
    }

    // string device = 1;

    pub fn get_device(&self) -> &str {
        &self.device
    }
    pub fn clear_device(&mut self) {
        self.device.clear();
    }

    // Param is passed by value, moved
    pub fn set_device(&mut self, v: ::std::string::String) {
        self.device = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_device(&mut self) -> &mut ::std::string::String {
        &mut self.device
    }

    // Take field
    pub fn take_device(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.device, ::std::string::String::new())
    }

    // string container = 2;

    pub fn get_container(&self) -> &str {
        &self.container
    }
    pub fn clear_container(&mut self) {
        self.container.clear();
    }

    // Param is passed by value, moved
    pub fn set_container(&mut self, v: ::std::string::String) {
        self.container = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_container(&mut self) -> &mut ::std::string::String {
        &mut self.container
    }

    // Take field
    pub fn take_container(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.container, ::std::string::String::new())
    }

    // string name = 3;

    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }

    // uint64 hash_code = 4;

    pub fn get_hash_code(&self) -> u64 {
        self.hash_code
    }
    pub fn clear_hash_code(&mut self) {
        self.hash_code = 0;
    }

    // Param is passed by value, moved
    pub fn set_hash_code(&mut self, v: u64) {
        self.hash_code = v;
    }

    // string maybe_type_name = 5;

    pub fn get_maybe_type_name(&self) -> &str {
        &self.maybe_type_name
    }
    pub fn clear_maybe_type_name(&mut self) {
        self.maybe_type_name.clear();
    }

    // Param is passed by value, moved
    pub fn set_maybe_type_name(&mut self, v: ::std::string::String) {
        self.maybe_type_name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_maybe_type_name(&mut self) -> &mut ::std::string::String {
        &mut self.maybe_type_name
    }

    // Take field
    pub fn take_maybe_type_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.maybe_type_name, ::std::string::String::new())
    }
}

impl ::protobuf::Message for ResourceHandleProto {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(
        &mut self,
        is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(
                        wire_type,
                        is,
                        &mut self.device,
                    )?;
                }
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(
                        wire_type,
                        is,
                        &mut self.container,
                    )?;
                }
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(
                        wire_type,
                        is,
                        &mut self.name,
                    )?;
                }
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(
                            wire_type,
                        ));
                    }
                    let tmp = is.read_uint64()?;
                    self.hash_code = tmp;
                }
                5 => {
                    ::protobuf::rt::read_singular_proto3_string_into(
                        wire_type,
                        is,
                        &mut self.maybe_type_name,
                    )?;
                }
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(
                        field_number,
                        wire_type,
                        is,
                        self.mut_unknown_fields(),
                    )?;
                }
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.device.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.device);
        }
        if !self.container.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.container);
        }
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.name);
        }
        if self.hash_code != 0 {
            my_size += ::protobuf::rt::value_size(
                4,
                self.hash_code,
                ::protobuf::wire_format::WireTypeVarint,
            );
        }
        if !self.maybe_type_name.is_empty() {
            my_size += ::protobuf::rt::string_size(5, &self.maybe_type_name);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(
        &self,
        os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        if !self.device.is_empty() {
            os.write_string(1, &self.device)?;
        }
        if !self.container.is_empty() {
            os.write_string(2, &self.container)?;
        }
        if !self.name.is_empty() {
            os.write_string(3, &self.name)?;
        }
        if self.hash_code != 0 {
            os.write_uint64(4, self.hash_code)?;
        }
        if !self.maybe_type_name.is_empty() {
            os.write_string(5, &self.maybe_type_name)?;
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
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> ResourceHandleProto {
        ResourceHandleProto::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> =
            ::protobuf::lazy::Lazy {
                lock: ::protobuf::lazy::ONCE_INIT,
                ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
            };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<
                    _,
                    ::protobuf::types::ProtobufTypeString,
                >(
                    "device",
                    |m: &ResourceHandleProto| &m.device,
                    |m: &mut ResourceHandleProto| &mut m.device,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<
                    _,
                    ::protobuf::types::ProtobufTypeString,
                >(
                    "container",
                    |m: &ResourceHandleProto| &m.container,
                    |m: &mut ResourceHandleProto| &mut m.container,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<
                    _,
                    ::protobuf::types::ProtobufTypeString,
                >(
                    "name",
                    |m: &ResourceHandleProto| &m.name,
                    |m: &mut ResourceHandleProto| &mut m.name,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<
                    _,
                    ::protobuf::types::ProtobufTypeUint64,
                >(
                    "hash_code",
                    |m: &ResourceHandleProto| &m.hash_code,
                    |m: &mut ResourceHandleProto| &mut m.hash_code,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<
                    _,
                    ::protobuf::types::ProtobufTypeString,
                >(
                    "maybe_type_name",
                    |m: &ResourceHandleProto| &m.maybe_type_name,
                    |m: &mut ResourceHandleProto| &mut m.maybe_type_name,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ResourceHandleProto>(
                    "ResourceHandleProto",
                    fields,
                    file_descriptor_proto(),
                )
            })
        }
    }

    fn default_instance() -> &'static ResourceHandleProto {
        static mut instance: ::protobuf::lazy::Lazy<ResourceHandleProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ResourceHandleProto,
        };
        unsafe { instance.get(ResourceHandleProto::new) }
    }
}

impl ::protobuf::Clear for ResourceHandleProto {
    fn clear(&mut self) {
        self.device.clear();
        self.container.clear();
        self.name.clear();
        self.hash_code = 0;
        self.maybe_type_name.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ResourceHandleProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ResourceHandleProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n/tensorflow/core/framework/resource_handle.proto\x12\ntensorflow\"\xa4\
    \x01\n\x13ResourceHandleProto\x12\x16\n\x06device\x18\x01\x20\x01(\tR\
    \x06device\x12\x1c\n\tcontainer\x18\x02\x20\x01(\tR\tcontainer\x12\x12\n\
    \x04name\x18\x03\x20\x01(\tR\x04name\x12\x1b\n\thash_code\x18\x04\x20\
    \x01(\x04R\x08hashCode\x12&\n\x0fmaybe_type_name\x18\x05\x20\x01(\tR\rma\
    ybeTypeNameBn\n\x18org.tensorflow.frameworkB\x0eResourceHandleP\x01Z=git\
    hub.com/tensorflow/tensorflow/tensorflow/go/core/framework\xf8\x01\x01b\
    \x06proto3\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<
    ::protobuf::descriptor::FileDescriptorProto,
> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe { file_descriptor_proto_lazy.get(|| parse_descriptor_proto()) }
}
