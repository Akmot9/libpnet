// Copyright (c) 2014, 2015 Robert Clipsham <robert@octarineparrot.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! A Profinet real time packet abstraction.

use pnet_macros::packet;
use pnet_macros_support::types::*;

/// The profinet header.
#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
pub mod Ipv4Flags {
    use pnet_macros_support::types::*;

    /// Don't Fragment flag.
    pub const DontFragment: u3 = 0b010;
    /// More Fragments flag.
    pub const MoreFragments: u3 = 0b001;
}