// DO NOT EDIT
// This file was generated by Stone

#![allow(
    unknown_lints,  // keep rustc from complaining about clippy lints
    too_many_arguments,
    large_enum_variant,
    doc_markdown,
)]

//! This namespace contains common data types used within the users namespace.

pub type AccountId = String;

/// What type of account this user has.
#[derive(Debug)]
pub enum AccountType {
    /// The basic account type.
    Basic,
    /// The Dropbox Pro account type.
    Pro,
    /// The Dropbox Business account type.
    Business,
}

impl<'de> ::serde::de::Deserialize<'de> for AccountType {
    fn deserialize<D: ::serde::de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        // union deserializer
        use serde::de::{self, MapAccess, Visitor};
        struct EnumVisitor;
        impl<'de> Visitor<'de> for EnumVisitor {
            type Value = AccountType;
            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                f.write_str("a AccountType structure")
            }
            fn visit_map<V: MapAccess<'de>>(self, mut map: V) -> Result<Self::Value, V::Error> {
                let tag: &str = match map.next_key()? {
                    Some(".tag") => map.next_value()?,
                    _ => return Err(de::Error::missing_field(".tag"))
                };
                match tag {
                    "basic" => Ok(AccountType::Basic),
                    "pro" => Ok(AccountType::Pro),
                    "business" => Ok(AccountType::Business),
                    _ => Err(de::Error::unknown_variant(tag, VARIANTS))
                }
            }
        }
        const VARIANTS: &'static [&'static str] = &["basic",
                                                    "pro",
                                                    "business"];
        deserializer.deserialize_struct("AccountType", VARIANTS, EnumVisitor)
    }
}

impl ::serde::ser::Serialize for AccountType {
    fn serialize<S: ::serde::ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // union serializer
        use serde::ser::SerializeStruct;
        match *self {
            AccountType::Basic => {
                // unit
                let mut s = serializer.serialize_struct("AccountType", 1)?;
                s.serialize_field(".tag", "basic")?;
                s.end()
            }
            AccountType::Pro => {
                // unit
                let mut s = serializer.serialize_struct("AccountType", 1)?;
                s.serialize_field(".tag", "pro")?;
                s.end()
            }
            AccountType::Business => {
                // unit
                let mut s = serializer.serialize_struct("AccountType", 1)?;
                s.serialize_field(".tag", "business")?;
                s.end()
            }
        }
    }
}

