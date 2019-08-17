// This file is generated by rust-protobuf 2.7.0. Do not edit
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
//! Generated file from `tensorflow/core/framework/tensor_description.proto`

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_7_0;

#[derive(PartialEq,Clone,Default)]
pub struct TensorDescription {
    // message fields
    pub dtype: super::types::DataType,
    pub shape: ::protobuf::SingularPtrField<super::tensor_shape::TensorShapeProto>,
    pub allocation_description: ::protobuf::SingularPtrField<super::allocation_description::AllocationDescription>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a TensorDescription {
    fn default() -> &'a TensorDescription {
        <TensorDescription as ::protobuf::Message>::default_instance()
    }
}

impl TensorDescription {
    pub fn new() -> TensorDescription {
        ::std::default::Default::default()
    }

    // .tensorflow.DataType dtype = 1;


    pub fn get_dtype(&self) -> super::types::DataType {
        self.dtype
    }
    pub fn clear_dtype(&mut self) {
        self.dtype = super::types::DataType::DT_INVALID;
    }

    // Param is passed by value, moved
    pub fn set_dtype(&mut self, v: super::types::DataType) {
        self.dtype = v;
    }

    // .tensorflow.TensorShapeProto shape = 2;


    pub fn get_shape(&self) -> &super::tensor_shape::TensorShapeProto {
        self.shape.as_ref().unwrap_or_else(|| super::tensor_shape::TensorShapeProto::default_instance())
    }
    pub fn clear_shape(&mut self) {
        self.shape.clear();
    }

    pub fn has_shape(&self) -> bool {
        self.shape.is_some()
    }

    // Param is passed by value, moved
    pub fn set_shape(&mut self, v: super::tensor_shape::TensorShapeProto) {
        self.shape = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_shape(&mut self) -> &mut super::tensor_shape::TensorShapeProto {
        if self.shape.is_none() {
            self.shape.set_default();
        }
        self.shape.as_mut().unwrap()
    }

    // Take field
    pub fn take_shape(&mut self) -> super::tensor_shape::TensorShapeProto {
        self.shape.take().unwrap_or_else(|| super::tensor_shape::TensorShapeProto::new())
    }

    // .tensorflow.AllocationDescription allocation_description = 4;


    pub fn get_allocation_description(&self) -> &super::allocation_description::AllocationDescription {
        self.allocation_description.as_ref().unwrap_or_else(|| super::allocation_description::AllocationDescription::default_instance())
    }
    pub fn clear_allocation_description(&mut self) {
        self.allocation_description.clear();
    }

    pub fn has_allocation_description(&self) -> bool {
        self.allocation_description.is_some()
    }

    // Param is passed by value, moved
    pub fn set_allocation_description(&mut self, v: super::allocation_description::AllocationDescription) {
        self.allocation_description = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_allocation_description(&mut self) -> &mut super::allocation_description::AllocationDescription {
        if self.allocation_description.is_none() {
            self.allocation_description.set_default();
        }
        self.allocation_description.as_mut().unwrap()
    }

    // Take field
    pub fn take_allocation_description(&mut self) -> super::allocation_description::AllocationDescription {
        self.allocation_description.take().unwrap_or_else(|| super::allocation_description::AllocationDescription::new())
    }
}

impl ::protobuf::Message for TensorDescription {
    fn is_initialized(&self) -> bool {
        for v in &self.shape {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.allocation_description {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.dtype, 1, &mut self.unknown_fields)?
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.shape)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.allocation_description)?;
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
        if self.dtype != super::types::DataType::DT_INVALID {
            my_size += ::protobuf::rt::enum_size(1, self.dtype);
        }
        if let Some(ref v) = self.shape.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.allocation_description.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.dtype != super::types::DataType::DT_INVALID {
            os.write_enum(1, self.dtype.value())?;
        }
        if let Some(ref v) = self.shape.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.allocation_description.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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

    fn new() -> TensorDescription {
        TensorDescription::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::types::DataType>>(
                    "dtype",
                    |m: &TensorDescription| { &m.dtype },
                    |m: &mut TensorDescription| { &mut m.dtype },
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::tensor_shape::TensorShapeProto>>(
                    "shape",
                    |m: &TensorDescription| { &m.shape },
                    |m: &mut TensorDescription| { &mut m.shape },
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::allocation_description::AllocationDescription>>(
                    "allocation_description",
                    |m: &TensorDescription| { &m.allocation_description },
                    |m: &mut TensorDescription| { &mut m.allocation_description },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TensorDescription>(
                    "TensorDescription",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static TensorDescription {
        static mut instance: ::protobuf::lazy::Lazy<TensorDescription> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TensorDescription,
        };
        unsafe {
            instance.get(TensorDescription::new)
        }
    }
}

impl ::protobuf::Clear for TensorDescription {
    fn clear(&mut self) {
        self.dtype = super::types::DataType::DT_INVALID;
        self.shape.clear();
        self.allocation_description.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TensorDescription {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TensorDescription {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n2tensorflow/core/framework/tensor_description.proto\x12\ntensorflow\
    \x1a%tensorflow/core/framework/types.proto\x1a,tensorflow/core/framework\
    /tensor_shape.proto\x1a6tensorflow/core/framework/allocation_description\
    .proto\"\xcd\x01\n\x11TensorDescription\x12*\n\x05dtype\x18\x01\x20\x01(\
    \x0e2\x14.tensorflow.DataTypeR\x05dtype\x122\n\x05shape\x18\x02\x20\x01(\
    \x0b2\x1c.tensorflow.TensorShapeProtoR\x05shape\x12X\n\x16allocation_des\
    cription\x18\x04\x20\x01(\x0b2!.tensorflow.AllocationDescriptionR\x15all\
    ocationDescriptionBw\n\x18org.tensorflow.frameworkB\x17TensorDescription\
    ProtosP\x01Z=github.com/tensorflow/tensorflow/tensorflow/go/core/framewo\
    rk\xf8\x01\x01b\x06proto3\
";

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
