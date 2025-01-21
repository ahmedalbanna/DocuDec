use docubase::models::ReportMetadata;
use schema2struct::schema2struct;

schema2struct! {
    type: object,
    struct: AdditionalInstruction,
    required: ["additionalInstructions"],
    properties: {
        "additionalInstructions": {
            type: object,
            struct: InnerAdditionalInstructions,
            required: ["toneAndStyle", "keywords", "constraints"],
            properties: {
                "toneAndStyle": { type: string },
                "keywords": { type: string },
                "constraints": { type: string }
            }
        }
    }

}

impl ReportMetadata for AdditionalInstruction {
    fn content(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap()
    }
    fn report_type(&self) -> docubase::models::ReportType {
        docubase::models::ReportType::AdditionalInstruction
    }
}

//use serde_json::Value;
//
//pub fn create_additional_instruction_schema() -> Value {
//    let additional_instructions = Schema::new(SchemaType::Object)
//        .required(&["toneAndStyle", "keywords", "constraints"])
//        .property("toneAndStyle", string!())
//        .property("keywords", string!())
//        .property("constraints", string!());
//
//    Schema::new(SchemaType::Object)
//        .required(&["additionalInstructions"])
//        .property("additionalInstructions", additional_instructions)
//        .build()
//}
