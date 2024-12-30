#[cfg(test)]
mod tests {
    use docuai::docu_requests::responses::{alter_titles::Titles, project_info::ProjectInfo};

    #[test]
    fn test_project_info_conversion() {
        let json_data = r#"
    {
        "fieldOfStudy": {
            "academicField": "Computer Science",
            "justification": [
                "This project involves software development, algorithm design, and automation, which are all core components of computer science.",
                "The use of PlantUML for diagram generation and text processing aligns with the curriculum and research areas within computer science, particularly in software engineering and formal methods.",
                "Related fields include software engineering, information technology, and systems engineering. The decision to focus on Computer Science is based on the project's core nature being related to building a software tool to automate report generation."
            ]
        },
        "projectScope": {
            "approvalProcess": "The final report and the system will be evaluated and approved by the university's academic advisor, according to the standard graduation project review guidelines.",
            "boundaries": "This project focuses on the automated generation of the graduation project report using PlantUML diagrams as input. It will not address other document formats or non-technical documentation. It is limited to a predefined report structure and content based on the university's guidelines. The system’s functionality will be focused on the basic conversion and doesn’t cover any advanced report generation functionality.",
            "description": "This project aims to develop an automated system that generates graduation project reports based on diagrams created using PlantUML. This system will streamline the report generation process, improve report consistency and reduce manual efforts involved in documentation.",
            "objectivesAndGoals": [
                "Develop a software tool that converts PlantUML diagrams into structured text for a graduation project report.",
                "Ensure that the generated reports adhere to the university’s specific formatting and content requirements.",
                "Reduce the time and effort required for manual report generation.",
                "Improve report consistency across different graduation projects.",
                "Provide a user-friendly tool for students to create documentation."
            ],
            "resourceAllocation": {
                "teamResponsibilities": [
                    "Project Manager: overseeing overall project progress, coordinating team efforts and maintaining communication.",
                    "Software Developer: implementing the code for PlantUML parsing and report generation.",
                    "Quality Assurance: performing testing and validation of the generated reports."
                ]
            },
            "riskAssessment": "Potential risks include difficulties in parsing complex PlantUML diagrams, inconsistencies in the university’s report guidelines, and time constraints in the development phase. Mitigation strategies include thorough testing, regular communication with advisors, and maintaining a flexible development schedule. Regular progress meetings, testing of the tool against a variety of examples, and communication with advisors to clarify university guidelines will be used to mitigate the risks identified.",
            "stakeholderIdentification": [
                "Students: the primary users of the automated report generation tool.",
                "University Advisors: who will oversee the project and assess the reports.",
                "Academic Department: which defines the report formatting and content guidelines."
            ],
            "timeline": {
                "start": "01/15/2024",
                "end": "12/20/2024",
                "milestones": [
                    "Project Proposal Approval (02/05/2024)",
                    "System Architecture Design (03/05/2024)",
                    "PlantUML Parser Implementation (04/15/2024)",
                    "Report Template Development (05/15/2024)",
                    "System Integration and Testing (07/30/2024)",
                    "User Guide Completion (09/15/2024)",
                    "Final Report Submission (11/15/2024)",
                    "Final Presentation (12/15/2024)"
                ]
            }
        },
        "projectTitle": "PlantUML-Based Automated System for Graduation Project Report Generation",
        "targetAudience": {
            "audienceFeedback": [
                "Gather feedback through online surveys.",
                "Conduct interviews with students.",
                "Request feedback during project presentations."
            ],
            "audiencePersonas": [
                "Undergraduate CS student, struggling with report writing.",
                "Master student, needs automation for reports",
                "Professor, assessing the graduation projects."
            ],
            "customerDataAnalysis": [
                "Analyze student feedback on existing documentation.",
                "Gather usage data after deployment of the tool."
            ],
            "demographics": {
                "age": "20-30",
                "gender": "Male and Female",
                "location": "University Campus"
            },
            "marketResearch": "The market includes students needing to document their projects. There is high demand for automation tools in documentation within academia. Competitor analysis shows that other tools may exist but are not specifically tailored to PlantUML integration. Surveys will help confirm the target needs and the demand for this kind of tools.",
            "psychographics": [
                "Interested in technology and automation.",
                "Value efficiency and time-saving tools.",
                "Appreciate clear and concise documentation."
            ]
        }
    }
    "#;
        // Deserialize JSON into the ProjectInfo struct
        let titles: ProjectInfo = serde_json::from_str(json_data).expect("Failed to parse JSON");

        println!("{:#?}", titles);
    }
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
        let titles: Titles = serde_json::from_str(json_data).expect("Failed to parse JSON");

        println!("{:#?}", titles);
    }
}
