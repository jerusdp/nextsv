//! Semantic Versioning Management
//!
//! Calculates the next semantic version number based on the current version
//! number and the conventional commits that have been made since the
//! last version has been released.
//!
//! Add the dependency to Cargo.toml
//!
//! ```toml
//! [dependencies]
//! nextsv = {version = "0.3.1", features = ["level", "version"] }
//! ```
//!
//! ```no_run
//! # fn main() -> Result<(), nextsv_lib::Error> {
//!     use nextsv_lib::VersionTag;
//!     let version_prefix = "v";
//!
//!     let latest_version = VersionTag::latest(version_prefix)?;
//!     let for_level = latest_version.clone();
//!
//!     let next_version = latest_version.commits()?.next_version();
//!     let next_level = for_level.commits()?.next_level()?;
//!
//!     println!("Next Version: {}\nNext Level: {}", next_version, next_level);
//!
//! #    Ok(())
//! # }
//! ```

mod conventional;
mod error;
mod semantic;
mod version_tag;

pub(crate) use conventional::ConventionalCommits;
pub use error::Error;
pub use semantic::{Level, Semantic};
pub use version_tag::VersionTag;
