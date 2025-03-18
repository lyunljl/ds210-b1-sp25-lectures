/*
 * This will make the compiler look for either:
 * 1. src/lib/bar.rs or 
 * 2. src/lib/bar/mod.rs
 */
pub mod bar;

/*
 * This will make the compiler look for either:
 * 1. src/lib/foo.rs or
 * 2. src/lib/foo/mod.rs
 */
pub mod foo;
