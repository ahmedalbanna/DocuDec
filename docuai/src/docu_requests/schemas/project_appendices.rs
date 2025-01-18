use jsonschema::jsonschema;

jsonschema! {
    type: object,
    struct: ProjectAppendices,
    required: ["appendices"],
    properties: {
        "appendices": {
            type: array,
            items: {
                type: object,
                struct: AppendicesValue,
                required: ["title", "description", "content"],
                properties: {
                    "title": { type: string },
                    "description": { type: string },
                    "content": { type: string }
                }
            }
        }
    }
}
