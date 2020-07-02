#![allow(nonstandard_style)]
#![deny(missing_copy_implementations, missing_debug_implementations)]
#![cfg_attr(doc, feature(external_doc))]
#![cfg_attr(doc, doc(include = "../README.md"))]

#[cfg(not(all(target_arch = "x86", windows)))]
compile_error!("The only supported platform is Windows 32-bit");

#[cfg(feature = "dsp")]
pub mod dsp;
#[cfg(feature = "gen")]
pub mod gen;
#[cfg(feature = "in")]
pub mod in2;
#[cfg(feature = "ipc-pe")]
pub mod ipc_pe;
#[cfg(feature = "out")]
pub mod out;
#[cfg(feature = "wa-dlg")]
pub mod wa_dlg;
pub mod wa_ipc;
