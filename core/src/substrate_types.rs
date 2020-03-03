// Copyright 2019 Parity Technologies (UK) Ltd.
// This file is part of substrate-desub.
//
// substrate-desub is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// substrate-desub is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with substrate-desub.  If not, see <http://www.gnu.org/licenses/>.

//! Stucture for registering substrate types

use crate::SetField;
use std::marker::PhantomData;

#[derive(Debug, Clone, PartialEq)]
// a 'stateful' Rust Type marker?
pub enum SubstrateType {
    H512(primitives::H512),
    H256(primitives::H256),
    Composite(Vec<SubstrateType>),

    // Rust Data Primitive Types
    Set(SetField),
    UnitEnum(String),
    Array(Vec<SubstrateType>),
    // Std
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    USize(usize),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    ISize(isize),
    F32(f32),
    F64(f64),
    Bool(bool),
    // not sure what to do with this yet
    Null
}
