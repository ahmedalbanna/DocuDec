use jsonschema::jsonschema;

jsonschema! {
    type: object,
    struct: ProjectReferences,
    required: ["references"],
    properties: {
        "references": {
            type: array,
            items: {
                type: object,
                struct: ReferencesValue,
                required: ["authors", "title", "year", "publisher", "url"],
                properties: {
                    "authors": { type: string },
                    "title": { type: string },
                    "year": { type: string },
                    "publisher": { type: string },
                    "url": { type: string }
                }
            }
        }
    }

}
