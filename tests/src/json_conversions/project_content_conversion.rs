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

    let project_content: docuai::docu_requests::responses::project_content::ProjectContent =
        serde_json::from_str(json_data).expect("Failed to parse JSON to ProjectContent");

    println!("{:#?}", project_content);
}
