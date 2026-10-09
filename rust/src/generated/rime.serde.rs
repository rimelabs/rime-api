impl serde::Serialize for AudioParameters {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.audio_format.is_some() {
            len += 1;
        }
        if self.sampling_rate.is_some() {
            len += 1;
        }
        if self.time_scale_factor.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rime.AudioParameters", len)?;
        if let Some(v) = self.audio_format.as_ref() {
            struct_ser.serialize_field("audioFormat", v)?;
        }
        if let Some(v) = self.sampling_rate.as_ref() {
            struct_ser.serialize_field("samplingRate", v)?;
        }
        if let Some(v) = self.time_scale_factor.as_ref() {
            struct_ser.serialize_field("timeScaleFactor", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AudioParameters {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "audio_format",
            "audioFormat",
            "sampling_rate",
            "samplingRate",
            "time_scale_factor",
            "timeScaleFactor",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AudioFormat,
            SamplingRate,
            TimeScaleFactor,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "audioFormat" | "audio_format" => Ok(GeneratedField::AudioFormat),
                            "samplingRate" | "sampling_rate" => Ok(GeneratedField::SamplingRate),
                            "timeScaleFactor" | "time_scale_factor" => Ok(GeneratedField::TimeScaleFactor),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AudioParameters;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rime.AudioParameters")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AudioParameters, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut audio_format__ = None;
                let mut sampling_rate__ = None;
                let mut time_scale_factor__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AudioFormat => {
                            if audio_format__.is_some() {
                                return Err(serde::de::Error::duplicate_field("audioFormat"));
                            }
                            audio_format__ = map_.next_value()?;
                        }
                        GeneratedField::SamplingRate => {
                            if sampling_rate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("samplingRate"));
                            }
                            sampling_rate__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::TimeScaleFactor => {
                            if time_scale_factor__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timeScaleFactor"));
                            }
                            time_scale_factor__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                    }
                }
                Ok(AudioParameters {
                    audio_format: audio_format__,
                    sampling_rate: sampling_rate__,
                    time_scale_factor: time_scale_factor__,
                })
            }
        }
        deserializer.deserialize_struct("rime.AudioParameters", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CodaParameters {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.text_lookahead_tokens.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rime.CodaParameters", len)?;
        if let Some(v) = self.text_lookahead_tokens.as_ref() {
            struct_ser.serialize_field("textLookaheadTokens", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CodaParameters {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "text_lookahead_tokens",
            "textLookaheadTokens",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TextLookaheadTokens,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "textLookaheadTokens" | "text_lookahead_tokens" => Ok(GeneratedField::TextLookaheadTokens),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CodaParameters;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rime.CodaParameters")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CodaParameters, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut text_lookahead_tokens__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TextLookaheadTokens => {
                            if text_lookahead_tokens__.is_some() {
                                return Err(serde::de::Error::duplicate_field("textLookaheadTokens"));
                            }
                            text_lookahead_tokens__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                    }
                }
                Ok(CodaParameters {
                    text_lookahead_tokens: text_lookahead_tokens__,
                })
            }
        }
        deserializer.deserialize_struct("rime.CodaParameters", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetSupportedLanguagesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("rime.GetSupportedLanguagesRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetSupportedLanguagesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetSupportedLanguagesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rime.GetSupportedLanguagesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetSupportedLanguagesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(GetSupportedLanguagesRequest {
                })
            }
        }
        deserializer.deserialize_struct("rime.GetSupportedLanguagesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetSupportedLanguagesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.languages.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rime.GetSupportedLanguagesResponse", len)?;
        if !self.languages.is_empty() {
            struct_ser.serialize_field("languages", &self.languages)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetSupportedLanguagesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "languages",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Languages,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "languages" => Ok(GeneratedField::Languages),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetSupportedLanguagesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rime.GetSupportedLanguagesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetSupportedLanguagesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut languages__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Languages => {
                            if languages__.is_some() {
                                return Err(serde::de::Error::duplicate_field("languages"));
                            }
                            languages__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetSupportedLanguagesResponse {
                    languages: languages__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("rime.GetSupportedLanguagesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetSupportedSpeakersRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.language.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rime.GetSupportedSpeakersRequest", len)?;
        if let Some(v) = self.language.as_ref() {
            struct_ser.serialize_field("language", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetSupportedSpeakersRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "language",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Language,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "language" => Ok(GeneratedField::Language),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetSupportedSpeakersRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rime.GetSupportedSpeakersRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetSupportedSpeakersRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut language__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Language => {
                            if language__.is_some() {
                                return Err(serde::de::Error::duplicate_field("language"));
                            }
                            language__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetSupportedSpeakersRequest {
                    language: language__,
                })
            }
        }
        deserializer.deserialize_struct("rime.GetSupportedSpeakersRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetSupportedSpeakersResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.speakers.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rime.GetSupportedSpeakersResponse", len)?;
        if !self.speakers.is_empty() {
            struct_ser.serialize_field("speakers", &self.speakers)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetSupportedSpeakersResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "speakers",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Speakers,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "speakers" => Ok(GeneratedField::Speakers),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetSupportedSpeakersResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rime.GetSupportedSpeakersResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetSupportedSpeakersResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut speakers__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Speakers => {
                            if speakers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("speakers"));
                            }
                            speakers__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetSupportedSpeakersResponse {
                    speakers: speakers__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("rime.GetSupportedSpeakersResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LanguageSource {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "LANGUAGE_SOURCE_UNSPECIFIED",
            Self::Selected => "LANGUAGE_SOURCE_SELECTED",
            Self::Detected => "LANGUAGE_SOURCE_DETECTED",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for LanguageSource {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "LANGUAGE_SOURCE_UNSPECIFIED",
            "LANGUAGE_SOURCE_SELECTED",
            "LANGUAGE_SOURCE_DETECTED",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LanguageSource;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "LANGUAGE_SOURCE_UNSPECIFIED" => Ok(LanguageSource::Unspecified),
                    "LANGUAGE_SOURCE_SELECTED" => Ok(LanguageSource::Selected),
                    "LANGUAGE_SOURCE_DETECTED" => Ok(LanguageSource::Detected),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for MistParameters {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.pause_between_brackets.is_some() {
            len += 1;
        }
        if self.phonemize_between_brackets.is_some() {
            len += 1;
        }
        if !self.inline_time_scale_factors.is_empty() {
            len += 1;
        }
        if self.save_oovs.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rime.MistParameters", len)?;
        if let Some(v) = self.pause_between_brackets.as_ref() {
            struct_ser.serialize_field("pauseBetweenBrackets", v)?;
        }
        if let Some(v) = self.phonemize_between_brackets.as_ref() {
            struct_ser.serialize_field("phonemizeBetweenBrackets", v)?;
        }
        if !self.inline_time_scale_factors.is_empty() {
            struct_ser.serialize_field("inlineTimeScaleFactors", &self.inline_time_scale_factors)?;
        }
        if let Some(v) = self.save_oovs.as_ref() {
            struct_ser.serialize_field("saveOovs", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MistParameters {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "pause_between_brackets",
            "pauseBetweenBrackets",
            "phonemize_between_brackets",
            "phonemizeBetweenBrackets",
            "inline_time_scale_factors",
            "inlineTimeScaleFactors",
            "save_oovs",
            "saveOovs",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PauseBetweenBrackets,
            PhonemizeBetweenBrackets,
            InlineTimeScaleFactors,
            SaveOovs,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "pauseBetweenBrackets" | "pause_between_brackets" => Ok(GeneratedField::PauseBetweenBrackets),
                            "phonemizeBetweenBrackets" | "phonemize_between_brackets" => Ok(GeneratedField::PhonemizeBetweenBrackets),
                            "inlineTimeScaleFactors" | "inline_time_scale_factors" => Ok(GeneratedField::InlineTimeScaleFactors),
                            "saveOovs" | "save_oovs" => Ok(GeneratedField::SaveOovs),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MistParameters;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rime.MistParameters")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MistParameters, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut pause_between_brackets__ = None;
                let mut phonemize_between_brackets__ = None;
                let mut inline_time_scale_factors__ = None;
                let mut save_oovs__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PauseBetweenBrackets => {
                            if pause_between_brackets__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pauseBetweenBrackets"));
                            }
                            pause_between_brackets__ = map_.next_value()?;
                        }
                        GeneratedField::PhonemizeBetweenBrackets => {
                            if phonemize_between_brackets__.is_some() {
                                return Err(serde::de::Error::duplicate_field("phonemizeBetweenBrackets"));
                            }
                            phonemize_between_brackets__ = map_.next_value()?;
                        }
                        GeneratedField::InlineTimeScaleFactors => {
                            if inline_time_scale_factors__.is_some() {
                                return Err(serde::de::Error::duplicate_field("inlineTimeScaleFactors"));
                            }
                            inline_time_scale_factors__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::SaveOovs => {
                            if save_oovs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("saveOovs"));
                            }
                            save_oovs__ = map_.next_value()?;
                        }
                    }
                }
                Ok(MistParameters {
                    pause_between_brackets: pause_between_brackets__,
                    phonemize_between_brackets: phonemize_between_brackets__,
                    inline_time_scale_factors: inline_time_scale_factors__.unwrap_or_default(),
                    save_oovs: save_oovs__,
                })
            }
        }
        deserializer.deserialize_struct("rime.MistParameters", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for NormalizeTextRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.text.is_empty() {
            len += 1;
        }
        if self.language.is_some() {
            len += 1;
        }
        if !self.custom_lexicon.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rime.NormalizeTextRequest", len)?;
        if !self.text.is_empty() {
            struct_ser.serialize_field("text", &self.text)?;
        }
        if let Some(v) = self.language.as_ref() {
            struct_ser.serialize_field("language", v)?;
        }
        if !self.custom_lexicon.is_empty() {
            struct_ser.serialize_field("customLexicon", &self.custom_lexicon)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for NormalizeTextRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "text",
            "language",
            "custom_lexicon",
            "customLexicon",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Text,
            Language,
            CustomLexicon,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "text" => Ok(GeneratedField::Text),
                            "language" => Ok(GeneratedField::Language),
                            "customLexicon" | "custom_lexicon" => Ok(GeneratedField::CustomLexicon),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = NormalizeTextRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rime.NormalizeTextRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<NormalizeTextRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut text__ = None;
                let mut language__ = None;
                let mut custom_lexicon__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Text => {
                            if text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("text"));
                            }
                            text__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Language => {
                            if language__.is_some() {
                                return Err(serde::de::Error::duplicate_field("language"));
                            }
                            language__ = map_.next_value()?;
                        }
                        GeneratedField::CustomLexicon => {
                            if custom_lexicon__.is_some() {
                                return Err(serde::de::Error::duplicate_field("customLexicon"));
                            }
                            custom_lexicon__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(NormalizeTextRequest {
                    text: text__.unwrap_or_default(),
                    language: language__,
                    custom_lexicon: custom_lexicon__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("rime.NormalizeTextRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for NormalizeTextResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.normalized_text.is_empty() {
            len += 1;
        }
        if !self.normalized_sentences.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rime.NormalizeTextResponse", len)?;
        if !self.normalized_text.is_empty() {
            struct_ser.serialize_field("normalizedText", &self.normalized_text)?;
        }
        if !self.normalized_sentences.is_empty() {
            struct_ser.serialize_field("normalizedSentences", &self.normalized_sentences)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for NormalizeTextResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "normalized_text",
            "normalizedText",
            "normalized_sentences",
            "normalizedSentences",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            NormalizedText,
            NormalizedSentences,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "normalizedText" | "normalized_text" => Ok(GeneratedField::NormalizedText),
                            "normalizedSentences" | "normalized_sentences" => Ok(GeneratedField::NormalizedSentences),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = NormalizeTextResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rime.NormalizeTextResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<NormalizeTextResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut normalized_text__ = None;
                let mut normalized_sentences__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::NormalizedText => {
                            if normalized_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("normalizedText"));
                            }
                            normalized_text__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NormalizedSentences => {
                            if normalized_sentences__.is_some() {
                                return Err(serde::de::Error::duplicate_field("normalizedSentences"));
                            }
                            normalized_sentences__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(NormalizeTextResponse {
                    normalized_text: normalized_text__.unwrap_or_default(),
                    normalized_sentences: normalized_sentences__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("rime.NormalizeTextResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PronunciationEntry {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.spelling.is_empty() {
            len += 1;
        }
        if !self.pronunciation.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rime.PronunciationEntry", len)?;
        if !self.spelling.is_empty() {
            struct_ser.serialize_field("spelling", &self.spelling)?;
        }
        if !self.pronunciation.is_empty() {
            struct_ser.serialize_field("pronunciation", &self.pronunciation)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PronunciationEntry {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "spelling",
            "pronunciation",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Spelling,
            Pronunciation,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "spelling" => Ok(GeneratedField::Spelling),
                            "pronunciation" => Ok(GeneratedField::Pronunciation),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PronunciationEntry;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rime.PronunciationEntry")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PronunciationEntry, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut spelling__ = None;
                let mut pronunciation__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Spelling => {
                            if spelling__.is_some() {
                                return Err(serde::de::Error::duplicate_field("spelling"));
                            }
                            spelling__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Pronunciation => {
                            if pronunciation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pronunciation"));
                            }
                            pronunciation__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(PronunciationEntry {
                    spelling: spelling__.unwrap_or_default(),
                    pronunciation: pronunciation__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("rime.PronunciationEntry", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ResolvedLanguage {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.tag.is_empty() {
            len += 1;
        }
        if self.source != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rime.ResolvedLanguage", len)?;
        if !self.tag.is_empty() {
            struct_ser.serialize_field("tag", &self.tag)?;
        }
        if self.source != 0 {
            let v = LanguageSource::try_from(self.source)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.source)))?;
            struct_ser.serialize_field("source", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ResolvedLanguage {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "tag",
            "source",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Tag,
            Source,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "tag" => Ok(GeneratedField::Tag),
                            "source" => Ok(GeneratedField::Source),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ResolvedLanguage;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rime.ResolvedLanguage")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ResolvedLanguage, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut tag__ = None;
                let mut source__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Tag => {
                            if tag__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tag"));
                            }
                            tag__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Source => {
                            if source__.is_some() {
                                return Err(serde::de::Error::duplicate_field("source"));
                            }
                            source__ = Some(map_.next_value::<LanguageSource>()? as i32);
                        }
                    }
                }
                Ok(ResolvedLanguage {
                    tag: tag__.unwrap_or_default(),
                    source: source__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("rime.ResolvedLanguage", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SpanTimestamp {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.text.is_empty() {
            len += 1;
        }
        if self.start.is_some() {
            len += 1;
        }
        if self.end.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rime.SpanTimestamp", len)?;
        if !self.text.is_empty() {
            struct_ser.serialize_field("text", &self.text)?;
        }
        if let Some(v) = self.start.as_ref() {
            struct_ser.serialize_field("start", v)?;
        }
        if let Some(v) = self.end.as_ref() {
            struct_ser.serialize_field("end", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SpanTimestamp {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "text",
            "start",
            "end",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Text,
            Start,
            End,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "text" => Ok(GeneratedField::Text),
                            "start" => Ok(GeneratedField::Start),
                            "end" => Ok(GeneratedField::End),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SpanTimestamp;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rime.SpanTimestamp")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SpanTimestamp, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut text__ = None;
                let mut start__ = None;
                let mut end__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Text => {
                            if text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("text"));
                            }
                            text__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Start => {
                            if start__.is_some() {
                                return Err(serde::de::Error::duplicate_field("start"));
                            }
                            start__ = map_.next_value()?;
                        }
                        GeneratedField::End => {
                            if end__.is_some() {
                                return Err(serde::de::Error::duplicate_field("end"));
                            }
                            end__ = map_.next_value()?;
                        }
                    }
                }
                Ok(SpanTimestamp {
                    text: text__.unwrap_or_default(),
                    start: start__,
                    end: end__,
                })
            }
        }
        deserializer.deserialize_struct("rime.SpanTimestamp", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SpeechWebSocketCancel {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("rime.SpeechWebSocketCancel", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SpeechWebSocketCancel {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SpeechWebSocketCancel;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rime.SpeechWebSocketCancel")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SpeechWebSocketCancel, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(SpeechWebSocketCancel {
                })
            }
        }
        deserializer.deserialize_struct("rime.SpeechWebSocketCancel", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SpeechWebSocketCancelled {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("rime.SpeechWebSocketCancelled", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SpeechWebSocketCancelled {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SpeechWebSocketCancelled;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rime.SpeechWebSocketCancelled")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SpeechWebSocketCancelled, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(SpeechWebSocketCancelled {
                })
            }
        }
        deserializer.deserialize_struct("rime.SpeechWebSocketCancelled", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SpeechWebSocketEnd {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("rime.SpeechWebSocketEnd", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SpeechWebSocketEnd {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SpeechWebSocketEnd;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rime.SpeechWebSocketEnd")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SpeechWebSocketEnd, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(SpeechWebSocketEnd {
                })
            }
        }
        deserializer.deserialize_struct("rime.SpeechWebSocketEnd", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SpeechWebSocketError {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.kind.is_empty() {
            len += 1;
        }
        if !self.message.is_empty() {
            len += 1;
        }
        if self.request_id.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rime.SpeechWebSocketError", len)?;
        if !self.kind.is_empty() {
            struct_ser.serialize_field("kind", &self.kind)?;
        }
        if !self.message.is_empty() {
            struct_ser.serialize_field("message", &self.message)?;
        }
        if let Some(v) = self.request_id.as_ref() {
            struct_ser.serialize_field("requestId", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SpeechWebSocketError {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "kind",
            "message",
            "request_id",
            "requestId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Kind,
            Message,
            RequestId,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "kind" => Ok(GeneratedField::Kind),
                            "message" => Ok(GeneratedField::Message),
                            "requestId" | "request_id" => Ok(GeneratedField::RequestId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SpeechWebSocketError;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rime.SpeechWebSocketError")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SpeechWebSocketError, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut kind__ = None;
                let mut message__ = None;
                let mut request_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Kind => {
                            if kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("kind"));
                            }
                            kind__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Message => {
                            if message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("message"));
                            }
                            message__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RequestId => {
                            if request_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestId"));
                            }
                            request_id__ = map_.next_value()?;
                        }
                    }
                }
                Ok(SpeechWebSocketError {
                    kind: kind__.unwrap_or_default(),
                    message: message__.unwrap_or_default(),
                    request_id: request_id__,
                })
            }
        }
        deserializer.deserialize_struct("rime.SpeechWebSocketError", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SpeechWebSocketRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.payload.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rime.SpeechWebSocketRequest", len)?;
        if let Some(v) = self.payload.as_ref() {
            match v {
                speech_web_socket_request::Payload::Start(v) => {
                    struct_ser.serialize_field("start", v)?;
                }
                speech_web_socket_request::Payload::Audio(v) => {
                    #[allow(clippy::needless_borrow)]
                    #[allow(clippy::needless_borrows_for_generic_args)]
                    struct_ser.serialize_field("audio", pbjson::private::base64::encode(&v).as_str())?;
                }
                speech_web_socket_request::Payload::End(v) => {
                    struct_ser.serialize_field("end", v)?;
                }
                speech_web_socket_request::Payload::Cancel(v) => {
                    struct_ser.serialize_field("cancel", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SpeechWebSocketRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "start",
            "audio",
            "end",
            "cancel",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Start,
            Audio,
            End,
            Cancel,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "start" => Ok(GeneratedField::Start),
                            "audio" => Ok(GeneratedField::Audio),
                            "end" => Ok(GeneratedField::End),
                            "cancel" => Ok(GeneratedField::Cancel),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SpeechWebSocketRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rime.SpeechWebSocketRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SpeechWebSocketRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut payload__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Start => {
                            if payload__.is_some() {
                                return Err(serde::de::Error::duplicate_field("start"));
                            }
                            payload__ = map_.next_value::<::std::option::Option<_>>()?.map(speech_web_socket_request::Payload::Start)
;
                        }
                        GeneratedField::Audio => {
                            if payload__.is_some() {
                                return Err(serde::de::Error::duplicate_field("audio"));
                            }
                            payload__ = map_.next_value::<::std::option::Option<::pbjson::private::BytesDeserialize<_>>>()?.map(|x| speech_web_socket_request::Payload::Audio(x.0));
                        }
                        GeneratedField::End => {
                            if payload__.is_some() {
                                return Err(serde::de::Error::duplicate_field("end"));
                            }
                            payload__ = map_.next_value::<::std::option::Option<_>>()?.map(speech_web_socket_request::Payload::End)
;
                        }
                        GeneratedField::Cancel => {
                            if payload__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cancel"));
                            }
                            payload__ = map_.next_value::<::std::option::Option<_>>()?.map(speech_web_socket_request::Payload::Cancel)
;
                        }
                    }
                }
                Ok(SpeechWebSocketRequest {
                    payload: payload__,
                })
            }
        }
        deserializer.deserialize_struct("rime.SpeechWebSocketRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SpeechWebSocketResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.payload.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rime.SpeechWebSocketResponse", len)?;
        if let Some(v) = self.payload.as_ref() {
            match v {
                speech_web_socket_response::Payload::Started(v) => {
                    struct_ser.serialize_field("started", v)?;
                }
                speech_web_socket_response::Payload::Delta(v) => {
                    struct_ser.serialize_field("delta", v)?;
                }
                speech_web_socket_response::Payload::Done(v) => {
                    struct_ser.serialize_field("done", v)?;
                }
                speech_web_socket_response::Payload::Error(v) => {
                    struct_ser.serialize_field("error", v)?;
                }
                speech_web_socket_response::Payload::Cancelled(v) => {
                    struct_ser.serialize_field("cancelled", v)?;
                }
                speech_web_socket_response::Payload::Hypothesis(v) => {
                    struct_ser.serialize_field("hypothesis", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SpeechWebSocketResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "started",
            "delta",
            "done",
            "error",
            "cancelled",
            "hypothesis",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Started,
            Delta,
            Done,
            Error,
            Cancelled,
            Hypothesis,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "started" => Ok(GeneratedField::Started),
                            "delta" => Ok(GeneratedField::Delta),
                            "done" => Ok(GeneratedField::Done),
                            "error" => Ok(GeneratedField::Error),
                            "cancelled" => Ok(GeneratedField::Cancelled),
                            "hypothesis" => Ok(GeneratedField::Hypothesis),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SpeechWebSocketResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rime.SpeechWebSocketResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SpeechWebSocketResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut payload__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Started => {
                            if payload__.is_some() {
                                return Err(serde::de::Error::duplicate_field("started"));
                            }
                            payload__ = map_.next_value::<::std::option::Option<_>>()?.map(speech_web_socket_response::Payload::Started)
;
                        }
                        GeneratedField::Delta => {
                            if payload__.is_some() {
                                return Err(serde::de::Error::duplicate_field("delta"));
                            }
                            payload__ = map_.next_value::<::std::option::Option<_>>()?.map(speech_web_socket_response::Payload::Delta)
;
                        }
                        GeneratedField::Done => {
                            if payload__.is_some() {
                                return Err(serde::de::Error::duplicate_field("done"));
                            }
                            payload__ = map_.next_value::<::std::option::Option<_>>()?.map(speech_web_socket_response::Payload::Done)
;
                        }
                        GeneratedField::Error => {
                            if payload__.is_some() {
                                return Err(serde::de::Error::duplicate_field("error"));
                            }
                            payload__ = map_.next_value::<::std::option::Option<_>>()?.map(speech_web_socket_response::Payload::Error)
;
                        }
                        GeneratedField::Cancelled => {
                            if payload__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cancelled"));
                            }
                            payload__ = map_.next_value::<::std::option::Option<_>>()?.map(speech_web_socket_response::Payload::Cancelled)
;
                        }
                        GeneratedField::Hypothesis => {
                            if payload__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hypothesis"));
                            }
                            payload__ = map_.next_value::<::std::option::Option<_>>()?.map(speech_web_socket_response::Payload::Hypothesis)
;
                        }
                    }
                }
                Ok(SpeechWebSocketResponse {
                    payload: payload__,
                })
            }
        }
        deserializer.deserialize_struct("rime.SpeechWebSocketResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SpeechWebSocketStarted {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.request_id.is_empty() {
            len += 1;
        }
        if self.accepted.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rime.SpeechWebSocketStarted", len)?;
        if !self.request_id.is_empty() {
            struct_ser.serialize_field("requestId", &self.request_id)?;
        }
        if let Some(v) = self.accepted.as_ref() {
            struct_ser.serialize_field("accepted", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SpeechWebSocketStarted {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "request_id",
            "requestId",
            "accepted",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RequestId,
            Accepted,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "requestId" | "request_id" => Ok(GeneratedField::RequestId),
                            "accepted" => Ok(GeneratedField::Accepted),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SpeechWebSocketStarted;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rime.SpeechWebSocketStarted")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SpeechWebSocketStarted, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut request_id__ = None;
                let mut accepted__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RequestId => {
                            if request_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestId"));
                            }
                            request_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Accepted => {
                            if accepted__.is_some() {
                                return Err(serde::de::Error::duplicate_field("accepted"));
                            }
                            accepted__ = map_.next_value()?;
                        }
                    }
                }
                Ok(SpeechWebSocketStarted {
                    request_id: request_id__.unwrap_or_default(),
                    accepted: accepted__,
                })
            }
        }
        deserializer.deserialize_struct("rime.SpeechWebSocketStarted", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SplitStrategy {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "SPLIT_STRATEGY_UNSPECIFIED",
            Self::Sentence => "SPLIT_STRATEGY_SENTENCE",
            Self::None => "SPLIT_STRATEGY_NONE",
            Self::AccumulateSentence => "SPLIT_STRATEGY_ACCUMULATE_SENTENCE",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for SplitStrategy {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "SPLIT_STRATEGY_UNSPECIFIED",
            "SPLIT_STRATEGY_SENTENCE",
            "SPLIT_STRATEGY_NONE",
            "SPLIT_STRATEGY_ACCUMULATE_SENTENCE",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SplitStrategy;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "SPLIT_STRATEGY_UNSPECIFIED" => Ok(SplitStrategy::Unspecified),
                    "SPLIT_STRATEGY_SENTENCE" => Ok(SplitStrategy::Sentence),
                    "SPLIT_STRATEGY_NONE" => Ok(SplitStrategy::None),
                    "SPLIT_STRATEGY_ACCUMULATE_SENTENCE" => Ok(SplitStrategy::AccumulateSentence),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for StreamingAccepted {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.output_contract != 0 {
            len += 1;
        }
        if self.language.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rime.StreamingAccepted", len)?;
        if self.output_contract != 0 {
            let v = StreamingOutputContract::try_from(self.output_contract)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.output_contract)))?;
            struct_ser.serialize_field("outputContract", &v)?;
        }
        if let Some(v) = self.language.as_ref() {
            struct_ser.serialize_field("language", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StreamingAccepted {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "output_contract",
            "outputContract",
            "language",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OutputContract,
            Language,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "outputContract" | "output_contract" => Ok(GeneratedField::OutputContract),
                            "language" => Ok(GeneratedField::Language),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StreamingAccepted;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rime.StreamingAccepted")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<StreamingAccepted, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut output_contract__ = None;
                let mut language__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OutputContract => {
                            if output_contract__.is_some() {
                                return Err(serde::de::Error::duplicate_field("outputContract"));
                            }
                            output_contract__ = Some(map_.next_value::<StreamingOutputContract>()? as i32);
                        }
                        GeneratedField::Language => {
                            if language__.is_some() {
                                return Err(serde::de::Error::duplicate_field("language"));
                            }
                            language__ = map_.next_value()?;
                        }
                    }
                }
                Ok(StreamingAccepted {
                    output_contract: output_contract__.unwrap_or_default(),
                    language: language__,
                })
            }
        }
        deserializer.deserialize_struct("rime.StreamingAccepted", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StreamingConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.language.is_some() {
            len += 1;
        }
        if self.mode != 0 {
            len += 1;
        }
        if self.output_contract != 0 {
            len += 1;
        }
        if !self.context_terms.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rime.StreamingConfig", len)?;
        if let Some(v) = self.language.as_ref() {
            struct_ser.serialize_field("language", v)?;
        }
        if self.mode != 0 {
            let v = TranscriptionMode::try_from(self.mode)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.mode)))?;
            struct_ser.serialize_field("mode", &v)?;
        }
        if self.output_contract != 0 {
            let v = StreamingOutputContract::try_from(self.output_contract)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.output_contract)))?;
            struct_ser.serialize_field("outputContract", &v)?;
        }
        if !self.context_terms.is_empty() {
            struct_ser.serialize_field("contextTerms", &self.context_terms)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StreamingConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "language",
            "mode",
            "output_contract",
            "outputContract",
            "context_terms",
            "contextTerms",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Language,
            Mode,
            OutputContract,
            ContextTerms,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "language" => Ok(GeneratedField::Language),
                            "mode" => Ok(GeneratedField::Mode),
                            "outputContract" | "output_contract" => Ok(GeneratedField::OutputContract),
                            "contextTerms" | "context_terms" => Ok(GeneratedField::ContextTerms),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StreamingConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rime.StreamingConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<StreamingConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut language__ = None;
                let mut mode__ = None;
                let mut output_contract__ = None;
                let mut context_terms__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Language => {
                            if language__.is_some() {
                                return Err(serde::de::Error::duplicate_field("language"));
                            }
                            language__ = map_.next_value()?;
                        }
                        GeneratedField::Mode => {
                            if mode__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mode"));
                            }
                            mode__ = Some(map_.next_value::<TranscriptionMode>()? as i32);
                        }
                        GeneratedField::OutputContract => {
                            if output_contract__.is_some() {
                                return Err(serde::de::Error::duplicate_field("outputContract"));
                            }
                            output_contract__ = Some(map_.next_value::<StreamingOutputContract>()? as i32);
                        }
                        GeneratedField::ContextTerms => {
                            if context_terms__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contextTerms"));
                            }
                            context_terms__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(StreamingConfig {
                    language: language__,
                    mode: mode__.unwrap_or_default(),
                    output_contract: output_contract__.unwrap_or_default(),
                    context_terms: context_terms__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("rime.StreamingConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StreamingOutputContract {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "STREAMING_OUTPUT_CONTRACT_UNSPECIFIED",
            Self::AppendFragments => "STREAMING_OUTPUT_CONTRACT_APPEND_FRAGMENTS",
            Self::RevisedHypotheses => "STREAMING_OUTPUT_CONTRACT_REVISED_HYPOTHESES",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for StreamingOutputContract {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "STREAMING_OUTPUT_CONTRACT_UNSPECIFIED",
            "STREAMING_OUTPUT_CONTRACT_APPEND_FRAGMENTS",
            "STREAMING_OUTPUT_CONTRACT_REVISED_HYPOTHESES",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StreamingOutputContract;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "STREAMING_OUTPUT_CONTRACT_UNSPECIFIED" => Ok(StreamingOutputContract::Unspecified),
                    "STREAMING_OUTPUT_CONTRACT_APPEND_FRAGMENTS" => Ok(StreamingOutputContract::AppendFragments),
                    "STREAMING_OUTPUT_CONTRACT_REVISED_HYPOTHESES" => Ok(StreamingOutputContract::RevisedHypotheses),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for StreamingSynthesisRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.payload.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rime.StreamingSynthesisRequest", len)?;
        if let Some(v) = self.payload.as_ref() {
            match v {
                streaming_synthesis_request::Payload::Header(v) => {
                    struct_ser.serialize_field("header", v)?;
                }
                streaming_synthesis_request::Payload::TextChunk(v) => {
                    struct_ser.serialize_field("textChunk", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StreamingSynthesisRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "header",
            "text_chunk",
            "textChunk",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Header,
            TextChunk,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "header" => Ok(GeneratedField::Header),
                            "textChunk" | "text_chunk" => Ok(GeneratedField::TextChunk),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StreamingSynthesisRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rime.StreamingSynthesisRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<StreamingSynthesisRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut payload__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Header => {
                            if payload__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            payload__ = map_.next_value::<::std::option::Option<_>>()?.map(streaming_synthesis_request::Payload::Header)
;
                        }
                        GeneratedField::TextChunk => {
                            if payload__.is_some() {
                                return Err(serde::de::Error::duplicate_field("textChunk"));
                            }
                            payload__ = map_.next_value::<::std::option::Option<_>>()?.map(streaming_synthesis_request::Payload::TextChunk);
                        }
                    }
                }
                Ok(StreamingSynthesisRequest {
                    payload: payload__,
                })
            }
        }
        deserializer.deserialize_struct("rime.StreamingSynthesisRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StreamingTranscriptionRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.payload.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rime.StreamingTranscriptionRequest", len)?;
        if let Some(v) = self.payload.as_ref() {
            match v {
                streaming_transcription_request::Payload::Config(v) => {
                    struct_ser.serialize_field("config", v)?;
                }
                streaming_transcription_request::Payload::Audio(v) => {
                    #[allow(clippy::needless_borrow)]
                    #[allow(clippy::needless_borrows_for_generic_args)]
                    struct_ser.serialize_field("audio", pbjson::private::base64::encode(&v).as_str())?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StreamingTranscriptionRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "config",
            "audio",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Config,
            Audio,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "config" => Ok(GeneratedField::Config),
                            "audio" => Ok(GeneratedField::Audio),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StreamingTranscriptionRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rime.StreamingTranscriptionRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<StreamingTranscriptionRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut payload__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Config => {
                            if payload__.is_some() {
                                return Err(serde::de::Error::duplicate_field("config"));
                            }
                            payload__ = map_.next_value::<::std::option::Option<_>>()?.map(streaming_transcription_request::Payload::Config)
;
                        }
                        GeneratedField::Audio => {
                            if payload__.is_some() {
                                return Err(serde::de::Error::duplicate_field("audio"));
                            }
                            payload__ = map_.next_value::<::std::option::Option<::pbjson::private::BytesDeserialize<_>>>()?.map(|x| streaming_transcription_request::Payload::Audio(x.0));
                        }
                    }
                }
                Ok(StreamingTranscriptionRequest {
                    payload: payload__,
                })
            }
        }
        deserializer.deserialize_struct("rime.StreamingTranscriptionRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StreamingTranscriptionResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.payload.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rime.StreamingTranscriptionResponse", len)?;
        if let Some(v) = self.payload.as_ref() {
            match v {
                streaming_transcription_response::Payload::Delta(v) => {
                    struct_ser.serialize_field("delta", v)?;
                }
                streaming_transcription_response::Payload::Done(v) => {
                    struct_ser.serialize_field("done", v)?;
                }
                streaming_transcription_response::Payload::Accepted(v) => {
                    struct_ser.serialize_field("accepted", v)?;
                }
                streaming_transcription_response::Payload::Hypothesis(v) => {
                    struct_ser.serialize_field("hypothesis", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StreamingTranscriptionResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "delta",
            "done",
            "accepted",
            "hypothesis",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Delta,
            Done,
            Accepted,
            Hypothesis,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "delta" => Ok(GeneratedField::Delta),
                            "done" => Ok(GeneratedField::Done),
                            "accepted" => Ok(GeneratedField::Accepted),
                            "hypothesis" => Ok(GeneratedField::Hypothesis),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StreamingTranscriptionResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rime.StreamingTranscriptionResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<StreamingTranscriptionResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut payload__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Delta => {
                            if payload__.is_some() {
                                return Err(serde::de::Error::duplicate_field("delta"));
                            }
                            payload__ = map_.next_value::<::std::option::Option<_>>()?.map(streaming_transcription_response::Payload::Delta)
;
                        }
                        GeneratedField::Done => {
                            if payload__.is_some() {
                                return Err(serde::de::Error::duplicate_field("done"));
                            }
                            payload__ = map_.next_value::<::std::option::Option<_>>()?.map(streaming_transcription_response::Payload::Done)
;
                        }
                        GeneratedField::Accepted => {
                            if payload__.is_some() {
                                return Err(serde::de::Error::duplicate_field("accepted"));
                            }
                            payload__ = map_.next_value::<::std::option::Option<_>>()?.map(streaming_transcription_response::Payload::Accepted)
;
                        }
                        GeneratedField::Hypothesis => {
                            if payload__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hypothesis"));
                            }
                            payload__ = map_.next_value::<::std::option::Option<_>>()?.map(streaming_transcription_response::Payload::Hypothesis)
;
                        }
                    }
                }
                Ok(StreamingTranscriptionResponse {
                    payload: payload__,
                })
            }
        }
        deserializer.deserialize_struct("rime.StreamingTranscriptionResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SynthesisRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.language.is_some() {
            len += 1;
        }
        if self.speaker.is_some() {
            len += 1;
        }
        if !self.text.is_empty() {
            len += 1;
        }
        if self.audio_parameters.is_some() {
            len += 1;
        }
        if self.split_strategy.is_some() {
            len += 1;
        }
        if self.coda_parameters.is_some() {
            len += 1;
        }
        if self.mist_parameters.is_some() {
            len += 1;
        }
        if self.timestamps.is_some() {
            len += 1;
        }
        if !self.custom_lexicon.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rime.SynthesisRequest", len)?;
        if let Some(v) = self.language.as_ref() {
            struct_ser.serialize_field("language", v)?;
        }
        if let Some(v) = self.speaker.as_ref() {
            struct_ser.serialize_field("speaker", v)?;
        }
        if !self.text.is_empty() {
            struct_ser.serialize_field("text", &self.text)?;
        }
        if let Some(v) = self.audio_parameters.as_ref() {
            struct_ser.serialize_field("audioParameters", v)?;
        }
        if let Some(v) = self.split_strategy.as_ref() {
            let v = SplitStrategy::try_from(*v)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
            struct_ser.serialize_field("splitStrategy", &v)?;
        }
        if let Some(v) = self.coda_parameters.as_ref() {
            struct_ser.serialize_field("codaParameters", v)?;
        }
        if let Some(v) = self.mist_parameters.as_ref() {
            struct_ser.serialize_field("mistParameters", v)?;
        }
        if let Some(v) = self.timestamps.as_ref() {
            struct_ser.serialize_field("timestamps", v)?;
        }
        if !self.custom_lexicon.is_empty() {
            struct_ser.serialize_field("customLexicon", &self.custom_lexicon)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SynthesisRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "language",
            "speaker",
            "text",
            "audio_parameters",
            "audioParameters",
            "split_strategy",
            "splitStrategy",
            "coda_parameters",
            "codaParameters",
            "mist_parameters",
            "mistParameters",
            "timestamps",
            "custom_lexicon",
            "customLexicon",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Language,
            Speaker,
            Text,
            AudioParameters,
            SplitStrategy,
            CodaParameters,
            MistParameters,
            Timestamps,
            CustomLexicon,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "language" => Ok(GeneratedField::Language),
                            "speaker" => Ok(GeneratedField::Speaker),
                            "text" => Ok(GeneratedField::Text),
                            "audioParameters" | "audio_parameters" => Ok(GeneratedField::AudioParameters),
                            "splitStrategy" | "split_strategy" => Ok(GeneratedField::SplitStrategy),
                            "codaParameters" | "coda_parameters" => Ok(GeneratedField::CodaParameters),
                            "mistParameters" | "mist_parameters" => Ok(GeneratedField::MistParameters),
                            "timestamps" => Ok(GeneratedField::Timestamps),
                            "customLexicon" | "custom_lexicon" => Ok(GeneratedField::CustomLexicon),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SynthesisRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rime.SynthesisRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SynthesisRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut language__ = None;
                let mut speaker__ = None;
                let mut text__ = None;
                let mut audio_parameters__ = None;
                let mut split_strategy__ = None;
                let mut coda_parameters__ = None;
                let mut mist_parameters__ = None;
                let mut timestamps__ = None;
                let mut custom_lexicon__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Language => {
                            if language__.is_some() {
                                return Err(serde::de::Error::duplicate_field("language"));
                            }
                            language__ = map_.next_value()?;
                        }
                        GeneratedField::Speaker => {
                            if speaker__.is_some() {
                                return Err(serde::de::Error::duplicate_field("speaker"));
                            }
                            speaker__ = map_.next_value()?;
                        }
                        GeneratedField::Text => {
                            if text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("text"));
                            }
                            text__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AudioParameters => {
                            if audio_parameters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("audioParameters"));
                            }
                            audio_parameters__ = map_.next_value()?;
                        }
                        GeneratedField::SplitStrategy => {
                            if split_strategy__.is_some() {
                                return Err(serde::de::Error::duplicate_field("splitStrategy"));
                            }
                            split_strategy__ = map_.next_value::<::std::option::Option<SplitStrategy>>()?.map(|x| x as i32);
                        }
                        GeneratedField::CodaParameters => {
                            if coda_parameters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("codaParameters"));
                            }
                            coda_parameters__ = map_.next_value()?;
                        }
                        GeneratedField::MistParameters => {
                            if mist_parameters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mistParameters"));
                            }
                            mist_parameters__ = map_.next_value()?;
                        }
                        GeneratedField::Timestamps => {
                            if timestamps__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timestamps"));
                            }
                            timestamps__ = map_.next_value()?;
                        }
                        GeneratedField::CustomLexicon => {
                            if custom_lexicon__.is_some() {
                                return Err(serde::de::Error::duplicate_field("customLexicon"));
                            }
                            custom_lexicon__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(SynthesisRequest {
                    language: language__,
                    speaker: speaker__,
                    text: text__.unwrap_or_default(),
                    audio_parameters: audio_parameters__,
                    split_strategy: split_strategy__,
                    coda_parameters: coda_parameters__,
                    mist_parameters: mist_parameters__,
                    timestamps: timestamps__,
                    custom_lexicon: custom_lexicon__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("rime.SynthesisRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SynthesisResponseStream {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.payload.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rime.SynthesisResponseStream", len)?;
        if let Some(v) = self.payload.as_ref() {
            match v {
                synthesis_response_stream::Payload::Audio(v) => {
                    #[allow(clippy::needless_borrow)]
                    #[allow(clippy::needless_borrows_for_generic_args)]
                    struct_ser.serialize_field("audio", pbjson::private::base64::encode(&v).as_str())?;
                }
                synthesis_response_stream::Payload::Trailer(v) => {
                    struct_ser.serialize_field("trailer", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SynthesisResponseStream {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "audio",
            "trailer",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Audio,
            Trailer,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "audio" => Ok(GeneratedField::Audio),
                            "trailer" => Ok(GeneratedField::Trailer),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SynthesisResponseStream;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rime.SynthesisResponseStream")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SynthesisResponseStream, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut payload__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Audio => {
                            if payload__.is_some() {
                                return Err(serde::de::Error::duplicate_field("audio"));
                            }
                            payload__ = map_.next_value::<::std::option::Option<::pbjson::private::BytesDeserialize<_>>>()?.map(|x| synthesis_response_stream::Payload::Audio(x.0));
                        }
                        GeneratedField::Trailer => {
                            if payload__.is_some() {
                                return Err(serde::de::Error::duplicate_field("trailer"));
                            }
                            payload__ = map_.next_value::<::std::option::Option<_>>()?.map(synthesis_response_stream::Payload::Trailer)
;
                        }
                    }
                }
                Ok(SynthesisResponseStream {
                    payload: payload__,
                })
            }
        }
        deserializer.deserialize_struct("rime.SynthesisResponseStream", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SynthesisResponseTrailer {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.timestamps.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rime.SynthesisResponseTrailer", len)?;
        if let Some(v) = self.timestamps.as_ref() {
            struct_ser.serialize_field("timestamps", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SynthesisResponseTrailer {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "timestamps",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Timestamps,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "timestamps" => Ok(GeneratedField::Timestamps),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SynthesisResponseTrailer;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rime.SynthesisResponseTrailer")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SynthesisResponseTrailer, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut timestamps__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Timestamps => {
                            if timestamps__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timestamps"));
                            }
                            timestamps__ = map_.next_value()?;
                        }
                    }
                }
                Ok(SynthesisResponseTrailer {
                    timestamps: timestamps__,
                })
            }
        }
        deserializer.deserialize_struct("rime.SynthesisResponseTrailer", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TimestampOptions {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.enable {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rime.TimestampOptions", len)?;
        if self.enable {
            struct_ser.serialize_field("enable", &self.enable)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TimestampOptions {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "enable",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Enable,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "enable" => Ok(GeneratedField::Enable),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TimestampOptions;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rime.TimestampOptions")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TimestampOptions, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut enable__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Enable => {
                            if enable__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enable"));
                            }
                            enable__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(TimestampOptions {
                    enable: enable__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("rime.TimestampOptions", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Timestamps {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.status.is_some() {
            len += 1;
        }
        if !self.spans.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rime.Timestamps", len)?;
        if let Some(v) = self.status.as_ref() {
            struct_ser.serialize_field("status", v)?;
        }
        if !self.spans.is_empty() {
            struct_ser.serialize_field("spans", &self.spans)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Timestamps {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "status",
            "spans",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Status,
            Spans,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "status" => Ok(GeneratedField::Status),
                            "spans" => Ok(GeneratedField::Spans),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Timestamps;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rime.Timestamps")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Timestamps, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut status__ = None;
                let mut spans__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = map_.next_value()?;
                        }
                        GeneratedField::Spans => {
                            if spans__.is_some() {
                                return Err(serde::de::Error::duplicate_field("spans"));
                            }
                            spans__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Timestamps {
                    status: status__,
                    spans: spans__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("rime.Timestamps", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TranscriptionDelta {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.text.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rime.TranscriptionDelta", len)?;
        if !self.text.is_empty() {
            struct_ser.serialize_field("text", &self.text)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TranscriptionDelta {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "text",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Text,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "text" => Ok(GeneratedField::Text),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TranscriptionDelta;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rime.TranscriptionDelta")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TranscriptionDelta, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut text__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Text => {
                            if text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("text"));
                            }
                            text__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(TranscriptionDelta {
                    text: text__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("rime.TranscriptionDelta", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TranscriptionDone {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.text.is_empty() {
            len += 1;
        }
        if self.language.is_some() {
            len += 1;
        }
        if self.revision != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rime.TranscriptionDone", len)?;
        if !self.text.is_empty() {
            struct_ser.serialize_field("text", &self.text)?;
        }
        if let Some(v) = self.language.as_ref() {
            struct_ser.serialize_field("language", v)?;
        }
        if self.revision != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("revision", ToString::to_string(&self.revision).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TranscriptionDone {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "text",
            "language",
            "revision",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Text,
            Language,
            Revision,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "text" => Ok(GeneratedField::Text),
                            "language" => Ok(GeneratedField::Language),
                            "revision" => Ok(GeneratedField::Revision),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TranscriptionDone;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rime.TranscriptionDone")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TranscriptionDone, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut text__ = None;
                let mut language__ = None;
                let mut revision__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Text => {
                            if text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("text"));
                            }
                            text__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Language => {
                            if language__.is_some() {
                                return Err(serde::de::Error::duplicate_field("language"));
                            }
                            language__ = map_.next_value()?;
                        }
                        GeneratedField::Revision => {
                            if revision__.is_some() {
                                return Err(serde::de::Error::duplicate_field("revision"));
                            }
                            revision__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(TranscriptionDone {
                    text: text__.unwrap_or_default(),
                    language: language__,
                    revision: revision__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("rime.TranscriptionDone", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TranscriptionHypothesis {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.text.is_empty() {
            len += 1;
        }
        if self.revision != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rime.TranscriptionHypothesis", len)?;
        if !self.text.is_empty() {
            struct_ser.serialize_field("text", &self.text)?;
        }
        if self.revision != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("revision", ToString::to_string(&self.revision).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TranscriptionHypothesis {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "text",
            "revision",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Text,
            Revision,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "text" => Ok(GeneratedField::Text),
                            "revision" => Ok(GeneratedField::Revision),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TranscriptionHypothesis;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rime.TranscriptionHypothesis")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TranscriptionHypothesis, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut text__ = None;
                let mut revision__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Text => {
                            if text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("text"));
                            }
                            text__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Revision => {
                            if revision__.is_some() {
                                return Err(serde::de::Error::duplicate_field("revision"));
                            }
                            revision__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(TranscriptionHypothesis {
                    text: text__.unwrap_or_default(),
                    revision: revision__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("rime.TranscriptionHypothesis", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TranscriptionMode {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "TRANSCRIPTION_MODE_UNSPECIFIED",
            Self::Written => "TRANSCRIPTION_MODE_WRITTEN",
            Self::Verbatim => "TRANSCRIPTION_MODE_VERBATIM",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for TranscriptionMode {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "TRANSCRIPTION_MODE_UNSPECIFIED",
            "TRANSCRIPTION_MODE_WRITTEN",
            "TRANSCRIPTION_MODE_VERBATIM",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TranscriptionMode;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "TRANSCRIPTION_MODE_UNSPECIFIED" => Ok(TranscriptionMode::Unspecified),
                    "TRANSCRIPTION_MODE_WRITTEN" => Ok(TranscriptionMode::Written),
                    "TRANSCRIPTION_MODE_VERBATIM" => Ok(TranscriptionMode::Verbatim),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for TranscriptionOptions {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.language.is_some() {
            len += 1;
        }
        if self.mode != 0 {
            len += 1;
        }
        if !self.context_terms.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rime.TranscriptionOptions", len)?;
        if let Some(v) = self.language.as_ref() {
            struct_ser.serialize_field("language", v)?;
        }
        if self.mode != 0 {
            let v = TranscriptionMode::try_from(self.mode)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.mode)))?;
            struct_ser.serialize_field("mode", &v)?;
        }
        if !self.context_terms.is_empty() {
            struct_ser.serialize_field("contextTerms", &self.context_terms)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TranscriptionOptions {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "language",
            "mode",
            "context_terms",
            "contextTerms",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Language,
            Mode,
            ContextTerms,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "language" => Ok(GeneratedField::Language),
                            "mode" => Ok(GeneratedField::Mode),
                            "contextTerms" | "context_terms" => Ok(GeneratedField::ContextTerms),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TranscriptionOptions;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rime.TranscriptionOptions")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TranscriptionOptions, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut language__ = None;
                let mut mode__ = None;
                let mut context_terms__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Language => {
                            if language__.is_some() {
                                return Err(serde::de::Error::duplicate_field("language"));
                            }
                            language__ = map_.next_value()?;
                        }
                        GeneratedField::Mode => {
                            if mode__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mode"));
                            }
                            mode__ = Some(map_.next_value::<TranscriptionMode>()? as i32);
                        }
                        GeneratedField::ContextTerms => {
                            if context_terms__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contextTerms"));
                            }
                            context_terms__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(TranscriptionOptions {
                    language: language__,
                    mode: mode__.unwrap_or_default(),
                    context_terms: context_terms__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("rime.TranscriptionOptions", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TranscriptionRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.audio.is_empty() {
            len += 1;
        }
        if self.options.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rime.TranscriptionRequest", len)?;
        if !self.audio.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("audio", pbjson::private::base64::encode(&self.audio).as_str())?;
        }
        if let Some(v) = self.options.as_ref() {
            struct_ser.serialize_field("options", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TranscriptionRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "audio",
            "options",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Audio,
            Options,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "audio" => Ok(GeneratedField::Audio),
                            "options" => Ok(GeneratedField::Options),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TranscriptionRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rime.TranscriptionRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TranscriptionRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut audio__ = None;
                let mut options__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Audio => {
                            if audio__.is_some() {
                                return Err(serde::de::Error::duplicate_field("audio"));
                            }
                            audio__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Options => {
                            if options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("options"));
                            }
                            options__ = map_.next_value()?;
                        }
                    }
                }
                Ok(TranscriptionRequest {
                    audio: audio__.unwrap_or_default(),
                    options: options__,
                })
            }
        }
        deserializer.deserialize_struct("rime.TranscriptionRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TranscriptionResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.text.is_empty() {
            len += 1;
        }
        if self.language.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rime.TranscriptionResponse", len)?;
        if !self.text.is_empty() {
            struct_ser.serialize_field("text", &self.text)?;
        }
        if let Some(v) = self.language.as_ref() {
            struct_ser.serialize_field("language", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TranscriptionResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "text",
            "language",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Text,
            Language,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "text" => Ok(GeneratedField::Text),
                            "language" => Ok(GeneratedField::Language),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TranscriptionResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rime.TranscriptionResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TranscriptionResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut text__ = None;
                let mut language__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Text => {
                            if text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("text"));
                            }
                            text__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Language => {
                            if language__.is_some() {
                                return Err(serde::de::Error::duplicate_field("language"));
                            }
                            language__ = map_.next_value()?;
                        }
                    }
                }
                Ok(TranscriptionResponse {
                    text: text__.unwrap_or_default(),
                    language: language__,
                })
            }
        }
        deserializer.deserialize_struct("rime.TranscriptionResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for WebSocketCancel {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("rime.WebSocketCancel", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for WebSocketCancel {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = WebSocketCancel;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rime.WebSocketCancel")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<WebSocketCancel, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(WebSocketCancel {
                })
            }
        }
        deserializer.deserialize_struct("rime.WebSocketCancel", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for WebSocketCancelled {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("rime.WebSocketCancelled", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for WebSocketCancelled {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = WebSocketCancelled;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rime.WebSocketCancelled")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<WebSocketCancelled, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(WebSocketCancelled {
                })
            }
        }
        deserializer.deserialize_struct("rime.WebSocketCancelled", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for WebSocketConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.authorization.is_some() {
            len += 1;
        }
        if self.license.is_some() {
            len += 1;
        }
        if self.defaults.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rime.WebSocketConfig", len)?;
        if let Some(v) = self.authorization.as_ref() {
            struct_ser.serialize_field("authorization", v)?;
        }
        if let Some(v) = self.license.as_ref() {
            struct_ser.serialize_field("license", v)?;
        }
        if let Some(v) = self.defaults.as_ref() {
            struct_ser.serialize_field("defaults", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for WebSocketConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "authorization",
            "license",
            "defaults",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Authorization,
            License,
            Defaults,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "authorization" => Ok(GeneratedField::Authorization),
                            "license" => Ok(GeneratedField::License),
                            "defaults" => Ok(GeneratedField::Defaults),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = WebSocketConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rime.WebSocketConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<WebSocketConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut authorization__ = None;
                let mut license__ = None;
                let mut defaults__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Authorization => {
                            if authorization__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authorization"));
                            }
                            authorization__ = map_.next_value()?;
                        }
                        GeneratedField::License => {
                            if license__.is_some() {
                                return Err(serde::de::Error::duplicate_field("license"));
                            }
                            license__ = map_.next_value()?;
                        }
                        GeneratedField::Defaults => {
                            if defaults__.is_some() {
                                return Err(serde::de::Error::duplicate_field("defaults"));
                            }
                            defaults__ = map_.next_value()?;
                        }
                    }
                }
                Ok(WebSocketConfig {
                    authorization: authorization__,
                    license: license__,
                    defaults: defaults__,
                })
            }
        }
        deserializer.deserialize_struct("rime.WebSocketConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for WebSocketDone {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.timestamps.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rime.WebSocketDone", len)?;
        if let Some(v) = self.timestamps.as_ref() {
            struct_ser.serialize_field("timestamps", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for WebSocketDone {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "timestamps",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Timestamps,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "timestamps" => Ok(GeneratedField::Timestamps),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = WebSocketDone;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rime.WebSocketDone")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<WebSocketDone, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut timestamps__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Timestamps => {
                            if timestamps__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timestamps"));
                            }
                            timestamps__ = map_.next_value()?;
                        }
                    }
                }
                Ok(WebSocketDone {
                    timestamps: timestamps__,
                })
            }
        }
        deserializer.deserialize_struct("rime.WebSocketDone", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for WebSocketEnd {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("rime.WebSocketEnd", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for WebSocketEnd {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = WebSocketEnd;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rime.WebSocketEnd")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<WebSocketEnd, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(WebSocketEnd {
                })
            }
        }
        deserializer.deserialize_struct("rime.WebSocketEnd", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for WebSocketError {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.kind.is_empty() {
            len += 1;
        }
        if !self.message.is_empty() {
            len += 1;
        }
        if self.request_id.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rime.WebSocketError", len)?;
        if !self.kind.is_empty() {
            struct_ser.serialize_field("kind", &self.kind)?;
        }
        if !self.message.is_empty() {
            struct_ser.serialize_field("message", &self.message)?;
        }
        if let Some(v) = self.request_id.as_ref() {
            struct_ser.serialize_field("requestId", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for WebSocketError {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "kind",
            "message",
            "request_id",
            "requestId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Kind,
            Message,
            RequestId,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "kind" => Ok(GeneratedField::Kind),
                            "message" => Ok(GeneratedField::Message),
                            "requestId" | "request_id" => Ok(GeneratedField::RequestId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = WebSocketError;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rime.WebSocketError")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<WebSocketError, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut kind__ = None;
                let mut message__ = None;
                let mut request_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Kind => {
                            if kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("kind"));
                            }
                            kind__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Message => {
                            if message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("message"));
                            }
                            message__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RequestId => {
                            if request_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestId"));
                            }
                            request_id__ = map_.next_value()?;
                        }
                    }
                }
                Ok(WebSocketError {
                    kind: kind__.unwrap_or_default(),
                    message: message__.unwrap_or_default(),
                    request_id: request_id__,
                })
            }
        }
        deserializer.deserialize_struct("rime.WebSocketError", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for WebSocketReady {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.protocol != 0 {
            len += 1;
        }
        if !self.languages.is_empty() {
            len += 1;
        }
        if self.default_language.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rime.WebSocketReady", len)?;
        if self.protocol != 0 {
            struct_ser.serialize_field("protocol", &self.protocol)?;
        }
        if !self.languages.is_empty() {
            struct_ser.serialize_field("languages", &self.languages)?;
        }
        if let Some(v) = self.default_language.as_ref() {
            struct_ser.serialize_field("defaultLanguage", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for WebSocketReady {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "protocol",
            "languages",
            "default_language",
            "defaultLanguage",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Protocol,
            Languages,
            DefaultLanguage,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "protocol" => Ok(GeneratedField::Protocol),
                            "languages" => Ok(GeneratedField::Languages),
                            "defaultLanguage" | "default_language" => Ok(GeneratedField::DefaultLanguage),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = WebSocketReady;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rime.WebSocketReady")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<WebSocketReady, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut protocol__ = None;
                let mut languages__ = None;
                let mut default_language__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Protocol => {
                            if protocol__.is_some() {
                                return Err(serde::de::Error::duplicate_field("protocol"));
                            }
                            protocol__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Languages => {
                            if languages__.is_some() {
                                return Err(serde::de::Error::duplicate_field("languages"));
                            }
                            languages__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DefaultLanguage => {
                            if default_language__.is_some() {
                                return Err(serde::de::Error::duplicate_field("defaultLanguage"));
                            }
                            default_language__ = map_.next_value()?;
                        }
                    }
                }
                Ok(WebSocketReady {
                    protocol: protocol__.unwrap_or_default(),
                    languages: languages__.unwrap_or_default(),
                    default_language: default_language__,
                })
            }
        }
        deserializer.deserialize_struct("rime.WebSocketReady", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for WebSocketRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.context_id.is_empty() {
            len += 1;
        }
        if self.payload.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rime.WebSocketRequest", len)?;
        if !self.context_id.is_empty() {
            struct_ser.serialize_field("contextId", &self.context_id)?;
        }
        if let Some(v) = self.payload.as_ref() {
            match v {
                web_socket_request::Payload::Config(v) => {
                    struct_ser.serialize_field("config", v)?;
                }
                web_socket_request::Payload::Start(v) => {
                    struct_ser.serialize_field("start", v)?;
                }
                web_socket_request::Payload::Text(v) => {
                    struct_ser.serialize_field("text", v)?;
                }
                web_socket_request::Payload::End(v) => {
                    struct_ser.serialize_field("end", v)?;
                }
                web_socket_request::Payload::Cancel(v) => {
                    struct_ser.serialize_field("cancel", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for WebSocketRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "context_id",
            "contextId",
            "config",
            "start",
            "text",
            "end",
            "cancel",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ContextId,
            Config,
            Start,
            Text,
            End,
            Cancel,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "contextId" | "context_id" => Ok(GeneratedField::ContextId),
                            "config" => Ok(GeneratedField::Config),
                            "start" => Ok(GeneratedField::Start),
                            "text" => Ok(GeneratedField::Text),
                            "end" => Ok(GeneratedField::End),
                            "cancel" => Ok(GeneratedField::Cancel),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = WebSocketRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rime.WebSocketRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<WebSocketRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut context_id__ = None;
                let mut payload__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ContextId => {
                            if context_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contextId"));
                            }
                            context_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Config => {
                            if payload__.is_some() {
                                return Err(serde::de::Error::duplicate_field("config"));
                            }
                            payload__ = map_.next_value::<::std::option::Option<_>>()?.map(web_socket_request::Payload::Config)
;
                        }
                        GeneratedField::Start => {
                            if payload__.is_some() {
                                return Err(serde::de::Error::duplicate_field("start"));
                            }
                            payload__ = map_.next_value::<::std::option::Option<_>>()?.map(web_socket_request::Payload::Start)
;
                        }
                        GeneratedField::Text => {
                            if payload__.is_some() {
                                return Err(serde::de::Error::duplicate_field("text"));
                            }
                            payload__ = map_.next_value::<::std::option::Option<_>>()?.map(web_socket_request::Payload::Text);
                        }
                        GeneratedField::End => {
                            if payload__.is_some() {
                                return Err(serde::de::Error::duplicate_field("end"));
                            }
                            payload__ = map_.next_value::<::std::option::Option<_>>()?.map(web_socket_request::Payload::End)
;
                        }
                        GeneratedField::Cancel => {
                            if payload__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cancel"));
                            }
                            payload__ = map_.next_value::<::std::option::Option<_>>()?.map(web_socket_request::Payload::Cancel)
;
                        }
                    }
                }
                Ok(WebSocketRequest {
                    context_id: context_id__.unwrap_or_default(),
                    payload: payload__,
                })
            }
        }
        deserializer.deserialize_struct("rime.WebSocketRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for WebSocketResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.context_id.is_empty() {
            len += 1;
        }
        if self.payload.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rime.WebSocketResponse", len)?;
        if !self.context_id.is_empty() {
            struct_ser.serialize_field("contextId", &self.context_id)?;
        }
        if let Some(v) = self.payload.as_ref() {
            match v {
                web_socket_response::Payload::Ready(v) => {
                    struct_ser.serialize_field("ready", v)?;
                }
                web_socket_response::Payload::Started(v) => {
                    struct_ser.serialize_field("started", v)?;
                }
                web_socket_response::Payload::Audio(v) => {
                    #[allow(clippy::needless_borrow)]
                    #[allow(clippy::needless_borrows_for_generic_args)]
                    struct_ser.serialize_field("audio", pbjson::private::base64::encode(&v).as_str())?;
                }
                web_socket_response::Payload::Done(v) => {
                    struct_ser.serialize_field("done", v)?;
                }
                web_socket_response::Payload::Cancelled(v) => {
                    struct_ser.serialize_field("cancelled", v)?;
                }
                web_socket_response::Payload::Error(v) => {
                    struct_ser.serialize_field("error", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for WebSocketResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "context_id",
            "contextId",
            "ready",
            "started",
            "audio",
            "done",
            "cancelled",
            "error",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ContextId,
            Ready,
            Started,
            Audio,
            Done,
            Cancelled,
            Error,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "contextId" | "context_id" => Ok(GeneratedField::ContextId),
                            "ready" => Ok(GeneratedField::Ready),
                            "started" => Ok(GeneratedField::Started),
                            "audio" => Ok(GeneratedField::Audio),
                            "done" => Ok(GeneratedField::Done),
                            "cancelled" => Ok(GeneratedField::Cancelled),
                            "error" => Ok(GeneratedField::Error),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = WebSocketResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rime.WebSocketResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<WebSocketResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut context_id__ = None;
                let mut payload__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ContextId => {
                            if context_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contextId"));
                            }
                            context_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Ready => {
                            if payload__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ready"));
                            }
                            payload__ = map_.next_value::<::std::option::Option<_>>()?.map(web_socket_response::Payload::Ready)
;
                        }
                        GeneratedField::Started => {
                            if payload__.is_some() {
                                return Err(serde::de::Error::duplicate_field("started"));
                            }
                            payload__ = map_.next_value::<::std::option::Option<_>>()?.map(web_socket_response::Payload::Started)
;
                        }
                        GeneratedField::Audio => {
                            if payload__.is_some() {
                                return Err(serde::de::Error::duplicate_field("audio"));
                            }
                            payload__ = map_.next_value::<::std::option::Option<::pbjson::private::BytesDeserialize<_>>>()?.map(|x| web_socket_response::Payload::Audio(x.0));
                        }
                        GeneratedField::Done => {
                            if payload__.is_some() {
                                return Err(serde::de::Error::duplicate_field("done"));
                            }
                            payload__ = map_.next_value::<::std::option::Option<_>>()?.map(web_socket_response::Payload::Done)
;
                        }
                        GeneratedField::Cancelled => {
                            if payload__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cancelled"));
                            }
                            payload__ = map_.next_value::<::std::option::Option<_>>()?.map(web_socket_response::Payload::Cancelled)
;
                        }
                        GeneratedField::Error => {
                            if payload__.is_some() {
                                return Err(serde::de::Error::duplicate_field("error"));
                            }
                            payload__ = map_.next_value::<::std::option::Option<_>>()?.map(web_socket_response::Payload::Error)
;
                        }
                    }
                }
                Ok(WebSocketResponse {
                    context_id: context_id__.unwrap_or_default(),
                    payload: payload__,
                })
            }
        }
        deserializer.deserialize_struct("rime.WebSocketResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for WebSocketStarted {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.request_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("rime.WebSocketStarted", len)?;
        if !self.request_id.is_empty() {
            struct_ser.serialize_field("requestId", &self.request_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for WebSocketStarted {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "request_id",
            "requestId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RequestId,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "requestId" | "request_id" => Ok(GeneratedField::RequestId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = WebSocketStarted;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct rime.WebSocketStarted")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<WebSocketStarted, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut request_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RequestId => {
                            if request_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestId"));
                            }
                            request_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(WebSocketStarted {
                    request_id: request_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("rime.WebSocketStarted", FIELDS, GeneratedVisitor)
    }
}
