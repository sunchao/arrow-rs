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

use std::slice::Iter;
use arrow_data::Bitmap;
use arrow_array::{BooleanArray, UInt32Array};
use arrow_data::bit_iterator::BitIndexIterator;
use arrow_schema::ArrowError;

/// A selection vector is often used together with a `[Array]` or `[RecordBatch]`, to
/// represent the active rows that are selected so far. It is typically passed along during the
/// evaluation of a expression tree, and updated in-place accordingly.
///
/// TODO: how can we make in-place update of this efficient? when the underlying impl is a vec
/// TODO: how to implement efficient set union and intersect when the underlying impl is a vec?
///   perhaps checkout http://www.vldb.org/pvldb/vol8/p293-inoue.pdf
pub struct SelectionVector {
    inner: SelectionVectorInner,
    // The total number of rows that are selected
    num_rows: usize,
    // Whether all rows are selected
    pub(crate) all_selected: bool,
    // Whether all rows are not selected
    pub(crate) all_not_selected: bool,
}

impl SelectionVector {
    /// Returns an iterator over all row indices of this selection vector.
    pub fn iter(&self) -> impl Iterator<Item = usize> {
        match &self.inner {
            SelectionVectorInner::Bits(bm) => {
                BitIndexIterator::new(bm.buffer().as_slice(), 0, bm.bit_len())
            }
            SelectionVectorInner::Indices(v) => {
                v.iter()
            }
        }
    }

    pub fn intersect(&self, other: &SelectionVector) -> Result<SelectionVector, ArrowError> {
        unimplemented!()
    }
}

/// Actual implementation of selection vector. Specifically, there are two implementations:
///   - BitMap: a bit map containing the same number of elements as the accompanying arrow or batch,
///             with 1 indicating the row at the given index is selected, while 0 otherwise.
///   - Indices: a compact list of indices for the rows that are selected.
enum SelectionVectorInner {
    Bits(Bitmap),
    Indices(Vec<usize>),
}
