#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(
    clippy::try_err,
    clippy::must_use_candidate,
    clippy::doc_markdown,
    clippy::module_name_repetitions,
    clippy::semicolon_if_nothing_returned,
    clippy::shadow_unrelated,
    clippy::missing_errors_doc,
    clippy::similar_names
)]

use anchor_lang::prelude::*;

#[cfg(not(feature = "no-entrypoint"))]
mod entrypoint;

mod error;
pub mod instruction;
mod processor;
pub mod state;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

