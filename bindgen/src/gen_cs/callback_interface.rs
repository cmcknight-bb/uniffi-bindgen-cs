/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use super::CodeType;
use uniffi_bindgen::{backend::Literal, ComponentInterface};

#[derive(Debug)]
pub struct CallbackInterfaceCodeType {
    id: String,
}

impl CallbackInterfaceCodeType {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}

impl CodeType for CallbackInterfaceCodeType {
    fn type_label(&self, ci: &ComponentInterface) -> String {
        super::CsCodeOracle.class_name(&self.id, ci)
    }

    fn canonical_name(&self) -> String {
        format!("Type{}", self.id)
    }

    fn literal(&self, _literal: &Literal, _ci: &ComponentInterface) -> String {
        unreachable!();
    }

    fn initialization_fn(&self) -> Option<String> {
        super::filters::ffi_callback_registration(&self.id).ok()
    }
}
