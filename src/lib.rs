#![deny(
    clippy::cargo,
    clippy::nursery,
    clippy::pedantic,
    clippy::restriction,
    rustdoc::missing_crate_level_docs,
    rustdoc::private_doc_tests,
    rustdoc::invalid_html_tags,
    warnings,
    absolute_paths_not_starting_with_crate,
    elided_lifetimes_in_paths,
    explicit_outlives_requirements,
    keyword_idents,
    macro_use_extern_crate,
    meta_variable_misuse,
    missing_abi,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    non_ascii_idents,
    noop_method_call,
    pointer_structural_match,
    rust_2021_incompatible_closure_captures,
    rust_2021_incompatible_or_patterns,
    rust_2021_prefixes_incompatible_syntax,
    rust_2021_prelude_collisions,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unsafe_code,
    unsafe_op_in_unsafe_fn,
    unstable_features,
    unused_crate_dependencies,
    unused_extern_crates,
    unused_import_braces,
    unused_lifetimes,
    unused_macro_rules,
    unused_qualifications,
    variant_size_differences,
    // Nightly lints:
    // rustdoc::missing_doc_code_examples,
    // fuzzy_provenance_casts,
    // lossy_provenance_casts,
    // must_not_suspend,
    // non_exhaustive_omitted_patterns,
)]
#![allow(
    clippy::blanket_clippy_restriction_lints,
    clippy::exhaustive_structs,
    clippy::missing_inline_in_public_items,
    clippy::implicit_return,
    clippy::unwrap_used,
    clippy::multiple_inherent_impl,
    clippy::pattern_type_mismatch,
    clippy::wildcard_enum_match_arm,
    clippy::exhaustive_enums,
    clippy::self_named_module_files,
    clippy::pub_use,
    clippy::else_if_without_else
)]
#![doc = include_str!("../README.md")]

use core::sync::atomic::{AtomicI64, Ordering};

pub use backend::Backend;
pub use cache::Cache;

/// The trait to define how to get and set data in the backend
///
/// This is for adding support for a backend
pub mod backend;
/// The trait providing methods to use the cache
///
/// This is for the users of the cache
pub mod cache;
/// Definitions of cached structs, used when the cached data is different from
/// the event data
pub mod model;
/// Functions related to testing the implementor of [`Cache`]
#[cfg(feature = "tests")]
pub mod tests;

/// Used to create unique IDs when necessary
static ID_COUNTER: AtomicI64 = AtomicI64::new(0);

/// Returns a unique ID by adding 1 to [`ID_COUNTER`]
fn unique_id() -> i64 {
    ID_COUNTER.fetch_add(1, Ordering::Relaxed)
}
