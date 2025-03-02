// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

#[cfg(feature = "test_utils")]
pub mod bench_util;
pub mod bit_chunk_iterator;
pub(crate) mod bit_mask;
pub mod bit_util;
#[cfg(feature = "test_utils")]
pub mod data_gen;
pub mod display;
#[cfg(feature = "test_utils")]
pub mod integration_util;
#[cfg(feature = "prettyprint")]
pub mod pretty;
pub(crate) mod serialization;
pub mod string_writer;
#[cfg(feature = "test_utils")]
pub mod test_util;

mod trusted_len;
pub(crate) use trusted_len::trusted_len_unzip;

pub(crate) mod reader_parser;
