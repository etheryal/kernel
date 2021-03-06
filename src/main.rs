// MIT License
//
// Copyright (c) 2021 Miguel Peláez
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

#![allow(dead_code)]
#![feature(
    abi_x86_interrupt,
    custom_test_frameworks,
    default_alloc_error_handler,
    const_mut_refs,
    async_closure,
    alloc_prelude,
    asm,
    once_cell,
    box_syntax,
    const_fn_fn_ptr_basics
)]
#![test_runner(crate::tests::run_all_tests)]
#![reexport_test_harness_main = "test_main"]
#![no_std]
#![no_main]

mod build_info;
mod driver;
mod init;
mod logger;
mod memory;
mod panic;
mod platform;
mod prelude;
mod tasks;
mod tests;
mod wasm;

bootloader::entry_point!(init::main);

extern crate alloc;
