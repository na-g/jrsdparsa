extern crate serde;
extern crate serde_json;
extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
use std::fmt;

extern crate webrtc_sdp;
use webrtc_sdp::error::SdpParserError;
use webrtc_sdp::SdpSession;
use serde::ser::Serialize;

trait Jsonable {
    fn to_json_string(&self) -> String;
}

impl<T:Serialize> Jsonable for T {
    fn to_json_string(&self) -> String {
        match serde_json::to_string(&self) {
            Err(_) => "".to_string(),
            Ok(json_string) => json_string,
        }
    }
}

#[wasm_bindgen]
pub struct JrParseResult {
    result: String,
    is_err: bool
}

impl From<Result<SdpSession, SdpParserError>> for JrParseResult {
    fn from(result:Result<SdpSession, SdpParserError>) -> Self {
        JrParseResult {
            result: match result {
                Err(ref error) => error.to_json_string(),
                Ok(ref session) => session.to_json_string()
            },
            is_err: result.is_err(),
        }
    }
}

impl fmt::Debug for JrParseResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ParseResult {}", self.result.to_json_string())
    }
}


#[wasm_bindgen]
impl JrParseResult {
    pub fn is_err(&self) -> bool {
        self.is_err
    }
    pub fn result(&self) -> String {
        self.result.clone()
    }
}

#[wasm_bindgen]
pub struct JrSdpParsa {}

#[wasm_bindgen]
impl JrSdpParsa {
    pub fn parse(sdp:&str, fail_on_warning:bool) -> JrParseResult {
        JrParseResult::from(webrtc_sdp::parse_sdp(sdp, fail_on_warning))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
