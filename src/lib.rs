//! cdr.today poc
//!
//! Quick version
mod app;
mod cli;
pub mod config;
mod middleware;
mod orm;
mod result;
pub mod schema;
mod service;
mod share;

#[macro_use]
extern crate diesel;

pub use self::{
    cli::Opt,
    config::Config,
    result::{Error, Result},
};
