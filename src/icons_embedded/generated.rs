// This wrapper exists so the rest of the crate can `use crate::icons_embedded::...`
// without referencing OUT_DIR paths.

#![allow(non_upper_case_globals)]

include!(concat!(env!("OUT_DIR"), "/material_design_icons.rs"));
