use schema2struct::schema2struct;

use crate::DocuError;

schema2struct! {
    type: object,
    struct: ProjectDetails,
    required: ["projectDetails"],
    properties: {
        "projectDetails": {
            type: object,
            struct: InnerProjectDetails,
            required: [
                "overview",
                "projectPlanning",
                "requirementsGathering",
                "hardwareRequirements",
                "softwareRequirements",
                "designPhase",
                "implementationPhase",
                "testingPhase",
                "deploymentPhase",
                "maintenanceAndSupport",
                "documentation",
            ],
            properties: {
                "overview": { type: string },
                "projectPlanning": {
                    type: object,
                    struct: key,
                    required: [
                        "defineScope",
                        "establishTimeline",
                        "identifyStakeholders",
                        "allocateResources",
                    ],
                    properties: {
                        "defineScope": { type: string },
                        "establishTimeline": { type: string },
                        "identifyStakeholders": { type: string },
                        "allocateResources": { type: string }
                    }
                },
                "requirementsGathering": {
                    type: object,
                    struct: key,
                    required: [
                        "conductInterviews",
                        "reviewDocumentation",
                        "compileUserRequirements",
                    ],
                    properties: {
                        "conductInterviews": { type: string },
                        "reviewDocumentation": { type: string },
                        "compileUserRequirements": { type: string }
                    }
                },
                "hardwareRequirements": {
                    type: object,
                    struct: key,
                    required: [
                         "computingPower",
                         "peripheralDevices",
                         "networkConnectivity",
                         "scalability",
                    ],
                    properties: {
                        "networkConnectivity": { type: string },
                        "scalability": { type: string },
                        "computingPower": {
                            type: object,
                            struct: key,
                            required: ["cpu", "ram", "storage"],
                            properties: {
                                "cpu": { type: string },
                                "ram": { type: string },
                                "storage": { type: string }
                            }
                        },
                        "peripheralDevices": {
                            type: object,
                            struct: key,
                            required: ["inputDevices", "outputDevices"],
                            properties: {
                                "inputDevices": { type: string },
                                "outputDevices": { type: string }
                            }
                        }
                    }
                },
                "softwareRequirements": {
                    type: object,
                    struct: key,
                    required: [
                          "operatingSystem",
                          "softwareApplications",
                          "databaseRequirements",
                          "securitySoftwareNeeds",
                    ],
                    properties: {
                        "operatingSystem": { type: string },
                        "databaseRequirements": { type: string },
                        "securitySoftwareNeeds": { type: string },
                        "softwareApplications": {
                            type: object,
                            struct: key,
                            required: [
                                 "listApplications",
                                 "versionCompatibility",
                            ],
                            properties: {
                                "listApplications": { type: string },
                                "versionCompatibility": { type: string }
                            }
                        }
                    }
                },
                "designPhase": {
                    type: object,
                    struct: key,
                    required: [
                       "systemArchitectureDiagrams",
                       "userInterfaceMockups",
                       "databaseSchema",
                    ],
                    properties: {
                        "systemArchitectureDiagrams": { type: string },
                        "userInterfaceMockups": { type: string },
                        "databaseSchema": { type: string }
                    }
                },
                "implementationPhase": {
                    type: object,
                    struct: key,
                    required: [
                        "hardwareSetup",
                        "softwareInstallation",
                        "codeDevelopment",
                    ],
                    properties: {
                        "hardwareSetup": { type: string },
                        "codeDevelopment": { type: string },
                        "softwareInstallation": {
                            type: object,
                            struct: key,
                            required: ["installOs", "installApplications"],
                            properties: {
                                "installOs": { type: string },
                                "installApplications": { type: string }
                            }
                        }
                    }
                },
                "testingPhase": {
                    type: object,
                    struct: key,
                    required: [
                        "createTestPlan",
                        "unitTesting",
                        "integrationTesting",
                        "userAcceptanceTesting",
                    ],
                    properties: {
                        "createTestPlan": { type: string },
                        "unitTesting": { type: string },
                        "integrationTesting": { type: string },
                        "userAcceptanceTesting": { type: string }
                    }
                },
                "deploymentPhase": {
                    type: object,
                    struct: key,
                    required: ["deploymentPlan", "userTraining"],
                    properties: {
                        "deploymentPlan": { type: string },
                        "userTraining": { type: string }
                    }
                },
                "maintenanceAndSupport": {
                    type: object,
                    struct: key,
                    required: ["maintenancePlan", "supportServices"],
                    properties: {
                        "maintenancePlan": { type: string },
                        "supportServices": { type: string }
                    }
                },
                "documentation": {
                    type: object,
                    struct: key,
                    required: ["userManual", "technicalDocumentation"],
                    properties: {
                        "userManual": { type: string },
                        "technicalDocumentation": { type: string }
                    }
                }
            }

        }
    }
}

impl ProjectDetails {
    #[must_use]
    pub fn to_json() -> serde_json::Value {
        (*PROJECTDETAILS_JSON_VALUE).clone()
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
        Ok(format!("projectDetails: {string:#?}\n"))
    }
}
