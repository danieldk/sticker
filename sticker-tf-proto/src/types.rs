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
//! Generated file from `tensorflow/core/framework/types.proto`

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_7_0;

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum DataType {
    DT_INVALID = 0,
    DT_FLOAT = 1,
    DT_DOUBLE = 2,
    DT_INT32 = 3,
    DT_UINT8 = 4,
    DT_INT16 = 5,
    DT_INT8 = 6,
    DT_STRING = 7,
    DT_COMPLEX64 = 8,
    DT_INT64 = 9,
    DT_BOOL = 10,
    DT_QINT8 = 11,
    DT_QUINT8 = 12,
    DT_QINT32 = 13,
    DT_BFLOAT16 = 14,
    DT_QINT16 = 15,
    DT_QUINT16 = 16,
    DT_UINT16 = 17,
    DT_COMPLEX128 = 18,
    DT_HALF = 19,
    DT_RESOURCE = 20,
    DT_VARIANT = 21,
    DT_UINT32 = 22,
    DT_UINT64 = 23,
    DT_FLOAT_REF = 101,
    DT_DOUBLE_REF = 102,
    DT_INT32_REF = 103,
    DT_UINT8_REF = 104,
    DT_INT16_REF = 105,
    DT_INT8_REF = 106,
    DT_STRING_REF = 107,
    DT_COMPLEX64_REF = 108,
    DT_INT64_REF = 109,
    DT_BOOL_REF = 110,
    DT_QINT8_REF = 111,
    DT_QUINT8_REF = 112,
    DT_QINT32_REF = 113,
    DT_BFLOAT16_REF = 114,
    DT_QINT16_REF = 115,
    DT_QUINT16_REF = 116,
    DT_UINT16_REF = 117,
    DT_COMPLEX128_REF = 118,
    DT_HALF_REF = 119,
    DT_RESOURCE_REF = 120,
    DT_VARIANT_REF = 121,
    DT_UINT32_REF = 122,
    DT_UINT64_REF = 123,
}

impl ::protobuf::ProtobufEnum for DataType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<DataType> {
        match value {
            0 => ::std::option::Option::Some(DataType::DT_INVALID),
            1 => ::std::option::Option::Some(DataType::DT_FLOAT),
            2 => ::std::option::Option::Some(DataType::DT_DOUBLE),
            3 => ::std::option::Option::Some(DataType::DT_INT32),
            4 => ::std::option::Option::Some(DataType::DT_UINT8),
            5 => ::std::option::Option::Some(DataType::DT_INT16),
            6 => ::std::option::Option::Some(DataType::DT_INT8),
            7 => ::std::option::Option::Some(DataType::DT_STRING),
            8 => ::std::option::Option::Some(DataType::DT_COMPLEX64),
            9 => ::std::option::Option::Some(DataType::DT_INT64),
            10 => ::std::option::Option::Some(DataType::DT_BOOL),
            11 => ::std::option::Option::Some(DataType::DT_QINT8),
            12 => ::std::option::Option::Some(DataType::DT_QUINT8),
            13 => ::std::option::Option::Some(DataType::DT_QINT32),
            14 => ::std::option::Option::Some(DataType::DT_BFLOAT16),
            15 => ::std::option::Option::Some(DataType::DT_QINT16),
            16 => ::std::option::Option::Some(DataType::DT_QUINT16),
            17 => ::std::option::Option::Some(DataType::DT_UINT16),
            18 => ::std::option::Option::Some(DataType::DT_COMPLEX128),
            19 => ::std::option::Option::Some(DataType::DT_HALF),
            20 => ::std::option::Option::Some(DataType::DT_RESOURCE),
            21 => ::std::option::Option::Some(DataType::DT_VARIANT),
            22 => ::std::option::Option::Some(DataType::DT_UINT32),
            23 => ::std::option::Option::Some(DataType::DT_UINT64),
            101 => ::std::option::Option::Some(DataType::DT_FLOAT_REF),
            102 => ::std::option::Option::Some(DataType::DT_DOUBLE_REF),
            103 => ::std::option::Option::Some(DataType::DT_INT32_REF),
            104 => ::std::option::Option::Some(DataType::DT_UINT8_REF),
            105 => ::std::option::Option::Some(DataType::DT_INT16_REF),
            106 => ::std::option::Option::Some(DataType::DT_INT8_REF),
            107 => ::std::option::Option::Some(DataType::DT_STRING_REF),
            108 => ::std::option::Option::Some(DataType::DT_COMPLEX64_REF),
            109 => ::std::option::Option::Some(DataType::DT_INT64_REF),
            110 => ::std::option::Option::Some(DataType::DT_BOOL_REF),
            111 => ::std::option::Option::Some(DataType::DT_QINT8_REF),
            112 => ::std::option::Option::Some(DataType::DT_QUINT8_REF),
            113 => ::std::option::Option::Some(DataType::DT_QINT32_REF),
            114 => ::std::option::Option::Some(DataType::DT_BFLOAT16_REF),
            115 => ::std::option::Option::Some(DataType::DT_QINT16_REF),
            116 => ::std::option::Option::Some(DataType::DT_QUINT16_REF),
            117 => ::std::option::Option::Some(DataType::DT_UINT16_REF),
            118 => ::std::option::Option::Some(DataType::DT_COMPLEX128_REF),
            119 => ::std::option::Option::Some(DataType::DT_HALF_REF),
            120 => ::std::option::Option::Some(DataType::DT_RESOURCE_REF),
            121 => ::std::option::Option::Some(DataType::DT_VARIANT_REF),
            122 => ::std::option::Option::Some(DataType::DT_UINT32_REF),
            123 => ::std::option::Option::Some(DataType::DT_UINT64_REF),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [DataType] = &[
            DataType::DT_INVALID,
            DataType::DT_FLOAT,
            DataType::DT_DOUBLE,
            DataType::DT_INT32,
            DataType::DT_UINT8,
            DataType::DT_INT16,
            DataType::DT_INT8,
            DataType::DT_STRING,
            DataType::DT_COMPLEX64,
            DataType::DT_INT64,
            DataType::DT_BOOL,
            DataType::DT_QINT8,
            DataType::DT_QUINT8,
            DataType::DT_QINT32,
            DataType::DT_BFLOAT16,
            DataType::DT_QINT16,
            DataType::DT_QUINT16,
            DataType::DT_UINT16,
            DataType::DT_COMPLEX128,
            DataType::DT_HALF,
            DataType::DT_RESOURCE,
            DataType::DT_VARIANT,
            DataType::DT_UINT32,
            DataType::DT_UINT64,
            DataType::DT_FLOAT_REF,
            DataType::DT_DOUBLE_REF,
            DataType::DT_INT32_REF,
            DataType::DT_UINT8_REF,
            DataType::DT_INT16_REF,
            DataType::DT_INT8_REF,
            DataType::DT_STRING_REF,
            DataType::DT_COMPLEX64_REF,
            DataType::DT_INT64_REF,
            DataType::DT_BOOL_REF,
            DataType::DT_QINT8_REF,
            DataType::DT_QUINT8_REF,
            DataType::DT_QINT32_REF,
            DataType::DT_BFLOAT16_REF,
            DataType::DT_QINT16_REF,
            DataType::DT_QUINT16_REF,
            DataType::DT_UINT16_REF,
            DataType::DT_COMPLEX128_REF,
            DataType::DT_HALF_REF,
            DataType::DT_RESOURCE_REF,
            DataType::DT_VARIANT_REF,
            DataType::DT_UINT32_REF,
            DataType::DT_UINT64_REF,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("DataType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for DataType {
}

impl ::std::default::Default for DataType {
    fn default() -> Self {
        DataType::DT_INVALID
    }
}

impl ::protobuf::reflect::ProtobufValue for DataType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n%tensorflow/core/framework/types.proto\x12\ntensorflow*\xaa\x06\n\x08D\
    ataType\x12\x0e\n\nDT_INVALID\x10\0\x12\x0c\n\x08DT_FLOAT\x10\x01\x12\r\
    \n\tDT_DOUBLE\x10\x02\x12\x0c\n\x08DT_INT32\x10\x03\x12\x0c\n\x08DT_UINT\
    8\x10\x04\x12\x0c\n\x08DT_INT16\x10\x05\x12\x0b\n\x07DT_INT8\x10\x06\x12\
    \r\n\tDT_STRING\x10\x07\x12\x10\n\x0cDT_COMPLEX64\x10\x08\x12\x0c\n\x08D\
    T_INT64\x10\t\x12\x0b\n\x07DT_BOOL\x10\n\x12\x0c\n\x08DT_QINT8\x10\x0b\
    \x12\r\n\tDT_QUINT8\x10\x0c\x12\r\n\tDT_QINT32\x10\r\x12\x0f\n\x0bDT_BFL\
    OAT16\x10\x0e\x12\r\n\tDT_QINT16\x10\x0f\x12\x0e\n\nDT_QUINT16\x10\x10\
    \x12\r\n\tDT_UINT16\x10\x11\x12\x11\n\rDT_COMPLEX128\x10\x12\x12\x0b\n\
    \x07DT_HALF\x10\x13\x12\x0f\n\x0bDT_RESOURCE\x10\x14\x12\x0e\n\nDT_VARIA\
    NT\x10\x15\x12\r\n\tDT_UINT32\x10\x16\x12\r\n\tDT_UINT64\x10\x17\x12\x10\
    \n\x0cDT_FLOAT_REF\x10e\x12\x11\n\rDT_DOUBLE_REF\x10f\x12\x10\n\x0cDT_IN\
    T32_REF\x10g\x12\x10\n\x0cDT_UINT8_REF\x10h\x12\x10\n\x0cDT_INT16_REF\
    \x10i\x12\x0f\n\x0bDT_INT8_REF\x10j\x12\x11\n\rDT_STRING_REF\x10k\x12\
    \x14\n\x10DT_COMPLEX64_REF\x10l\x12\x10\n\x0cDT_INT64_REF\x10m\x12\x0f\n\
    \x0bDT_BOOL_REF\x10n\x12\x10\n\x0cDT_QINT8_REF\x10o\x12\x11\n\rDT_QUINT8\
    _REF\x10p\x12\x11\n\rDT_QINT32_REF\x10q\x12\x13\n\x0fDT_BFLOAT16_REF\x10\
    r\x12\x11\n\rDT_QINT16_REF\x10s\x12\x12\n\x0eDT_QUINT16_REF\x10t\x12\x11\
    \n\rDT_UINT16_REF\x10u\x12\x15\n\x11DT_COMPLEX128_REF\x10v\x12\x0f\n\x0b\
    DT_HALF_REF\x10w\x12\x13\n\x0fDT_RESOURCE_REF\x10x\x12\x12\n\x0eDT_VARIA\
    NT_REF\x10y\x12\x11\n\rDT_UINT32_REF\x10z\x12\x11\n\rDT_UINT64_REF\x10{B\
    k\n\x18org.tensorflow.frameworkB\x0bTypesProtosP\x01Z=github.com/tensorf\
    low/tensorflow/tensorflow/go/core/framework\xf8\x01\x01b\x06proto3\
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
