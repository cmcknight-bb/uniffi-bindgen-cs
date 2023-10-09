/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use paste::paste;
use uniffi_bindgen::backend::{CodeType, Literal, Type};

fn render_literal(literal: &Literal, inner: &Type) -> String {
    match literal {
        Literal::Null => "null".into(),

        // details/1-empty-list-as-default-method-parameter.md
        Literal::EmptySequence => "null".into(),
        Literal::EmptyMap => "null".into(),

        // For optionals
        _ => super::CsCodeOracle.find(inner).literal(literal),
    }
}

macro_rules! impl_code_type_for_compound {
     ($T:ty, $type_label_pattern:literal, $canonical_name_pattern: literal) => {
        paste! {
            #[derive(Debug)]
            pub struct $T {
                inner: Type,
            }

            impl $T {
                pub fn new(inner: Type) -> Self {
                    Self { inner }
                }
                fn inner(&self) -> &Type {
                    &self.inner
                }
            }

            impl CodeType for $T  {
                fn type_label(&self) -> String {
                    format!($type_label_pattern, super::CsCodeOracle.find(self.inner()).type_label())
                }

                fn canonical_name(&self) -> String {
                    format!($canonical_name_pattern, super::CsCodeOracle.find(self.inner()).canonical_name())
                }

                fn literal(&self, literal: &Literal) -> String {
                    render_literal(literal, self.inner())
                }
            }
        }
    }
 }

impl_code_type_for_compound!(OptionalCodeType, "{}?", "Optional{}");
impl_code_type_for_compound!(SequenceCodeType, "List<{}>", "Sequence{}");

#[derive(Debug)]
pub struct MapCodeType {
    key: Type,
    value: Type,
}

impl MapCodeType {
    pub fn new(key: Type, value: Type) -> Self {
        Self { key, value }
    }

    fn key(&self) -> &Type {
        &self.key
    }

    fn value(&self) -> &Type {
        &self.value
    }
}

impl CodeType for MapCodeType {
    fn type_label(&self) -> String {
        format!(
            "Dictionary<{}, {}>",
            super::CsCodeOracle.find(self.key()).type_label(),
            super::CsCodeOracle.find(self.value()).type_label(),
        )
    }

    fn canonical_name(&self) -> String {
        format!(
            "Dictionary{}{}",
            super::CsCodeOracle.find(self.key()).canonical_name(),
            super::CsCodeOracle.find(self.value()).canonical_name(),
        )
    }

    fn literal(&self, literal: &Literal) -> String {
        render_literal(literal, &self.value)
    }
}
