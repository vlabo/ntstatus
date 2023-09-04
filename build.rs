use handlebars::to_json;
use handlebars::Handlebars;
use serde::Serialize;
use serde_json::value::Map;
use std::fs::read_to_string;
use std::fs::File;
use std::io::prelude::*;

const TEMPLATE: &str = r#"
#[cfg(not(feature = "std"))]
use core::fmt::{self, Debug, Display};

#[cfg(feature = "std")]
use std::fmt::{self, Display};

#[derive(Debug, PartialEq)]
#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum NtStatus {
{{#each codes}}
    {{symbol}} = {{value}},
{{/each}}
}

impl NtStatus {
	pub fn from_u32(value: u32) -> Option<NtStatus> {
		match NtStatus::try_from(value) {
			Ok(v) => Some(v),
			Err(_) => None,
		}
	}

	pub fn from_i32(value: i32) -> Option<NtStatus> {
		match NtStatus::try_from(value as u32) {
			Ok(v) => Some(v),
			Err(_) => None,
		}
	}

	pub fn try_from(value: u32) -> Result<NtStatus, ()> {
		match value {
		{{#each codes}}
			{{value}} => Ok(NtStatus::{{symbol}}),
		{{/each}}
			_ => Err(()),
		}
	}
}

impl Display for NtStatus {

	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
		{{#each codes}}
			NtStatus::{{symbol}} => write!(f, "{}", "{{symbol}}"),
		{{/each}}
		}
    }}

"#;

#[derive(Serialize)]
struct StatusCode {
    symbol: String,
    value: String,
}

fn main() {
    // if let Err(_) = std::env::var("CARGO_NTSTATUS_GENERATE") {
    //     return; // Skip generation if var is not set.
    // }

    let reg = Handlebars::new();
    let mut file = File::create("src/ntstatus.rs").unwrap();
    let mut codes = Vec::new();

    for line in read_to_string("ntstatus.txt").unwrap().lines() {
        if line.starts_with("//") {
            continue;
        }
        let code: Vec<&str> = line.split(":").collect();
        if code.len() != 2 {
            continue;
        }
        codes.push(StatusCode {
            symbol: code[0].to_string(),
            value: code[1].to_string(),
        });
    }

    let mut data = Map::new();
    data.insert("codes".to_string(), to_json(&codes));
    file.write_all(reg.render_template(TEMPLATE, &data).unwrap().as_bytes())
        .unwrap();
}
