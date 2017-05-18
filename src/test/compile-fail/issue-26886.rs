// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::sync::{self, Arc}; //~ NOTE previous import of the type `Arc` here
                            //~^ NOTE previous import of the module `sync` here
use std::sync::Arc; //~ ERROR the name `Arc` is defined multiple times
                    //~| NOTE `Arc` reimported here
                    //~| `Arc` must be defined only once in the type namespace of this module
use std::sync; //~ ERROR the name `sync` is defined multiple times
               //~| NOTE `sync` reimported here
               //~| `sync` must be defined only once in the type namespace of this module

fn main() {
}
