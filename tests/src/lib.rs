#[cfg(test)]
mod tests {
    use docuai::docu_requests::responses::{
        alter_titles::Titles, project_content::ProjectContent, project_info::ProjectInfo,
    };

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
        let titles: ProjectInfo =
            serde_json::from_str(json_data).expect("Failed to parse json to ProjectInfo");

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
        let titles: Titles =
            serde_json::from_str(json_data).expect("Failed to parse the json to Titles");

        println!("{:#?}", titles);
    }

    #[test]
    fn test_project_content_conversion() {
        let json_data = r#"
{
  "projectContent": {
    "contributions": {
      "description": "This project contributes an automated tool for generating project reports directly from PlantUML specifications, enhancing efficiency and consistency in software documentation. It also helps improve integration between textual documentation and visual design aspects of software.",
      "steps": [
        "Reviewing objectives and outcomes to highlight alignment with project goals.",
        "Gathering supporting data from testing and validation phases to demonstrate effectiveness.",
         "Drafting a summary of contributions, emphasizing improvements in workflow and integration."
      ]
    },
    "futureWork": {
      "description": "Future work includes integrating the system with external data sources, expanding support for additional diagram types, and exploring web-based user interface options for inputting PlantUML and customizing report generation. Additionally, research into improving the layout and design of the generated reports should be considered.",
      "steps": [
        "Research potential technology integrations, such as linking to Jira or Git for more context.",
        "Creating a roadmap for user interface development to broaden accessibility.",
         "Considering methods for layout improvements and design customization of the reports."
      ]
    },
    "implementation": {
      "description": "The system was implemented using Python for the parsing and report generation logic. Jinja2 templates were used for generating HTML reports, and ReportLab was used to produce PDF reports. Core functionality includes parsing PlantUML, extracting relevant information, and formatting it into reports. Additional features include generating diagrams, embedding textual descriptions, and creating a summary.",
      "steps": [
        "Detailing programming languages and techniques, specifically Python, Jinja2, and ReportLab.",
        "Outlining core functionality, including parsing, extraction, formatting, and report generation.",
         "Describing additional features like diagram integration and summary creation, and discussing testing, optimization, and code review."
      ]
    },
    "literatureReview": {
      "description": "The literature review identified existing tools and techniques for parsing UML specifications and generating reports. It also explored various approaches to diagram rendering and document formatting. The review highlighted a gap in tools that automatically generate reports from PlantUML specifications, leading to the focus on automation.",
      "steps": [
        "Identifying and organizing relevant papers about UML parsing, report generation, and diagram rendering.",
        "Analyzing methodologies, technologies, and research gaps in existing documentation tools.",
         "Summarizing findings and discussing implications for the project’s goals and approach."
      ]
    },
    "methodology": {
      "description": "The project employed an iterative approach involving design, implementation, testing, and refinement. Key steps included parsing PlantUML, extracting diagram information, generating both PDF and HTML reports, and testing the outputs against various inputs. The methodology was structured around the design of a clear, modular system that could be easily extended.",
      "steps": [
        "Describing the project design and architecture, emphasizing a modular design for ease of extension.",
        "Detailing the implementation steps, including core feature design and integration.",
        "Explaining the testing strategy, including test cases and scenario development."
      ]
    },
    "problemStatement": {
      "description": "The problem this project addresses is the manual effort required to generate project reports from PlantUML specifications. Current workflows often involve manually extracting diagrams and information, leading to inconsistencies and errors. The need for automation arises from the desire to improve efficiency and accuracy in documentation.",
      "steps": [
        "Background research and data collection to establish the context and prevalence of the issue.",
        "Defining the scope of the problem, especially in the context of software engineering.",
        "Analyzing the impact of manual documentation on project efficiency and data consistency, along with the formulation and validation of the problem statement."
      ]
    },
    "researchQuestions": {
      "description": "The main research questions include: How can PlantUML specifications be automatically parsed to extract diagram and textual information effectively? Can a reliable system generate comprehensive reports in both PDF and HTML formats? What is the best method for integrating extracted textual descriptions within these reports? How can the system be designed to ensure its reliability and maintainability?",
      "steps": [
         "Breaking down the main topic of automated report generation into clear and measurable questions.",
         "Aligning each question with research objectives to ensure clarity and direction.",
          "Prioritizing questions based on their importance and feasibility in achieving project goals."
      ]
    },
    "resultsAndAnalysis": {
      "description": "The project produced a system capable of generating accurate project reports in both PDF and HTML formats from given PlantUML files. Analysis showed the system could correctly parse various PlantUML structures, extract necessary information, and integrate diagrams with textual descriptions.  Performance analysis identified potential bottlenecks which were addressed through iterative improvements. The results also highlighted the importance of design clarity and modularity in complex software systems.",
       "steps": [
         "Data compilation and summarization of project results, and performance metrics.",
        "Presenting data and findings with visual representations and analysis techniques.",
        "Discussing any limitations encountered and recommending areas for further research."
       ]
    },
    "systemDesign": {
      "description": "The system consists of a parser module to read and extract information from PlantUML, a data storage module to handle parsed data, and format modules for PDF and HTML report generation. The parser utilizes regular expressions and a state machine approach. The design emphasizes modularity and ease of future extensions.",
      "steps": [
        "Gathering system requirements, including input and output specifications and formats.",
        "Designing components like the parser, data storage, and report generators, and reviewing algorithms.",
        "Documenting the design, ensuring clear specifications and flow, and going through a design review."
      ]
    },
    "testingAndValidation": {
      "description": "The testing process involved unit tests for each module, integration tests for module interactions, and system tests using a range of PlantUML inputs. The system was validated for correct parsing, accurate diagram generation, and report output. Reports were generated and inspected for accuracy and consistency against expected outputs.",
       "steps": [
         "Creating a test plan for unit, integration, and system testing and creating test cases.",
        "Executing all test cases, collecting results, and analyzing to identify defects.",
        "Documenting identified issues, implementing fixes, and retesting after each fix."
      ]
    }
  }
}
        "#;

        let project_content: ProjectContent =
            serde_json::from_str(json_data).expect("Failed to parse JSON to ProjectContent");

        println!("{:#?}", project_content);
    }
}
