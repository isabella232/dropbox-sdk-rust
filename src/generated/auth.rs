// DO NOT EDIT
// This file was generated by Stone

#![allow(
    unknown_lints,  // keep rustc from complaining about clippy lints
    too_many_arguments,
    large_enum_variant,
    doc_markdown,
)]

/// Creates an OAuth 2.0 access token from the supplied OAuth 1.0 access token.
pub fn token_from_oauth1(client: &::client_trait::HttpClient, arg: &TokenFromOAuth1Arg) -> ::Result<Result<TokenFromOAuth1Result, TokenFromOAuth1Error>> {
    ::client_helpers::request(client, ::client_trait::Endpoint::Api, "auth/token/from_oauth1", arg, None)
}

/// Disables the access token used to authenticate the call.
pub fn token_revoke(client: &::client_trait::HttpClient, arg: &()) -> ::Result<Result<(), ()>> {
    ::client_helpers::request(client, ::client_trait::Endpoint::Api, "auth/token/revoke", arg, None)
}

/// Errors occurred during authentication.
#[derive(Debug)]
pub enum AuthError {
    /// The access token is invalid.
    InvalidAccessToken,
    /// The user specified in 'Dropbox-API-Select-User' is no longer on the team.
    InvalidSelectUser,
    /// The user specified in 'Dropbox-API-Select-Admin' is not a Dropbox Business team admin.
    InvalidSelectAdmin,
    /// The user has been suspended.
    UserSuspended,
    Other,
}

impl<'de> ::serde::de::Deserialize<'de> for AuthError {
    fn deserialize<D: ::serde::de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        // union deserializer
        use serde::de::{self, MapAccess, Visitor};
        struct EnumVisitor;
        impl<'de> Visitor<'de> for EnumVisitor {
            type Value = AuthError;
            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                f.write_str("a AuthError structure")
            }
            fn visit_map<V: MapAccess<'de>>(self, mut map: V) -> Result<Self::Value, V::Error> {
                let tag: &str = match map.next_key()? {
                    Some(".tag") => map.next_value()?,
                    _ => return Err(de::Error::missing_field(".tag"))
                };
                match tag {
                    "invalid_access_token" => Ok(AuthError::InvalidAccessToken),
                    "invalid_select_user" => Ok(AuthError::InvalidSelectUser),
                    "invalid_select_admin" => Ok(AuthError::InvalidSelectAdmin),
                    "user_suspended" => Ok(AuthError::UserSuspended),
                    _ => Ok(AuthError::Other)
                }
            }
        }
        const VARIANTS: &'static [&'static str] = &["invalid_access_token",
                                                    "invalid_select_user",
                                                    "invalid_select_admin",
                                                    "user_suspended",
                                                    "other"];
        deserializer.deserialize_struct("AuthError", VARIANTS, EnumVisitor)
    }
}

impl ::serde::ser::Serialize for AuthError {
    fn serialize<S: ::serde::ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // union serializer
        use serde::ser::SerializeStruct;
        match *self {
            AuthError::InvalidAccessToken => {
                // unit
                let mut s = serializer.serialize_struct("AuthError", 1)?;
                s.serialize_field(".tag", "invalid_access_token")?;
                s.end()
            }
            AuthError::InvalidSelectUser => {
                // unit
                let mut s = serializer.serialize_struct("AuthError", 1)?;
                s.serialize_field(".tag", "invalid_select_user")?;
                s.end()
            }
            AuthError::InvalidSelectAdmin => {
                // unit
                let mut s = serializer.serialize_struct("AuthError", 1)?;
                s.serialize_field(".tag", "invalid_select_admin")?;
                s.end()
            }
            AuthError::UserSuspended => {
                // unit
                let mut s = serializer.serialize_struct("AuthError", 1)?;
                s.serialize_field(".tag", "user_suspended")?;
                s.end()
            }
            AuthError::Other => Err(::serde::ser::Error::custom("cannot serialize 'Other' variant"))
        }
    }
}

impl ::std::error::Error for AuthError {
    fn description(&self) -> &str {
        "AuthError"
    }
}

impl ::std::fmt::Display for AuthError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{:?}", *self)
    }
}

/// Error occurred because the app is being rate limited.
#[derive(Debug)]
pub struct RateLimitError {
    /// The reason why the app is being rate limited.
    pub reason: RateLimitReason,
    /// The number of seconds that the app should wait before making another request.
    pub retry_after: u64,
}

impl RateLimitError {
    pub fn new(reason: RateLimitReason) -> Self {
        RateLimitError {
            reason,
            retry_after: 1,
        }
    }

    pub fn with_retry_after(mut self, value: u64) -> Self {
        self.retry_after = value;
        self
    }

}

const RATE_LIMIT_ERROR_FIELDS: &'static [&'static str] = &["reason",
                                                           "retry_after"];
impl RateLimitError {
    pub(crate) fn internal_deserialize<'de, V: ::serde::de::MapAccess<'de>>(mut map: V) -> Result<RateLimitError, V::Error> {
        use serde::de;
        let mut field_reason = None;
        let mut field_retry_after = None;
        while let Some(key) = map.next_key()? {
            match key {
                "reason" => {
                    if field_reason.is_some() {
                        return Err(de::Error::duplicate_field("reason"));
                    }
                    field_reason = Some(map.next_value()?);
                }
                "retry_after" => {
                    if field_retry_after.is_some() {
                        return Err(de::Error::duplicate_field("retry_after"));
                    }
                    field_retry_after = Some(map.next_value()?);
                }
                _ => return Err(de::Error::unknown_field(key, RATE_LIMIT_ERROR_FIELDS))
            }
        }
        Ok(RateLimitError {
            reason: field_reason.ok_or_else(|| de::Error::missing_field("reason"))?,
            retry_after: field_retry_after.unwrap_or(1),
        })
    }

    pub(crate) fn internal_serialize<S: ::serde::ser::Serializer>(&self, s: &mut S::SerializeStruct) -> Result<(), S::Error> {
        use serde::ser::SerializeStruct;
        s.serialize_field("reason", &self.reason)?;
        s.serialize_field("retry_after", &self.retry_after)
    }
}

impl<'de> ::serde::de::Deserialize<'de> for RateLimitError {
    fn deserialize<D: ::serde::de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        // struct deserializer
        use serde::de::{MapAccess, Visitor};
        struct StructVisitor;
        impl<'de> Visitor<'de> for StructVisitor {
            type Value = RateLimitError;
            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                f.write_str("a RateLimitError struct")
            }
            fn visit_map<V: MapAccess<'de>>(self, map: V) -> Result<Self::Value, V::Error> {
                RateLimitError::internal_deserialize(map)
            }
        }
        deserializer.deserialize_struct("RateLimitError", RATE_LIMIT_ERROR_FIELDS, StructVisitor)
    }
}

impl ::serde::ser::Serialize for RateLimitError {
    fn serialize<S: ::serde::ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // struct serializer
        use serde::ser::SerializeStruct;
        let mut s = serializer.serialize_struct("RateLimitError", 2)?;
        self.internal_serialize::<S>(&mut s)?;
        s.end()
    }
}

/// Error occurred because the account doesn't have permission to access the resource.
#[derive(Debug)]
pub enum AccessError {
    /// Current account type cannot access the resource.
    InvalidAccountType(InvalidAccountTypeError),
    /// Current account cannot access Paper.
    PaperAccessDenied(PaperAccessError),
    Other,
}

impl<'de> ::serde::de::Deserialize<'de> for AccessError {
    fn deserialize<D: ::serde::de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        // union deserializer
        use serde::de::{self, MapAccess, Visitor};
        struct EnumVisitor;
        impl<'de> Visitor<'de> for EnumVisitor {
            type Value = AccessError;
            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                f.write_str("a AccessError structure")
            }
            fn visit_map<V: MapAccess<'de>>(self, mut map: V) -> Result<Self::Value, V::Error> {
                let tag: &str = match map.next_key()? {
                    Some(".tag") => map.next_value()?,
                    _ => return Err(de::Error::missing_field(".tag"))
                };
                match tag {
                    "invalid_account_type" => {
                        if map.next_key()? != Some("invalid_account_type") {
                            return Err(de::Error::missing_field("invalid_account_type"));
                        }
                        Ok(AccessError::InvalidAccountType(map.next_value()?))
                    }
                    "paper_access_denied" => {
                        if map.next_key()? != Some("paper_access_denied") {
                            return Err(de::Error::missing_field("paper_access_denied"));
                        }
                        Ok(AccessError::PaperAccessDenied(map.next_value()?))
                    }
                    _ => Ok(AccessError::Other)
                }
            }
        }
        const VARIANTS: &'static [&'static str] = &["invalid_account_type",
                                                    "paper_access_denied",
                                                    "other"];
        deserializer.deserialize_struct("AccessError", VARIANTS, EnumVisitor)
    }
}

impl ::serde::ser::Serialize for AccessError {
    fn serialize<S: ::serde::ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // union serializer
        use serde::ser::SerializeStruct;
        match *self {
            AccessError::InvalidAccountType(ref x) => {
                // union or polymporphic struct
                let mut s = serializer.serialize_struct("{}", 2)?;
                s.serialize_field(".tag", "invalid_account_type")?;
                s.serialize_field("invalid_account_type", x)?;
                s.end()
            }
            AccessError::PaperAccessDenied(ref x) => {
                // union or polymporphic struct
                let mut s = serializer.serialize_struct("{}", 2)?;
                s.serialize_field(".tag", "paper_access_denied")?;
                s.serialize_field("paper_access_denied", x)?;
                s.end()
            }
            AccessError::Other => Err(::serde::ser::Error::custom("cannot serialize 'Other' variant"))
        }
    }
}

impl ::std::error::Error for AccessError {
    fn description(&self) -> &str {
        "AccessError"
    }
}

impl ::std::fmt::Display for AccessError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{:?}", *self)
    }
}

#[derive(Debug)]
pub struct TokenFromOAuth1Arg {
    /// The supplied OAuth 1.0 access token.
    pub oauth1_token: String,
    /// The token secret associated with the supplied access token.
    pub oauth1_token_secret: String,
}

impl TokenFromOAuth1Arg {
    pub fn new(oauth1_token: String, oauth1_token_secret: String) -> Self {
        TokenFromOAuth1Arg {
            oauth1_token,
            oauth1_token_secret,
        }
    }

}

const TOKEN_FROM_O_AUTH1_ARG_FIELDS: &'static [&'static str] = &["oauth1_token",
                                                                 "oauth1_token_secret"];
impl TokenFromOAuth1Arg {
    pub(crate) fn internal_deserialize<'de, V: ::serde::de::MapAccess<'de>>(mut map: V) -> Result<TokenFromOAuth1Arg, V::Error> {
        use serde::de;
        let mut field_oauth1_token = None;
        let mut field_oauth1_token_secret = None;
        while let Some(key) = map.next_key()? {
            match key {
                "oauth1_token" => {
                    if field_oauth1_token.is_some() {
                        return Err(de::Error::duplicate_field("oauth1_token"));
                    }
                    field_oauth1_token = Some(map.next_value()?);
                }
                "oauth1_token_secret" => {
                    if field_oauth1_token_secret.is_some() {
                        return Err(de::Error::duplicate_field("oauth1_token_secret"));
                    }
                    field_oauth1_token_secret = Some(map.next_value()?);
                }
                _ => return Err(de::Error::unknown_field(key, TOKEN_FROM_O_AUTH1_ARG_FIELDS))
            }
        }
        Ok(TokenFromOAuth1Arg {
            oauth1_token: field_oauth1_token.ok_or_else(|| de::Error::missing_field("oauth1_token"))?,
            oauth1_token_secret: field_oauth1_token_secret.ok_or_else(|| de::Error::missing_field("oauth1_token_secret"))?,
        })
    }

    pub(crate) fn internal_serialize<S: ::serde::ser::Serializer>(&self, s: &mut S::SerializeStruct) -> Result<(), S::Error> {
        use serde::ser::SerializeStruct;
        s.serialize_field("oauth1_token", &self.oauth1_token)?;
        s.serialize_field("oauth1_token_secret", &self.oauth1_token_secret)
    }
}

impl<'de> ::serde::de::Deserialize<'de> for TokenFromOAuth1Arg {
    fn deserialize<D: ::serde::de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        // struct deserializer
        use serde::de::{MapAccess, Visitor};
        struct StructVisitor;
        impl<'de> Visitor<'de> for StructVisitor {
            type Value = TokenFromOAuth1Arg;
            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                f.write_str("a TokenFromOAuth1Arg struct")
            }
            fn visit_map<V: MapAccess<'de>>(self, map: V) -> Result<Self::Value, V::Error> {
                TokenFromOAuth1Arg::internal_deserialize(map)
            }
        }
        deserializer.deserialize_struct("TokenFromOAuth1Arg", TOKEN_FROM_O_AUTH1_ARG_FIELDS, StructVisitor)
    }
}

impl ::serde::ser::Serialize for TokenFromOAuth1Arg {
    fn serialize<S: ::serde::ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // struct serializer
        use serde::ser::SerializeStruct;
        let mut s = serializer.serialize_struct("TokenFromOAuth1Arg", 2)?;
        self.internal_serialize::<S>(&mut s)?;
        s.end()
    }
}

#[derive(Debug)]
pub enum InvalidAccountTypeError {
    /// Current account type doesn't have permission to access this route endpoint.
    Endpoint,
    /// Current account type doesn't have permission to access this feature.
    Feature,
    Other,
}

impl<'de> ::serde::de::Deserialize<'de> for InvalidAccountTypeError {
    fn deserialize<D: ::serde::de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        // union deserializer
        use serde::de::{self, MapAccess, Visitor};
        struct EnumVisitor;
        impl<'de> Visitor<'de> for EnumVisitor {
            type Value = InvalidAccountTypeError;
            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                f.write_str("a InvalidAccountTypeError structure")
            }
            fn visit_map<V: MapAccess<'de>>(self, mut map: V) -> Result<Self::Value, V::Error> {
                let tag: &str = match map.next_key()? {
                    Some(".tag") => map.next_value()?,
                    _ => return Err(de::Error::missing_field(".tag"))
                };
                match tag {
                    "endpoint" => Ok(InvalidAccountTypeError::Endpoint),
                    "feature" => Ok(InvalidAccountTypeError::Feature),
                    _ => Ok(InvalidAccountTypeError::Other)
                }
            }
        }
        const VARIANTS: &'static [&'static str] = &["endpoint",
                                                    "feature",
                                                    "other"];
        deserializer.deserialize_struct("InvalidAccountTypeError", VARIANTS, EnumVisitor)
    }
}

impl ::serde::ser::Serialize for InvalidAccountTypeError {
    fn serialize<S: ::serde::ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // union serializer
        use serde::ser::SerializeStruct;
        match *self {
            InvalidAccountTypeError::Endpoint => {
                // unit
                let mut s = serializer.serialize_struct("InvalidAccountTypeError", 1)?;
                s.serialize_field(".tag", "endpoint")?;
                s.end()
            }
            InvalidAccountTypeError::Feature => {
                // unit
                let mut s = serializer.serialize_struct("InvalidAccountTypeError", 1)?;
                s.serialize_field(".tag", "feature")?;
                s.end()
            }
            InvalidAccountTypeError::Other => Err(::serde::ser::Error::custom("cannot serialize 'Other' variant"))
        }
    }
}

impl ::std::error::Error for InvalidAccountTypeError {
    fn description(&self) -> &str {
        "InvalidAccountTypeError"
    }
}

impl ::std::fmt::Display for InvalidAccountTypeError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{:?}", *self)
    }
}

#[derive(Debug)]
pub enum RateLimitReason {
    /// You are making too many requests in the past few minutes.
    TooManyRequests,
    /// There are currently too many write operations happening in the user's Dropbox.
    TooManyWriteOperations,
    Other,
}

impl<'de> ::serde::de::Deserialize<'de> for RateLimitReason {
    fn deserialize<D: ::serde::de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        // union deserializer
        use serde::de::{self, MapAccess, Visitor};
        struct EnumVisitor;
        impl<'de> Visitor<'de> for EnumVisitor {
            type Value = RateLimitReason;
            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                f.write_str("a RateLimitReason structure")
            }
            fn visit_map<V: MapAccess<'de>>(self, mut map: V) -> Result<Self::Value, V::Error> {
                let tag: &str = match map.next_key()? {
                    Some(".tag") => map.next_value()?,
                    _ => return Err(de::Error::missing_field(".tag"))
                };
                match tag {
                    "too_many_requests" => Ok(RateLimitReason::TooManyRequests),
                    "too_many_write_operations" => Ok(RateLimitReason::TooManyWriteOperations),
                    _ => Ok(RateLimitReason::Other)
                }
            }
        }
        const VARIANTS: &'static [&'static str] = &["too_many_requests",
                                                    "too_many_write_operations",
                                                    "other"];
        deserializer.deserialize_struct("RateLimitReason", VARIANTS, EnumVisitor)
    }
}

impl ::serde::ser::Serialize for RateLimitReason {
    fn serialize<S: ::serde::ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // union serializer
        use serde::ser::SerializeStruct;
        match *self {
            RateLimitReason::TooManyRequests => {
                // unit
                let mut s = serializer.serialize_struct("RateLimitReason", 1)?;
                s.serialize_field(".tag", "too_many_requests")?;
                s.end()
            }
            RateLimitReason::TooManyWriteOperations => {
                // unit
                let mut s = serializer.serialize_struct("RateLimitReason", 1)?;
                s.serialize_field(".tag", "too_many_write_operations")?;
                s.end()
            }
            RateLimitReason::Other => Err(::serde::ser::Error::custom("cannot serialize 'Other' variant"))
        }
    }
}

#[derive(Debug)]
pub enum TokenFromOAuth1Error {
    /// Part or all of the OAuth 1.0 access token info is invalid.
    InvalidOauth1TokenInfo,
    /// The authorized app does not match the app associated with the supplied access token.
    AppIdMismatch,
    Other,
}

impl<'de> ::serde::de::Deserialize<'de> for TokenFromOAuth1Error {
    fn deserialize<D: ::serde::de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        // union deserializer
        use serde::de::{self, MapAccess, Visitor};
        struct EnumVisitor;
        impl<'de> Visitor<'de> for EnumVisitor {
            type Value = TokenFromOAuth1Error;
            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                f.write_str("a TokenFromOAuth1Error structure")
            }
            fn visit_map<V: MapAccess<'de>>(self, mut map: V) -> Result<Self::Value, V::Error> {
                let tag: &str = match map.next_key()? {
                    Some(".tag") => map.next_value()?,
                    _ => return Err(de::Error::missing_field(".tag"))
                };
                match tag {
                    "invalid_oauth1_token_info" => Ok(TokenFromOAuth1Error::InvalidOauth1TokenInfo),
                    "app_id_mismatch" => Ok(TokenFromOAuth1Error::AppIdMismatch),
                    _ => Ok(TokenFromOAuth1Error::Other)
                }
            }
        }
        const VARIANTS: &'static [&'static str] = &["invalid_oauth1_token_info",
                                                    "app_id_mismatch",
                                                    "other"];
        deserializer.deserialize_struct("TokenFromOAuth1Error", VARIANTS, EnumVisitor)
    }
}

impl ::serde::ser::Serialize for TokenFromOAuth1Error {
    fn serialize<S: ::serde::ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // union serializer
        use serde::ser::SerializeStruct;
        match *self {
            TokenFromOAuth1Error::InvalidOauth1TokenInfo => {
                // unit
                let mut s = serializer.serialize_struct("TokenFromOAuth1Error", 1)?;
                s.serialize_field(".tag", "invalid_oauth1_token_info")?;
                s.end()
            }
            TokenFromOAuth1Error::AppIdMismatch => {
                // unit
                let mut s = serializer.serialize_struct("TokenFromOAuth1Error", 1)?;
                s.serialize_field(".tag", "app_id_mismatch")?;
                s.end()
            }
            TokenFromOAuth1Error::Other => Err(::serde::ser::Error::custom("cannot serialize 'Other' variant"))
        }
    }
}

impl ::std::error::Error for TokenFromOAuth1Error {
    fn description(&self) -> &str {
        "TokenFromOAuth1Error"
    }
}

impl ::std::fmt::Display for TokenFromOAuth1Error {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{:?}", *self)
    }
}

#[derive(Debug)]
pub enum PaperAccessError {
    /// Paper is disabled.
    PaperDisabled,
    /// The provided user has not used Paper yet.
    NotPaperUser,
    Other,
}

impl<'de> ::serde::de::Deserialize<'de> for PaperAccessError {
    fn deserialize<D: ::serde::de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        // union deserializer
        use serde::de::{self, MapAccess, Visitor};
        struct EnumVisitor;
        impl<'de> Visitor<'de> for EnumVisitor {
            type Value = PaperAccessError;
            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                f.write_str("a PaperAccessError structure")
            }
            fn visit_map<V: MapAccess<'de>>(self, mut map: V) -> Result<Self::Value, V::Error> {
                let tag: &str = match map.next_key()? {
                    Some(".tag") => map.next_value()?,
                    _ => return Err(de::Error::missing_field(".tag"))
                };
                match tag {
                    "paper_disabled" => Ok(PaperAccessError::PaperDisabled),
                    "not_paper_user" => Ok(PaperAccessError::NotPaperUser),
                    _ => Ok(PaperAccessError::Other)
                }
            }
        }
        const VARIANTS: &'static [&'static str] = &["paper_disabled",
                                                    "not_paper_user",
                                                    "other"];
        deserializer.deserialize_struct("PaperAccessError", VARIANTS, EnumVisitor)
    }
}

impl ::serde::ser::Serialize for PaperAccessError {
    fn serialize<S: ::serde::ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // union serializer
        use serde::ser::SerializeStruct;
        match *self {
            PaperAccessError::PaperDisabled => {
                // unit
                let mut s = serializer.serialize_struct("PaperAccessError", 1)?;
                s.serialize_field(".tag", "paper_disabled")?;
                s.end()
            }
            PaperAccessError::NotPaperUser => {
                // unit
                let mut s = serializer.serialize_struct("PaperAccessError", 1)?;
                s.serialize_field(".tag", "not_paper_user")?;
                s.end()
            }
            PaperAccessError::Other => Err(::serde::ser::Error::custom("cannot serialize 'Other' variant"))
        }
    }
}

impl ::std::error::Error for PaperAccessError {
    fn description(&self) -> &str {
        "PaperAccessError"
    }
}

impl ::std::fmt::Display for PaperAccessError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{:?}", *self)
    }
}

#[derive(Debug)]
pub struct TokenFromOAuth1Result {
    /// The OAuth 2.0 token generated from the supplied OAuth 1.0 token.
    pub oauth2_token: String,
}

impl TokenFromOAuth1Result {
    pub fn new(oauth2_token: String) -> Self {
        TokenFromOAuth1Result {
            oauth2_token,
        }
    }

}

const TOKEN_FROM_O_AUTH1_RESULT_FIELDS: &'static [&'static str] = &["oauth2_token"];
impl TokenFromOAuth1Result {
    pub(crate) fn internal_deserialize<'de, V: ::serde::de::MapAccess<'de>>(mut map: V) -> Result<TokenFromOAuth1Result, V::Error> {
        use serde::de;
        let mut field_oauth2_token = None;
        while let Some(key) = map.next_key()? {
            match key {
                "oauth2_token" => {
                    if field_oauth2_token.is_some() {
                        return Err(de::Error::duplicate_field("oauth2_token"));
                    }
                    field_oauth2_token = Some(map.next_value()?);
                }
                _ => return Err(de::Error::unknown_field(key, TOKEN_FROM_O_AUTH1_RESULT_FIELDS))
            }
        }
        Ok(TokenFromOAuth1Result {
            oauth2_token: field_oauth2_token.ok_or_else(|| de::Error::missing_field("oauth2_token"))?,
        })
    }

    pub(crate) fn internal_serialize<S: ::serde::ser::Serializer>(&self, s: &mut S::SerializeStruct) -> Result<(), S::Error> {
        use serde::ser::SerializeStruct;
        s.serialize_field("oauth2_token", &self.oauth2_token)
    }
}

impl<'de> ::serde::de::Deserialize<'de> for TokenFromOAuth1Result {
    fn deserialize<D: ::serde::de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        // struct deserializer
        use serde::de::{MapAccess, Visitor};
        struct StructVisitor;
        impl<'de> Visitor<'de> for StructVisitor {
            type Value = TokenFromOAuth1Result;
            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                f.write_str("a TokenFromOAuth1Result struct")
            }
            fn visit_map<V: MapAccess<'de>>(self, map: V) -> Result<Self::Value, V::Error> {
                TokenFromOAuth1Result::internal_deserialize(map)
            }
        }
        deserializer.deserialize_struct("TokenFromOAuth1Result", TOKEN_FROM_O_AUTH1_RESULT_FIELDS, StructVisitor)
    }
}

impl ::serde::ser::Serialize for TokenFromOAuth1Result {
    fn serialize<S: ::serde::ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // struct serializer
        use serde::ser::SerializeStruct;
        let mut s = serializer.serialize_struct("TokenFromOAuth1Result", 1)?;
        self.internal_serialize::<S>(&mut s)?;
        s.end()
    }
}

