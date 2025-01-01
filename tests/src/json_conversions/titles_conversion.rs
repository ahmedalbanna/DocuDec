#[test]
fn test_titles_conversion() {
    let json_data = r#"
    {
        "alternativeTitles": {
            "focusingOnAutomation": [
                "Automated Generation of Graduation Project Reports",
                "Automated Project Report Generation System",
                "Automating Graduation Project Report Creation"
            ],
            "focusingOnInnovationOrProblemSolving": [
                "Innovative System for Automated Project Report Generation",
                "Solving Project Report Generation with Automation",
                "Improving Project Report Efficiency via Automation"
            ],
            "focusingOnReports": [
                "Graduation Project Report Generation System",
                "Comprehensive Report Generation for Graduation Projects",
                "System for Automated Project Report Creation and Management"
            ],
            "moreConciseTitles": [
                "Automated Project Report System",
                "Project Report Automation Tool",
                "PlantUML Report Generator"
            ],
            "moreSpecificAndTechnicalTitles": [
                "PlantUML-Based Automated Project Report Generation",
                "Automated Generation of Graduation Reports Using PlantUML",
                "Automated Project Reporting with PlantUML Diagrams"
            ]
        },
        "originalTitle": "Automated Project Automation",
        "recommendation": "I recommend to chose  'Automated Project Reporting with PlantUML Diagrams'"
    }
    "#;

    // Deserialize JSON into the Titles struct
    let titles: docuai::docu_requests::responses::alter_titles::Titles =
        serde_json::from_str(json_data).expect("Failed to parse the json to Titles");

    assert!(!titles.recommendation.is_empty())
}
