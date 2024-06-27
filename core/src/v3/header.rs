use super::v2;

impl From<v2::Header> for openapiv3::Header {
    fn from(v2: v2::Header) -> Self {
        openapiv3::Header {
            description: v2.description,
            style: Default::default(),
            required: false,
            deprecated: None,
            format: openapiv3::ParameterSchemaOrContent::Content(Default::default()),
            example: None,
            extensions: Default::default(),
            examples: Default::default(),
        }
    }
}
