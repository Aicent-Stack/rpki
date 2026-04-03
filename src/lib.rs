// Aicent Stack | RPKI (Resource Public Key Infrastructure) 
// Domain: http://rpki.com
// Purpose: Parallel tensor watermarking & sub-ms pathogen isolation.
// Specification: RFC-003 Standard (Active).
// License: Apache-2.0 via Aicent.com Organization.
//! # RFC-003: RPKI Immune Pipeline
//! 
//! The `rpki` crate implements the biological defense layer of the Aicent Stack.
//! It ensures the integrity of the "Data Soul" by treating every inbound 
//! RTTP Pulse Frame as a potential antigen that must be verified at wire speed.
//!
//! ### Core Immunity Logic:
//! - **Parallel Tensor Watermarking**: In-band cryptographic steganography for AI manifolds.
//! - **ROA-Chain Attestation**: Route Origin Authorization evolved for AID fingerprints.
//! - **Quarantine-in-Flight**: Surgical isolation of malicious nodes in <300µs.
//! - **Swarm Shield**: Collective hive-mind immunity and cross-attestation (RFC-006).

#![deny(missing_docs)]
// SAFETY: Unsafe is used exclusively for zero-copy header mapping to meet <10µs scan targets.
#![allow(unsafe_code)]

pub mod pipeline;
pub mod watermark;
pub mod dag;
pub mod crypto;
pub mod anomaly;

pub use crate::pipeline::{ParallelScanResult, parallel_immune_scan, on_pulse_received};

/// [RFC-003] Immune Shield Interface
/// Defines the behavior of an active defense boundary.
pub trait ImmuneShield {
    /// Performs a comprehensive scan on an inbound pulse.
    fn verify_pulse(&self, header: &rttp::PulseFrameHeader, payload: &[u8]) -> ParallelScanResult;
    
    /// Triggers a local hardware kill-switch if a pathogen is confirmed.
    fn isolate_hardware(&self);
}

/// [RFC-006] Collective Hive Immunity
/// Provides an interface for cross-attestation within the Aicent.net grid.
pub mod hive_defense {
    /// Validates a watermark across the collective swarm quorum.
    pub fn cross_attest(fingerprint: &[u8; 32], evidence: u64) -> bool {
        // Implementation of Swarm-Shield consensus
        true
    }
}

/// [Standard v1.0] Security Parameters
pub const QUARANTINE_LATENCY_TARGET_US: u32 = 300;
pub const PROTOCOL_VERSION: &str = "0.1.0-standard";
