// Copyright 2024 Natalie Baker // AGPLv3 //

#![warn(
    clippy::all, 
    clippy::pedantic,
    clippy::alloc_instead_of_core,
    clippy::as_underscore,
    clippy::clone_on_ref_ptr,
    clippy::create_dir,
    clippy::empty_structs_with_brackets,
    clippy::error_impl_error,
    clippy::exit,
    clippy::filetype_is_file,
    clippy::fn_to_numeric_cast_any,
    clippy::format_push_string,
    clippy::if_then_some_else_none,
    clippy::mixed_read_write_in_expression,
    clippy::panic_in_result_fn,
    clippy::partial_pub_fields,
    clippy::unseparated_literal_suffix,
    clippy::shadow_unrelated,
    clippy::std_instead_of_core,
    clippy::str_to_string,
    clippy::string_to_string,
    clippy::tests_outside_test_module,
    clippy::undocumented_unsafe_blocks,
    clippy::unnecessary_safety_comment,
    clippy::unnecessary_safety_doc,
    clippy::unwrap_in_result,
    clippy::missing_const_for_fn,
)]

#![allow(
    clippy::missing_docs_in_private_items,
    clippy::module_name_repetitions,
    clippy::cast_possible_truncation,
    clippy::cast_lossless,
    clippy::cast_sign_loss,
    clippy::cast_precision_loss,
    clippy::cast_possible_wrap,
    clippy::missing_errors_doc,
    clippy::needless_pass_by_value, // Bevy systems need this
    clippy::shadow_unrelated,
    // clippy::shadow_unrelated,       // Egui lambda params
)]

pub mod util;
pub mod item;
pub mod track;
pub mod tick;
pub mod power;
pub mod plugin;

pub mod prelude {

    pub use super::plugin::*;
    pub use super::item::*;
    pub use super::track::*;

}