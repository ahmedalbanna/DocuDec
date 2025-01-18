use jsonschema::jsonschema;

jsonschema! {
    type: object,
    struct: ProjectConclusion,
    properties: {
        "conclusion": {
            type: object,
            struct: key,
            required: [
                "summaryOfFindings",
                "achievements",
                "lessonsLearned",
                "futureDirections",
                "finalThoughts",
            ],
            properties: {
                "summaryOfFindings": { type: string },
                "achievements": { type: string },
                "lessonsLearned": { type: string },
                "futureDirections": { type: string },
                "finalThoughts": { type: string },
            }
        }
    }
}
