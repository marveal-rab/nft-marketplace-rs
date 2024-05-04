use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AddRequest {
    // Write minimal output. Required: no.
    pub quiet: Option<bool>,
    // Write only final hash. Required: no.
    pub quieter: Option<bool>,
    // Write no output. Required: no.
    pub silent: Option<bool>,
    // Stream progress data. Required: no.
    pub progress: Option<bool>,
    // Use trickle-dag format for dag generation. Required: no.
    pub trickle: Option<bool>,
    // Only chunk and hash - do not write to disk. Required: no.
    pub only_hash: Option<bool>,
    // Wrap files with a directory object. Required: no.
    pub wrap_with_directory: Option<bool>,
    // Chunking algorithm, size-[bytes], rabin-[min]-[avg]-[max] or buzhash. Default: size-262144. Required: no.
    pub chunker: Option<String>,
    // Use raw blocks for leaf nodes. Required: no.
    pub raw_leaves: Option<bool>,
    // Add the file using filestore. Implies raw-leaves. (experimental). Required: no.
    pub nocopy: Option<bool>,
    // Check the filestore for pre-existing blocks. (experimental). Required: no.
    pub fscache: Option<bool>,
    // CID version. Defaults to 0 unless an option that depends on CIDv1 is passed. Passing version 1 will cause the raw-leaves option to default to true. Required: no.
    pub cid_version: Option<i32>,
    // Hash function to use. Implies CIDv1 if not sha2-256. (experimental). Default: sha2-256. Required: no.
    pub hash: Option<String>,
    // Inline small blocks into CIDs. (experimental). Required: no.
    pub inline: Option<bool>,
    // Maximum block size to inline. (experimental). Default: 32. Required: no.
    pub inline_limit: Option<i32>,
    // Pin locally to protect added files from garbage collection. Default: true. Required: no.
    pub pin: Option<bool>,
    // Add reference to Files API (MFS) at the provided path. Required: no.
    pub to_files: Option<String>,
}

impl crate::request::Request for AddRequest {
    const PATH: &'static str = "add";
}
