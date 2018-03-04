#![feature(proc_macro)]

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
use std::fmt;
use std::rc::Rc;

extern crate rsdparsa;
use rsdparsa::error::SdpParserError;
use rsdparsa::SdpSession;

#[wasm_bindgen]
#[derive(Clone)]
pub struct JrSdpParserError { error: Rc<SdpParserError> }

impl From<SdpParserError> for JrSdpParserError {
    fn from(error:SdpParserError) -> Self {
        JrSdpParserError { error: Rc::new(error) }
    }
}

impl fmt::Debug for JrSdpParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.error)
    }
}

#[wasm_bindgen]
impl JrSdpParserError {
    fn get_line_number(&self) -> String {
        match *self.error {
            SdpParserError::Line{ref line_number, ..} => line_number,
            SdpParserError::Unsupported{ref line_number, ..} => line_number,
            SdpParserError::Sequence{ref line_number, ..} => line_number,
        }.to_string()
    }
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct JrSdpSession { session: Rc<SdpSession> }

impl From<SdpSession> for JrSdpSession {
    fn from(session:SdpSession) -> Self {
        JrSdpSession { session: Rc::new(session) }
    }
}

impl fmt::Debug for JrSdpSession {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Session {}", self.session.get_session())
    }
}

#[wasm_bindgen]
impl JrSdpSession {
    pub fn version(&self) -> String {
        return self.session.get_version().to_string()
    }
}

#[wasm_bindgen]
pub struct JrParseResult {
    result: Result<JrSdpSession, JrSdpParserError>
}

impl From<Result<SdpSession, SdpParserError>> for JrParseResult {
    fn from(result:Result<SdpSession, SdpParserError>) -> Self {
        JrParseResult { result: match result {
                Err(error) => Err(JrSdpParserError::from(error)),
                Ok(session) => Ok(JrSdpSession::from(session)),
            }
        }
    }
}

impl fmt::Debug for JrParseResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.result {
            &Err(ref err) => write!(f, "ParseResult {:?}", err),
            &Ok(ref session) => write!(f, "Parse Result {:?}", session),
        }
    }
}


#[wasm_bindgen]
impl JrParseResult {
    pub fn is_err(&self) -> bool {
        self.result.is_err()
    }
    pub fn is_ok(&self) -> bool {
        self.result.is_ok()
    }
    pub fn err(&self) -> JrSdpParserError {
        self.result.as_ref().unwrap_err().clone()
    }
    pub fn ok(&self) -> JrSdpSession {
        self.result.as_ref().unwrap().clone()
    }
}

#[wasm_bindgen]
pub struct JrSdpParsa {}

#[wasm_bindgen]
impl JrSdpParsa {
    pub fn parse(sdp:&str, fail_on_warning:bool) -> JrParseResult {
        JrParseResult::from(rsdparsa::parse_sdp(sdp, fail_on_warning))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
