## Chapter 2: System Analysis and Requirements

This chapter details the system analysis for the Automated Graduation Project Report Generator, outlining the data model, system requirements, proposed solutions, and alternative considerations.

### 2.1 Data Modeling Diagrams

@startuml
!include <c4/C4_Context>
System_Boundary(ProjectReportGenerator) {
  Entity(ProjectData) {
    -- Attributes --
    String projectTitle
    String problemStatement
    String methodology
    String results
    String authorName
    String studentID
  }

  Entity(ReportTemplate) {
    -- Attributes --
    String templateName
    String structure
    String style
  }

  Entity(GeneratedReport) {
    -- Attributes --
    String reportContent
    String reportFormat
    String diagramData
  }

  Rel(ProjectData, ProjectReportGenerator, "Input Project Data")
  Rel(ReportTemplate, ProjectReportGenerator, "Select Template")
  Rel(ProjectReportGenerator, GeneratedReport, "Generate Report")
  Rel(GeneratedReport, Output, "Output Generated Report")

}
@enduml

**Diagram Explanation:**

The Entity-Relationship Diagram (ERD) depicts the core data structures.  `ProjectData` represents the input data from the student, encompassing project title, problem statement, methodology, results, and author information.  `ReportTemplate` holds various report structures and styles. The `GeneratedReport` entity captures the final report, including the generated text and the output format (e.g., PDF, HTML).  The relationships highlight the flow of data through the system: project data is input; a template is selected; and a generated report is output.


### 2.2 System Requirements

#### 2.2.1 Clients, Customers, and Users

The intended users are:

* **Students:**  Submit project data, access and download reports, and review generated content.  Their needs include a user-friendly interface, error messages, and clear generation process guidance. Pain points include potential time constraints, reliance on accurate data input, and the unknown quality of the automated report.
* **Faculty Advisors:** Review generated reports, provide feedback, and offer guidance to the students. They require access controls, feedback mechanisms, and the ability to compare the generated report with standard formats.  Pain points include a lack of human touch, potential errors in understanding project specifics.


#### 2.2.2 Functional and Data Requirements

* **Functional Requirements:**
    * Input Project Data: The system must accept project details (text and potentially images or code snippets) from the user.
    * Template Selection:  Users must be able to choose from predefined report templates.
    * Report Generation:  The system should automatically generate a report based on input data and the selected template.
    * Report Output: The system must provide download options for generated reports (PDF, DOCX, HTML).
* **Data Requirements:**
    * Input: Project data in various formats (text, images, potentially code snippets).
    * Storage: The system must store project data and generated reports in a secure and accessible manner.
    * Manipulation: Data must be processed to extract key information for report generation.
    * Output: Reports should be well-formatted and visually appealing, ideally including automatically generated diagrams from input data.


#### 2.2.3 Non-functional Requirements

* **Look and Feel Requirements:** The system should have a clean, modern design, using intuitive navigation and clear visual cues.  Aesthetics should follow the university's branding guidelines, with a focus on readability and accessibility.
* **Usability Requirements:**  The system should be intuitive and easy to use.  Clear instructions and error messages should be provided to guide the user.
* **Security Requirements:**  User authentication and authorization mechanisms are essential to protect sensitive data. Data encryption and secure storage are crucial.
* **Performance Requirements:** The system should generate reports within a reasonable time frame, considering potential data volume. Scalability is important to handle an increasing number of user requests.
* **Portability Requirements:**  The application should be adaptable to various operating systems (Windows, MacOS, Linux) and mobile platforms.


### 2.3 Proposed Solutions

The proposed solution is to develop a multi-tiered system utilizing FlutterFlow for the UI, the Gemini API for report summarization and text generation, and PlantUML for diagram generation. This allows automation of report-writing tasks.  The strengths include automation, potential cost savings, and improved report quality.  Weaknesses include the possibility of errors in summarization and understanding nuances.  Feasibility is high, given the availability of the Gemini API and FlutterFlow. Technical complexity is moderate.


### 2.4 Alternative Solutions

* **Manual Report Generation:**  This solution is time-consuming, error-prone, and does not scale.
* **Customized Report Template Development:**  Creating custom report templates from scratch would require significant time and effort, without offering automation.


**Justification for Rejection of Alternatives:**  Manual report generation lacks efficiency and scalability.  Custom report development defeats the automation goal. The chosen solution, while presenting challenges, offers a more complete approach by addressing report structuring and automation.
