use crate::{
    error::Error,
    request::{QueryParam, WithForm},
};
use ipfs_api_derive::QueryParam;
use reqwest::multipart::{Form, Part};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct ChcidQuery {
    // Path to change. Default: '/'. Required: no.
    pub arg: Option<String>,
    // Cid version to use. (experimental). Required: no.
    pub cid_version: Option<i32>,
    // Hash function to use. Will set Cid version to 1 if used. (experimental). Required: no.
    pub hash: Option<String>,
}

pub struct ChcidRequest {
    pub query: ChcidQuery,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CpQuery {
    // Source IPFS or MFS path to copy. Required: yes.
    #[serde(rename = "arg")]
    pub source: String,
    // Destination within MFS. Required: yes.
    #[serde(rename = "arg")]
    pub dest: String,
    // Make parent directories as needed. Required: no.
    pub parents: Option<bool>,
}

pub struct CpRequest {
    pub query: CpQuery,
}

#[derive(Serialize, Deserialize, Debug, QueryParam, Default)]
pub struct FlushQuery {
    // Path to flush. Default: '/'. Required: no.
    pub arg: Option<String>,
}

pub struct FlushRequest {
    pub query: &'static FlushQuery,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LsQuery {
    // Path to show listing for. Defaults to '/'. Required: no.
    pub arg: Option<String>,
    // Use long listing format. Required: no.
    pub long: Option<bool>,
    // Do not sort; list entries in directory order. Required: no.
    #[serde(rename = "U")]
    pub u: Option<bool>,
}

pub struct LsRequest {
    pub query: LsQuery,
}

#[derive(Serialize, Deserialize, Debug, Default, QueryParam)]
#[serde(rename_all = "kebab-case")]
pub struct MkdirQuery {
    // Path to dir to make. Required: yes.
    pub arg: String,
    // No error if existing, make parent directories as needed. Required: no.
    pub parents: Option<bool>,
    // Cid version to use. (experimental). Required: no.
    pub cid_version: Option<i32>,
    // Hash function to use. Will set Cid version to 1 if used. (experimental). Required: no.
    pub hash: Option<String>,
}

impl MkdirQuery {
    pub fn new_with_arg(arg: &String) -> Self {
        let mut arg = arg.clone();
        if !arg.starts_with("/") {
            arg = format!("/{}", arg);
        }

        Self {
            arg: arg.clone(),
            parents: Some(true),
            cid_version: None,
            hash: None,
        }
    }
}

pub struct MkdirRequest {
    pub query: MkdirQuery,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MvQuery {
    // Source file to move. Required: yes.
    #[serde(rename = "arg")]
    pub source: String,
    // Destination path for file to be moved to. Required: yes.
    #[serde(rename = "arg")]
    pub dest: String,
}

pub struct MvRequest {
    pub query: MvQuery,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReadQuery {
    // Path to file to be read. Required: yes.
    pub arg: String,
    // Byte offset to begin reading from. Required: no.
    pub offset: Option<i64>,
    // Maximum number of bytes to read. Required: no.
    pub count: Option<i64>,
}

pub struct ReadRequest {
    pub query: ReadQuery,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RmQuery {
    // File to remove. Required: yes.
    pub arg: String,
    // Recursively remove directories. Required: no.
    pub recursive: Option<bool>,
    // Forcibly remove target at path; implies -r for directories. Required: no.
    pub force: Option<bool>,
}

pub struct RmRequest {
    pub query: RmQuery,
}

#[derive(Serialize, Deserialize, Debug, QueryParam, Default)]
#[serde(rename_all = "kebab-case")]
pub struct StatQuery {
    // Path to node to stat. Required: yes.
    pub arg: String,
    // Print statistics in given format. Allowed tokens: <hash> <size> <cumulsize> <type> <childs>. Conflicts with other format options. Default: <hash> Size: <size> CumulativeSize: <cumulsize> ChildBlocks: <childs> Type: <type>. Default: <hash> Size: <size> CumulativeSize: <cumulsize> ChildBlocks: <childs> Type: <type>. Required: no.
    pub format: Option<String>,
    // Print only hash. Implies '--format=<hash>'. Conflicts with other format options. Required: no.
    pub hash: Option<bool>,
    // Print only size. Implies '--format=<cumulsize>'. Conflicts with other format options. Required: no.
    pub size: Option<bool>,
    // Compute the amount of the dag that is local, and if possible the total size. Required: no.
    pub with_local: Option<bool>,
}

impl StatQuery {
    pub fn new_with_arg(arg: &String) -> Self {
        let mut arg = arg.clone();
        if !arg.starts_with("/") {
            arg = format!("/{}", arg);
        }

        Self {
            arg: arg.clone(),
            format: None,
            hash: None,
            size: None,
            with_local: None,
        }
    }
}

pub struct StatRequest {
    pub query: StatQuery,
}

#[derive(Serialize, Deserialize, Debug, QueryParam, Default)]
#[serde(rename_all = "kebab-case")]
pub struct WriteQuery {
    // Path to write to. Required: yes.
    pub arg: String,
    // Byte offset to begin writing at. Required: no.
    pub offset: Option<i64>,
    // Create the file if it does not exist. Required: no.
    pub create: Option<bool>,
    // Make parent directories as needed. Required: no.
    pub parents: Option<bool>,
    // Truncate the file to size zero before writing. Required: no.
    pub truncate: Option<bool>,
    // Maximum number of bytes to read. Required: no.
    pub count: Option<i64>,
    // Use raw blocks for newly created leaf nodes. (experimental). Required: no.
    pub raw_leaves: Option<bool>,
    // Cid version to use. (experimental). Required: no.
    pub cid_version: Option<i32>,
    // Hash function to use. Will set Cid version to 1 if used. (experimental). Required: no.
    pub hash: Option<String>,
}

impl WriteQuery {
    pub fn new_with_arg(arg: String) -> Self {
        let mut arg = arg.clone();
        if !arg.starts_with("/") {
            arg = format!("/{}", arg);
        }
        Self {
            arg,
            offset: None,
            create: Some(true),
            parents: None,
            truncate: None,
            count: None,
            raw_leaves: None,
            cid_version: None,
            hash: None,
        }
    }
}

pub struct WriteRequest {
    pub query: WriteQuery,
    pub bytes: Vec<u8>,
    pub filename: String,
}

impl WithForm for WriteRequest {
    fn form(&self) -> Result<Form, Error> {
        let form = Form::new();
        let b = self.bytes.clone();
        let f = self.filename.clone();
        let form = form.part("data", Part::bytes(b).file_name(f));
        Ok(form)
    }
}
