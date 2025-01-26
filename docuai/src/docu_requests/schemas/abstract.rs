use docubase::models::ReportMetadata;
use schema2struct::schema2struct;

use crate::DocuError;

schema2struct! {
    type: object,
    struct: Abstract,
    required: ["r#abstract"],
    properties: {
        "r#abstract": { type: string }
    }

}

impl ReportMetadata for Abstract {
    fn content(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap()
    }

    fn report_type(&self) -> docubase::models::ReportType {
        docubase::models::ReportType::ProjectAbstract
    }
}

impl Abstract {
    #[must_use]
    pub fn to_json() -> serde_json::Value {
        (*ABSTRACT_JSON_VALUE).clone()
    }

    /// # Errors
    /// ```rust
    /// DocuError::JsonConversion(serde_json::Error)
    /// ```
    ///
    /// it returns an error if serde_json failed to convert the
    /// provided instance to json
    pub fn message(content: &Self) -> Result<String, DocuError> {
        let string = serde_json::to_string(content)?;
        Ok(format!("abstract: {string:#?}\n"))
    }
}

//pub(crate) fn create_project_abstract_schema() -> Value {
//    Schema::new(SchemaType::Object)
//        .required(&["abstract"])
//        .property("abstract", string!())
//        .build()
//}
