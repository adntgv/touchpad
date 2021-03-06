// This file is generated by rust-protobuf 2.3.0. Do not edit
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

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct Packet {
    // message fields
    pub messageType: MessageType,
    pub touchpad: ::protobuf::SingularPtrField<Touchpad>,
    pub accelerometer: ::protobuf::SingularPtrField<Accelerometer>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl Packet {
    pub fn new() -> Packet {
        ::std::default::Default::default()
    }

    // .MessageType messageType = 1;

    pub fn clear_messageType(&mut self) {
        self.messageType = MessageType::NONE;
    }

    // Param is passed by value, moved
    pub fn set_messageType(&mut self, v: MessageType) {
        self.messageType = v;
    }

    pub fn get_messageType(&self) -> MessageType {
        self.messageType
    }

    // .Touchpad touchpad = 2;

    pub fn clear_touchpad(&mut self) {
        self.touchpad.clear();
    }

    pub fn has_touchpad(&self) -> bool {
        self.touchpad.is_some()
    }

    // Param is passed by value, moved
    pub fn set_touchpad(&mut self, v: Touchpad) {
        self.touchpad = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_touchpad(&mut self) -> &mut Touchpad {
        if self.touchpad.is_none() {
            self.touchpad.set_default();
        }
        self.touchpad.as_mut().unwrap()
    }

    // Take field
    pub fn take_touchpad(&mut self) -> Touchpad {
        self.touchpad.take().unwrap_or_else(|| Touchpad::new())
    }

    pub fn get_touchpad(&self) -> &Touchpad {
        self.touchpad.as_ref().unwrap_or_else(|| Touchpad::default_instance())
    }

    // .Accelerometer accelerometer = 3;

    pub fn clear_accelerometer(&mut self) {
        self.accelerometer.clear();
    }

    pub fn has_accelerometer(&self) -> bool {
        self.accelerometer.is_some()
    }

    // Param is passed by value, moved
    pub fn set_accelerometer(&mut self, v: Accelerometer) {
        self.accelerometer = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_accelerometer(&mut self) -> &mut Accelerometer {
        if self.accelerometer.is_none() {
            self.accelerometer.set_default();
        }
        self.accelerometer.as_mut().unwrap()
    }

    // Take field
    pub fn take_accelerometer(&mut self) -> Accelerometer {
        self.accelerometer.take().unwrap_or_else(|| Accelerometer::new())
    }

    pub fn get_accelerometer(&self) -> &Accelerometer {
        self.accelerometer.as_ref().unwrap_or_else(|| Accelerometer::default_instance())
    }
}

impl ::protobuf::Message for Packet {
    fn is_initialized(&self) -> bool {
        for v in &self.touchpad {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.accelerometer {
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
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.messageType, 1, &mut self.unknown_fields)?
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.touchpad)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.accelerometer)?;
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
        if self.messageType != MessageType::NONE {
            my_size += ::protobuf::rt::enum_size(1, self.messageType);
        }
        if let Some(ref v) = self.touchpad.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.accelerometer.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.messageType != MessageType::NONE {
            os.write_enum(1, self.messageType.value())?;
        }
        if let Some(ref v) = self.touchpad.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.accelerometer.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> Packet {
        Packet::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<MessageType>>(
                    "messageType",
                    |m: &Packet| { &m.messageType },
                    |m: &mut Packet| { &mut m.messageType },
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Touchpad>>(
                    "touchpad",
                    |m: &Packet| { &m.touchpad },
                    |m: &mut Packet| { &mut m.touchpad },
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Accelerometer>>(
                    "accelerometer",
                    |m: &Packet| { &m.accelerometer },
                    |m: &mut Packet| { &mut m.accelerometer },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Packet>(
                    "Packet",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static Packet {
        static mut instance: ::protobuf::lazy::Lazy<Packet> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Packet,
        };
        unsafe {
            instance.get(Packet::new)
        }
    }
}

impl ::protobuf::Clear for Packet {
    fn clear(&mut self) {
        self.clear_messageType();
        self.clear_touchpad();
        self.clear_accelerometer();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Packet {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Packet {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Touchpad {
    // message fields
    pub action: Touchpad_Action,
    pub x: ::std::vec::Vec<f64>,
    pub y: ::std::vec::Vec<f64>,
    pub eventTime: i64,
    pub downTime: i64,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl Touchpad {
    pub fn new() -> Touchpad {
        ::std::default::Default::default()
    }

    // .Touchpad.Action action = 1;

    pub fn clear_action(&mut self) {
        self.action = Touchpad_Action::NONE;
    }

    // Param is passed by value, moved
    pub fn set_action(&mut self, v: Touchpad_Action) {
        self.action = v;
    }

    pub fn get_action(&self) -> Touchpad_Action {
        self.action
    }

    // repeated double x = 2;

    pub fn clear_x(&mut self) {
        self.x.clear();
    }

    // Param is passed by value, moved
    pub fn set_x(&mut self, v: ::std::vec::Vec<f64>) {
        self.x = v;
    }

    // Mutable pointer to the field.
    pub fn mut_x(&mut self) -> &mut ::std::vec::Vec<f64> {
        &mut self.x
    }

    // Take field
    pub fn take_x(&mut self) -> ::std::vec::Vec<f64> {
        ::std::mem::replace(&mut self.x, ::std::vec::Vec::new())
    }

    pub fn get_x(&self) -> &[f64] {
        &self.x
    }

    // repeated double y = 3;

    pub fn clear_y(&mut self) {
        self.y.clear();
    }

    // Param is passed by value, moved
    pub fn set_y(&mut self, v: ::std::vec::Vec<f64>) {
        self.y = v;
    }

    // Mutable pointer to the field.
    pub fn mut_y(&mut self) -> &mut ::std::vec::Vec<f64> {
        &mut self.y
    }

    // Take field
    pub fn take_y(&mut self) -> ::std::vec::Vec<f64> {
        ::std::mem::replace(&mut self.y, ::std::vec::Vec::new())
    }

    pub fn get_y(&self) -> &[f64] {
        &self.y
    }

    // int64 eventTime = 4;

    pub fn clear_eventTime(&mut self) {
        self.eventTime = 0;
    }

    // Param is passed by value, moved
    pub fn set_eventTime(&mut self, v: i64) {
        self.eventTime = v;
    }

    pub fn get_eventTime(&self) -> i64 {
        self.eventTime
    }

    // int64 downTime = 5;

    pub fn clear_downTime(&mut self) {
        self.downTime = 0;
    }

    // Param is passed by value, moved
    pub fn set_downTime(&mut self, v: i64) {
        self.downTime = v;
    }

    pub fn get_downTime(&self) -> i64 {
        self.downTime
    }
}

impl ::protobuf::Message for Touchpad {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.action, 1, &mut self.unknown_fields)?
                },
                2 => {
                    ::protobuf::rt::read_repeated_double_into(wire_type, is, &mut self.x)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_double_into(wire_type, is, &mut self.y)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.eventTime = tmp;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.downTime = tmp;
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
        if self.action != Touchpad_Action::NONE {
            my_size += ::protobuf::rt::enum_size(1, self.action);
        }
        my_size += 9 * self.x.len() as u32;
        my_size += 9 * self.y.len() as u32;
        if self.eventTime != 0 {
            my_size += ::protobuf::rt::value_size(4, self.eventTime, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.downTime != 0 {
            my_size += ::protobuf::rt::value_size(5, self.downTime, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.action != Touchpad_Action::NONE {
            os.write_enum(1, self.action.value())?;
        }
        for v in &self.x {
            os.write_double(2, *v)?;
        };
        for v in &self.y {
            os.write_double(3, *v)?;
        };
        if self.eventTime != 0 {
            os.write_int64(4, self.eventTime)?;
        }
        if self.downTime != 0 {
            os.write_int64(5, self.downTime)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> Touchpad {
        Touchpad::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Touchpad_Action>>(
                    "action",
                    |m: &Touchpad| { &m.action },
                    |m: &mut Touchpad| { &mut m.action },
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "x",
                    |m: &Touchpad| { &m.x },
                    |m: &mut Touchpad| { &mut m.x },
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "y",
                    |m: &Touchpad| { &m.y },
                    |m: &mut Touchpad| { &mut m.y },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "eventTime",
                    |m: &Touchpad| { &m.eventTime },
                    |m: &mut Touchpad| { &mut m.eventTime },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "downTime",
                    |m: &Touchpad| { &m.downTime },
                    |m: &mut Touchpad| { &mut m.downTime },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Touchpad>(
                    "Touchpad",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static Touchpad {
        static mut instance: ::protobuf::lazy::Lazy<Touchpad> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Touchpad,
        };
        unsafe {
            instance.get(Touchpad::new)
        }
    }
}

impl ::protobuf::Clear for Touchpad {
    fn clear(&mut self) {
        self.clear_action();
        self.clear_x();
        self.clear_y();
        self.clear_eventTime();
        self.clear_downTime();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Touchpad {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Touchpad {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Touchpad_Action {
    NONE = 0,
    DOWN = 1,
    UP = 2,
    MOVE = 3,
}

impl ::protobuf::ProtobufEnum for Touchpad_Action {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Touchpad_Action> {
        match value {
            0 => ::std::option::Option::Some(Touchpad_Action::NONE),
            1 => ::std::option::Option::Some(Touchpad_Action::DOWN),
            2 => ::std::option::Option::Some(Touchpad_Action::UP),
            3 => ::std::option::Option::Some(Touchpad_Action::MOVE),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Touchpad_Action] = &[
            Touchpad_Action::NONE,
            Touchpad_Action::DOWN,
            Touchpad_Action::UP,
            Touchpad_Action::MOVE,
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
                ::protobuf::reflect::EnumDescriptor::new("Touchpad_Action", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Touchpad_Action {
}

impl ::std::default::Default for Touchpad_Action {
    fn default() -> Self {
        Touchpad_Action::NONE
    }
}

impl ::protobuf::reflect::ProtobufValue for Touchpad_Action {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Accelerometer {
    // message fields
    pub acc_x: f64,
    pub acc_y: f64,
    pub acc_z: f64,
    pub rotation: i32,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl Accelerometer {
    pub fn new() -> Accelerometer {
        ::std::default::Default::default()
    }

    // double acc_x = 1;

    pub fn clear_acc_x(&mut self) {
        self.acc_x = 0.;
    }

    // Param is passed by value, moved
    pub fn set_acc_x(&mut self, v: f64) {
        self.acc_x = v;
    }

    pub fn get_acc_x(&self) -> f64 {
        self.acc_x
    }

    // double acc_y = 2;

    pub fn clear_acc_y(&mut self) {
        self.acc_y = 0.;
    }

    // Param is passed by value, moved
    pub fn set_acc_y(&mut self, v: f64) {
        self.acc_y = v;
    }

    pub fn get_acc_y(&self) -> f64 {
        self.acc_y
    }

    // double acc_z = 3;

    pub fn clear_acc_z(&mut self) {
        self.acc_z = 0.;
    }

    // Param is passed by value, moved
    pub fn set_acc_z(&mut self, v: f64) {
        self.acc_z = v;
    }

    pub fn get_acc_z(&self) -> f64 {
        self.acc_z
    }

    // int32 rotation = 4;

    pub fn clear_rotation(&mut self) {
        self.rotation = 0;
    }

    // Param is passed by value, moved
    pub fn set_rotation(&mut self, v: i32) {
        self.rotation = v;
    }

    pub fn get_rotation(&self) -> i32 {
        self.rotation
    }
}

impl ::protobuf::Message for Accelerometer {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.acc_x = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.acc_y = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.acc_z = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.rotation = tmp;
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
        if self.acc_x != 0. {
            my_size += 9;
        }
        if self.acc_y != 0. {
            my_size += 9;
        }
        if self.acc_z != 0. {
            my_size += 9;
        }
        if self.rotation != 0 {
            my_size += ::protobuf::rt::value_size(4, self.rotation, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.acc_x != 0. {
            os.write_double(1, self.acc_x)?;
        }
        if self.acc_y != 0. {
            os.write_double(2, self.acc_y)?;
        }
        if self.acc_z != 0. {
            os.write_double(3, self.acc_z)?;
        }
        if self.rotation != 0 {
            os.write_int32(4, self.rotation)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> Accelerometer {
        Accelerometer::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "acc_x",
                    |m: &Accelerometer| { &m.acc_x },
                    |m: &mut Accelerometer| { &mut m.acc_x },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "acc_y",
                    |m: &Accelerometer| { &m.acc_y },
                    |m: &mut Accelerometer| { &mut m.acc_y },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "acc_z",
                    |m: &Accelerometer| { &m.acc_z },
                    |m: &mut Accelerometer| { &mut m.acc_z },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "rotation",
                    |m: &Accelerometer| { &m.rotation },
                    |m: &mut Accelerometer| { &mut m.rotation },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Accelerometer>(
                    "Accelerometer",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static Accelerometer {
        static mut instance: ::protobuf::lazy::Lazy<Accelerometer> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Accelerometer,
        };
        unsafe {
            instance.get(Accelerometer::new)
        }
    }
}

impl ::protobuf::Clear for Accelerometer {
    fn clear(&mut self) {
        self.clear_acc_x();
        self.clear_acc_y();
        self.clear_acc_z();
        self.clear_rotation();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Accelerometer {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Accelerometer {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum MessageType {
    NONE = 0,
    TOUCHPAD = 1,
    ACCELEROMETER = 2,
}

impl ::protobuf::ProtobufEnum for MessageType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<MessageType> {
        match value {
            0 => ::std::option::Option::Some(MessageType::NONE),
            1 => ::std::option::Option::Some(MessageType::TOUCHPAD),
            2 => ::std::option::Option::Some(MessageType::ACCELEROMETER),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [MessageType] = &[
            MessageType::NONE,
            MessageType::TOUCHPAD,
            MessageType::ACCELEROMETER,
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
                ::protobuf::reflect::EnumDescriptor::new("MessageType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for MessageType {
}

impl ::std::default::Default for MessageType {
    fn default() -> Self {
        MessageType::NONE
    }
}

impl ::protobuf::reflect::ProtobufValue for MessageType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0etouchpad.proto\x12\0\"{\n\x06Packet\x12%\n\x0bmessageType\x18\x01\
    \x20\x01(\x0e2\x0c.MessageTypeB\x02\x18\0\x12\x1f\n\x08touchpad\x18\x02\
    \x20\x01(\x0b2\t.TouchpadB\x02\x18\0\x12)\n\raccelerometer\x18\x03\x20\
    \x01(\x0b2\x0e.AccelerometerB\x02\x18\0\"\xaf\x01\n\x08Touchpad\x12$\n\
    \x06action\x18\x01\x20\x01(\x0e2\x10.Touchpad.ActionB\x02\x18\0\x12\r\n\
    \x01x\x18\x02\x20\x03(\x01B\x02\x18\0\x12\r\n\x01y\x18\x03\x20\x03(\x01B\
    \x02\x18\0\x12\x15\n\teventTime\x18\x04\x20\x01(\x03B\x02\x18\0\x12\x14\
    \n\x08downTime\x18\x05\x20\x01(\x03B\x02\x18\0\"2\n\x06Action\x12\x08\n\
    \x04NONE\x10\0\x12\x08\n\x04DOWN\x10\x01\x12\x06\n\x02UP\x10\x02\x12\x08\
    \n\x04MOVE\x10\x03\x1a\x02\x10\0\"^\n\rAccelerometer\x12\x11\n\x05acc_x\
    \x18\x01\x20\x01(\x01B\x02\x18\0\x12\x11\n\x05acc_y\x18\x02\x20\x01(\x01\
    B\x02\x18\0\x12\x11\n\x05acc_z\x18\x03\x20\x01(\x01B\x02\x18\0\x12\x14\n\
    \x08rotation\x18\x04\x20\x01(\x05B\x02\x18\0*<\n\x0bMessageType\x12\x08\
    \n\x04NONE\x10\0\x12\x0c\n\x08TOUCHPAD\x10\x01\x12\x11\n\rACCELEROMETER\
    \x10\x02\x1a\x02\x10\0B\0b\x06proto3\
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
