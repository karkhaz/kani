// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// rmc-verify-fail

pub fn test_offset_in_double_array() {
    //let table: Vec<Vec<u64>> = Vec::with_capacity(1);
    let table: [[u64; 1]; 1] = [[rmc::nondet::<u64>()]];
    table[0][rmc::nondet::<usize>()]; // EXPCECTED FAIL
}

pub fn main() {
    test_offset_in_double_array();
}