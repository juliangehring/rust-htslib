// Copyright 2014 Christopher Schröder, Johannes Köster.
// Licensed under the MIT license (http://opensource.org/licenses/MIT)
// This file may not be copied, modified, or distributed
// except according to those terms.

//! Rust-Htslib provides a high level API to working with the common HTS file formats.
//!
//! Htslib itself is the *de facto* standard implementation for reading and writing files for
//! HTS alignments (SAM and BAM) as well as variant calls in VCF and BCF format.
//!
//! For example, reading and writing BAM files is as easy as
//!
//! ```
//! use rust_htslib::bam;
//! use rust_htslib::prelude::*;
//!
//! let mut bam = bam::Reader::from_path(&"test/test.bam").unwrap();
//! let header = bam::Header::from_template(bam.header());
//! let mut out = bam::Writer::from_path(&"test/out.bam", &header).unwrap();
//!
//! // copy reverse reads to new BAM file
//! for r in bam.records() {
//!     let record = r.unwrap();
//!     if record.is_reverse() {
//!         out.write(&record).unwrap();
//!     }
//! }
//! ```
//!
//! Pileups can be performed with
//!
//! ```
//! use rust_htslib::bam;
//! use rust_htslib::prelude::*;
//!
//! let mut bam = bam::Reader::from_path(&"test/test.bam").unwrap();
//!
//! // pileup over all covered sites
//! for p in bam.pileup() {
//!     let pileup = p.unwrap();
//!     println!("{}:{} depth {}", pileup.tid(), pileup.pos(), pileup.depth());
//!
//!     for alignment in pileup.alignments() {
//!         if !alignment.is_del() && !alignment.is_refskip() {
//!             println!("Base {}", alignment.record().seq()[alignment.qpos().unwrap()]);
//!         }
//!         // mark indel start
//!         match alignment.indel() {
//!             bam::pileup::Indel::Ins(len) => println!("Insertion of length {} between this and next position.", len),
//!             bam::pileup::Indel::Del(len) => println!("Deletion of length {} between this and next position.", len),
//!             bam::pileup::Indel::None => ()
//!         }
//!     }
//! }
//! ```
//!
//! In both cases, indexed BAM files can be seeked for specific regions, constraining either the record iterator or the pileups:
//!
//! ```
//! use rust_htslib::bam;
//! use rust_htslib::prelude::*;
//!
//! let mut bam = bam::IndexedReader::from_path(&"test/test.bam").unwrap();
//!
//! // seek to chr1:50000-100000
//! let tid = bam.header().tid(b"CHROMOSOME_I").unwrap();
//! bam.fetch(tid, 0, 20).unwrap();
//! // afterwards, read or pileup in this region
//! ```

extern crate bitflags;
#[macro_use]
extern crate custom_derive;
extern crate ieee754;
extern crate itertools;
#[macro_use]
extern crate lazy_static;
extern crate libc;
#[macro_use]
extern crate newtype_derive;
#[macro_use]
extern crate quick_error;
extern crate regex;
extern crate url;

extern crate linear_map;

#[cfg(feature = "serde")]
extern crate serde;

#[cfg(all(test, feature = "serde"))]
extern crate bincode;

#[cfg(test)] // <-- not needed in examples + integration tests
#[macro_use]
extern crate pretty_assertions;
#[cfg(all(test, feature = "serde"))]
extern crate serde_json;

extern crate bio_types;

extern crate cloudflare_zlib_sys;

#[cfg(feature = "bzip2")]
extern crate bzip2_sys;
#[cfg(feature = "lzma")]
extern crate lzma_sys;

pub mod bam;
pub mod bcf;
pub mod htslib;
pub mod prelude;
pub mod sam;
pub mod tbx;
pub mod utils;
