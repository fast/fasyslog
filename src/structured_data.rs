// Copyright 2024 FastLabs Developers
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Implementations of the structured data types (RFC-5424 §6.3).

use std::fmt;

/// A structured data parameter.
///
/// Each SD-PARAM consists of a name, referred to as PARAM-NAME, and a value, referred to as
/// PARAM-VALUE.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive] // only allow construction via the `new` function
pub struct SDParam {
    pub name: String,
    pub value: String,
}

impl SDParam {
    /// Create a new SD-PARAM.
    pub fn new(name: impl Into<String>, value: impl Into<String>) -> Result<Self, String> {
        let name = name.into();
        Self::validate_name(&name)?;
        let value = value.into();
        Ok(Self { name, value })
    }

    /// Return the escaped PARAM-VALUE (RFC-5424 §6.3.3).
    pub fn escape_value(&self) -> String {
        let mut escaped = String::new();
        for c in self.value.chars() {
            if matches!(c, '\\' | '"' | ']') {
                escaped.push('\\');
            }
            escaped.push(c);
        }
        escaped
    }

    // SD-NAME         = 1*32PRINTUSASCII
    //                   ; except '=', SP, ']', %d34 (")
    // SP              = %d32
    // PRINTUSASCII    = %d33-126
    fn validate_name(name: &str) -> Result<(), String> {
        if name.is_empty() {
            return Err("PARAM-NAME must not be empty".to_string());
        }

        if name.len() > 32 {
            return Err(format!(
                "PARAM-NAME must not be less than 32 characters: {name}"
            ));
        }

        for c in name.chars() {
            match c {
                '=' => return Err(format!("PARAM-NAME must not contain '=': {name}")),
                ']' => return Err(format!("PARAM-NAME must not contain ']': {name}")),
                ' ' => return Err(format!("PARAM-NAME must not contain ' ': {name}")),
                '"' => return Err(format!("PARAM-NAME must not contain '\"': {name}")),
                c => {
                    let codepoint = c as u32;
                    if !(33..=126).contains(&codepoint) {
                        return Err(format!(
                            "PARAM-NAME must only contain printable ASCII characters: {name}"
                        ));
                    }
                }
            }
        }

        Ok(())
    }
}

impl fmt::Display for SDParam {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}=\"{}\"", self.name, self.escape_value())
    }
}

/// A structured data element.
///
/// An SD-ELEMENT consists of a name and parameter name-value pairs. The name is referred to as
/// SD-ID. The name-value pairs are referred to as "SD-PARAM".
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SDElement {
    pub id: String,
    params: Vec<SDParam>,
}

impl SDElement {
    /// Create a new SD-ELEMENT.
    pub fn new(id: impl Into<String>) -> Result<Self, String> {
        let id = id.into();
        Self::validate_id(&id)?;
        Ok(Self { id, params: vec![] })
    }

    /// Add a new SD-PARAM to the SD-ELEMENT.
    pub fn add_param(
        &mut self,
        name: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<(), String> {
        let param = SDParam::new(name, value)?;
        // TODO(tisonkun): check PARAM-NAME when SD-ID is registered?
        self.params.push(param);
        Ok(())
    }

    // Registered SD-IDs as documented in RFC-5424 §9.2.
    const fn registered_ids() -> [&'static str; 3] {
        ["timeQuality", "origin", "meta"]
    }

    // SD-ID           = SD-NAME
    // SD-NAME         = 1*32PRINTUSASCII
    //                   ; except '=', SP, ']', %d34 (")
    // SP              = %d32
    // PRINTUSASCII    = %d33-126
    fn validate_id(id: &str) -> Result<(), String> {
        if id.is_empty() {
            return Err("SD-ID must not be empty".to_string());
        }

        if id.len() > 32 {
            return Err(format!("SD-ID must not be less than 32 characters: {id}"));
        }

        let mut has_at_sign = false;
        for c in id.chars() {
            match c {
                '@' => has_at_sign = true,
                '=' => return Err(format!("SD-ID must not contain '=': {id}")),
                ']' => return Err(format!("SD-ID must not contain ']': {id}")),
                ' ' => return Err(format!("SD-ID must not contain ' ': {id}")),
                '"' => return Err(format!("SD-ID must not contain '\"': {id}")),
                c => {
                    let codepoint = c as u32;
                    if !(33..=126).contains(&codepoint) {
                        return Err(format!(
                            "SD-ID must only contain printable ASCII characters: {id}"
                        ));
                    }
                }
            }
        }

        if !has_at_sign && !Self::registered_ids().contains(&id) {
            return Err(format!(
                "SD-ID must contain '@' or be one of the registered IDs: {id}"
            ));
        }

        Ok(())
    }
}

impl fmt::Display for SDElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}", self.id)?;
        for param in &self.params {
            write!(f, " {}", param)?;
        }
        write!(f, "]")
    }
}
