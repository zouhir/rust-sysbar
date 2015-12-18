#![allow(non_snake_case)]

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::mem;

// TODO need to figure out how to eliminate these, the macros use them
extern crate objc_foundation;
extern crate libc;

extern crate objc_id;
use objc_id::Id;

#[macro_use]
extern crate objc;
use objc::declare::ClassDecl;
use objc::runtime::{Object, Sel};

extern crate cocoa;
use cocoa::base::{nil, class};
use cocoa::foundation::{NSAutoreleasePool, NSString};
use cocoa::appkit::{NSMenu, NSMenuItem};

#[macro_use]
extern crate barfly;
use barfly::Barfly;

use objc::Message;
use objc_foundation::{INSObject, NSObject};

fn main() {
    let mut fly = Barfly::new("Barfly");

    // make a hash map for the callbacks to mess with
    let hm: HashMap<String, String> = HashMap::new();
    let hm = Arc::new(RwLock::new(hm));

    let phm = hm.clone();
    // the two names "PreferencesCallback" and "PrefCBS" should be unique for this app and different
    // from each other.  Doesn't matter what you call them otherwise.
    add_fly_item!(&fly,
                  "Prefs",
                  PreferencesCallback,
                  PrefCBS,
                  Box::new(move || {
                      let mut hm = phm.write().unwrap();
                      let size = hm.len();
                      let k = format!("Prefs{}", size);
                      hm.insert(k, "Bar".to_owned());

                      println!("prefs selected, new hm {:?}", *hm);
                  }));

    let fhm = hm.clone();
    add_fly_item!(&fly,
                  "Summon Herb",
                  Hcb,
                  Hcbs,
                  Box::new(move || {
                      let mut hm = fhm.write().unwrap();
                      let size = hm.len();
                      let k = format!("Herb{}", size);
                      hm.insert(k, "Bar".to_owned());

                      println!("Herb thinks you are a jerk and refuses to appear.  By the way, \
                                the hash map is: {:?}",
                               *hm);
                  }));

    fly.add_quit_item("Quit");

    fly.display();
}
