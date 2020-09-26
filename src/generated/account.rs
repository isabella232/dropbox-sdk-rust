// DO NOT EDIT
// This file was @generated by Stone

#![allow(
    clippy::too_many_arguments,
    clippy::large_enum_variant,
    clippy::doc_markdown,
)]

/// Sets a user's profile photo.
pub fn set_profile_photo(
    client: &dyn crate::client_trait::HttpClient,
    arg: &SetProfilePhotoArg,
) -> crate::Result<Result<SetProfilePhotoResult, SetProfilePhotoError>> {
    crate::client_helpers::request(
        client,
        crate::client_trait::Endpoint::Api,
        crate::client_trait::Style::Rpc,
        crate::client_trait::Auth::Token,
        "account/set_profile_photo",
        arg,
        None)
}

#[derive(Debug)]
pub enum PhotoSourceArg {
    /// Image data in base64-encoded bytes.
    Base64Data(String),
    /// Catch-all used for unrecognized values returned from the server. Encountering this value
    /// typically indicates that this SDK version is out of date.
    Other,
}

impl<'de> ::serde::de::Deserialize<'de> for PhotoSourceArg {
    fn deserialize<D: ::serde::de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        // union deserializer
        use serde::de::{self, MapAccess, Visitor};
        struct EnumVisitor;
        impl<'de> Visitor<'de> for EnumVisitor {
            type Value = PhotoSourceArg;
            fn expecting(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.write_str("a PhotoSourceArg structure")
            }
            fn visit_map<V: MapAccess<'de>>(self, mut map: V) -> Result<Self::Value, V::Error> {
                let tag: &str = match map.next_key()? {
                    Some(".tag") => map.next_value()?,
                    _ => return Err(de::Error::missing_field(".tag"))
                };
                match tag {
                    "base64_data" => {
                        match map.next_key()? {
                            Some("base64_data") => Ok(PhotoSourceArg::Base64Data(map.next_value()?)),
                            None => Err(de::Error::missing_field("base64_data")),
                            _ => Err(de::Error::unknown_field(tag, VARIANTS))
                        }
                    }
                    _ => {
                        crate::eat_json_fields(&mut map)?;
                        Ok(PhotoSourceArg::Other)
                    }
                }
            }
        }
        const VARIANTS: &[&str] = &["base64_data",
                                    "other"];
        deserializer.deserialize_struct("PhotoSourceArg", VARIANTS, EnumVisitor)
    }
}

impl ::serde::ser::Serialize for PhotoSourceArg {
    fn serialize<S: ::serde::ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // union serializer
        use serde::ser::SerializeStruct;
        match *self {
            PhotoSourceArg::Base64Data(ref x) => {
                // primitive
                let mut s = serializer.serialize_struct("PhotoSourceArg", 2)?;
                s.serialize_field(".tag", "base64_data")?;
                s.serialize_field("base64_data", x)?;
                s.end()
            }
            PhotoSourceArg::Other => Err(::serde::ser::Error::custom("cannot serialize 'Other' variant"))
        }
    }
}

#[derive(Debug)]
pub struct SetProfilePhotoArg {
    /// Image to set as the user's new profile photo.
    pub photo: PhotoSourceArg,
}

impl SetProfilePhotoArg {
    pub fn new(photo: PhotoSourceArg) -> Self {
        SetProfilePhotoArg {
            photo,
        }
    }

}

const SET_PROFILE_PHOTO_ARG_FIELDS: &[&str] = &["photo"];
impl SetProfilePhotoArg {
    pub(crate) fn internal_deserialize<'de, V: ::serde::de::MapAccess<'de>>(
        map: V,
    ) -> Result<SetProfilePhotoArg, V::Error> {
        Self::internal_deserialize_opt(map, false).map(Option::unwrap)
    }

    pub(crate) fn internal_deserialize_opt<'de, V: ::serde::de::MapAccess<'de>>(
        mut map: V,
        optional: bool,
    ) -> Result<Option<SetProfilePhotoArg>, V::Error> {
        let mut field_photo = None;
        let mut nothing = true;
        while let Some(key) = map.next_key::<&str>()? {
            nothing = false;
            match key {
                "photo" => {
                    if field_photo.is_some() {
                        return Err(::serde::de::Error::duplicate_field("photo"));
                    }
                    field_photo = Some(map.next_value()?);
                }
                _ => {
                    // unknown field allowed and ignored
                    map.next_value::<::serde_json::Value>()?;
                }
            }
        }
        if optional && nothing {
            return Ok(None);
        }
        let result = SetProfilePhotoArg {
            photo: field_photo.ok_or_else(|| ::serde::de::Error::missing_field("photo"))?,
        };
        Ok(Some(result))
    }

    pub(crate) fn internal_serialize<S: ::serde::ser::Serializer>(
        &self,
        s: &mut S::SerializeStruct,
    ) -> Result<(), S::Error> {
        use serde::ser::SerializeStruct;
        s.serialize_field("photo", &self.photo)
    }
}

impl<'de> ::serde::de::Deserialize<'de> for SetProfilePhotoArg {
    fn deserialize<D: ::serde::de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        // struct deserializer
        use serde::de::{MapAccess, Visitor};
        struct StructVisitor;
        impl<'de> Visitor<'de> for StructVisitor {
            type Value = SetProfilePhotoArg;
            fn expecting(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.write_str("a SetProfilePhotoArg struct")
            }
            fn visit_map<V: MapAccess<'de>>(self, map: V) -> Result<Self::Value, V::Error> {
                SetProfilePhotoArg::internal_deserialize(map)
            }
        }
        deserializer.deserialize_struct("SetProfilePhotoArg", SET_PROFILE_PHOTO_ARG_FIELDS, StructVisitor)
    }
}

impl ::serde::ser::Serialize for SetProfilePhotoArg {
    fn serialize<S: ::serde::ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // struct serializer
        use serde::ser::SerializeStruct;
        let mut s = serializer.serialize_struct("SetProfilePhotoArg", 1)?;
        self.internal_serialize::<S>(&mut s)?;
        s.end()
    }
}

#[derive(Debug)]
pub enum SetProfilePhotoError {
    /// File cannot be set as profile photo.
    FileTypeError,
    /// File cannot exceed 10 MB.
    FileSizeError,
    /// Image must be larger than 128 x 128.
    DimensionError,
    /// Image could not be thumbnailed.
    ThumbnailError,
    /// Temporary infrastructure failure, please retry.
    TransientError,
    /// Catch-all used for unrecognized values returned from the server. Encountering this value
    /// typically indicates that this SDK version is out of date.
    Other,
}

impl<'de> ::serde::de::Deserialize<'de> for SetProfilePhotoError {
    fn deserialize<D: ::serde::de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        // union deserializer
        use serde::de::{self, MapAccess, Visitor};
        struct EnumVisitor;
        impl<'de> Visitor<'de> for EnumVisitor {
            type Value = SetProfilePhotoError;
            fn expecting(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.write_str("a SetProfilePhotoError structure")
            }
            fn visit_map<V: MapAccess<'de>>(self, mut map: V) -> Result<Self::Value, V::Error> {
                let tag: &str = match map.next_key()? {
                    Some(".tag") => map.next_value()?,
                    _ => return Err(de::Error::missing_field(".tag"))
                };
                match tag {
                    "file_type_error" => {
                        crate::eat_json_fields(&mut map)?;
                        Ok(SetProfilePhotoError::FileTypeError)
                    }
                    "file_size_error" => {
                        crate::eat_json_fields(&mut map)?;
                        Ok(SetProfilePhotoError::FileSizeError)
                    }
                    "dimension_error" => {
                        crate::eat_json_fields(&mut map)?;
                        Ok(SetProfilePhotoError::DimensionError)
                    }
                    "thumbnail_error" => {
                        crate::eat_json_fields(&mut map)?;
                        Ok(SetProfilePhotoError::ThumbnailError)
                    }
                    "transient_error" => {
                        crate::eat_json_fields(&mut map)?;
                        Ok(SetProfilePhotoError::TransientError)
                    }
                    _ => {
                        crate::eat_json_fields(&mut map)?;
                        Ok(SetProfilePhotoError::Other)
                    }
                }
            }
        }
        const VARIANTS: &[&str] = &["file_type_error",
                                    "file_size_error",
                                    "dimension_error",
                                    "thumbnail_error",
                                    "transient_error",
                                    "other"];
        deserializer.deserialize_struct("SetProfilePhotoError", VARIANTS, EnumVisitor)
    }
}

impl ::serde::ser::Serialize for SetProfilePhotoError {
    fn serialize<S: ::serde::ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // union serializer
        use serde::ser::SerializeStruct;
        match *self {
            SetProfilePhotoError::FileTypeError => {
                // unit
                let mut s = serializer.serialize_struct("SetProfilePhotoError", 1)?;
                s.serialize_field(".tag", "file_type_error")?;
                s.end()
            }
            SetProfilePhotoError::FileSizeError => {
                // unit
                let mut s = serializer.serialize_struct("SetProfilePhotoError", 1)?;
                s.serialize_field(".tag", "file_size_error")?;
                s.end()
            }
            SetProfilePhotoError::DimensionError => {
                // unit
                let mut s = serializer.serialize_struct("SetProfilePhotoError", 1)?;
                s.serialize_field(".tag", "dimension_error")?;
                s.end()
            }
            SetProfilePhotoError::ThumbnailError => {
                // unit
                let mut s = serializer.serialize_struct("SetProfilePhotoError", 1)?;
                s.serialize_field(".tag", "thumbnail_error")?;
                s.end()
            }
            SetProfilePhotoError::TransientError => {
                // unit
                let mut s = serializer.serialize_struct("SetProfilePhotoError", 1)?;
                s.serialize_field(".tag", "transient_error")?;
                s.end()
            }
            SetProfilePhotoError::Other => Err(::serde::ser::Error::custom("cannot serialize 'Other' variant"))
        }
    }
}

impl ::std::error::Error for SetProfilePhotoError {
    fn description(&self) -> &str {
        "SetProfilePhotoError"
    }
}

impl ::std::fmt::Display for SetProfilePhotoError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", *self)
    }
}

#[derive(Debug)]
pub struct SetProfilePhotoResult {
    /// URL for the photo representing the user, if one is set.
    pub profile_photo_url: String,
}

impl SetProfilePhotoResult {
    pub fn new(profile_photo_url: String) -> Self {
        SetProfilePhotoResult {
            profile_photo_url,
        }
    }

}

const SET_PROFILE_PHOTO_RESULT_FIELDS: &[&str] = &["profile_photo_url"];
impl SetProfilePhotoResult {
    pub(crate) fn internal_deserialize<'de, V: ::serde::de::MapAccess<'de>>(
        map: V,
    ) -> Result<SetProfilePhotoResult, V::Error> {
        Self::internal_deserialize_opt(map, false).map(Option::unwrap)
    }

    pub(crate) fn internal_deserialize_opt<'de, V: ::serde::de::MapAccess<'de>>(
        mut map: V,
        optional: bool,
    ) -> Result<Option<SetProfilePhotoResult>, V::Error> {
        let mut field_profile_photo_url = None;
        let mut nothing = true;
        while let Some(key) = map.next_key::<&str>()? {
            nothing = false;
            match key {
                "profile_photo_url" => {
                    if field_profile_photo_url.is_some() {
                        return Err(::serde::de::Error::duplicate_field("profile_photo_url"));
                    }
                    field_profile_photo_url = Some(map.next_value()?);
                }
                _ => {
                    // unknown field allowed and ignored
                    map.next_value::<::serde_json::Value>()?;
                }
            }
        }
        if optional && nothing {
            return Ok(None);
        }
        let result = SetProfilePhotoResult {
            profile_photo_url: field_profile_photo_url.ok_or_else(|| ::serde::de::Error::missing_field("profile_photo_url"))?,
        };
        Ok(Some(result))
    }

    pub(crate) fn internal_serialize<S: ::serde::ser::Serializer>(
        &self,
        s: &mut S::SerializeStruct,
    ) -> Result<(), S::Error> {
        use serde::ser::SerializeStruct;
        s.serialize_field("profile_photo_url", &self.profile_photo_url)
    }
}

impl<'de> ::serde::de::Deserialize<'de> for SetProfilePhotoResult {
    fn deserialize<D: ::serde::de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        // struct deserializer
        use serde::de::{MapAccess, Visitor};
        struct StructVisitor;
        impl<'de> Visitor<'de> for StructVisitor {
            type Value = SetProfilePhotoResult;
            fn expecting(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.write_str("a SetProfilePhotoResult struct")
            }
            fn visit_map<V: MapAccess<'de>>(self, map: V) -> Result<Self::Value, V::Error> {
                SetProfilePhotoResult::internal_deserialize(map)
            }
        }
        deserializer.deserialize_struct("SetProfilePhotoResult", SET_PROFILE_PHOTO_RESULT_FIELDS, StructVisitor)
    }
}

impl ::serde::ser::Serialize for SetProfilePhotoResult {
    fn serialize<S: ::serde::ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // struct serializer
        use serde::ser::SerializeStruct;
        let mut s = serializer.serialize_struct("SetProfilePhotoResult", 1)?;
        self.internal_serialize::<S>(&mut s)?;
        s.end()
    }
}

