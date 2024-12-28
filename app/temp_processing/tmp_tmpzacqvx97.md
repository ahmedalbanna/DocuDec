## Chapter 2: System Analysis and Requirements

This chapter details the system analysis, data modeling, and requirements for the automated graduation project report generation system.  It outlines the target users, functional and non-functional requirements, and proposed solutions, along with alternative approaches considered.

### 2.1 Data Modeling Diagrams

**2.1.1 Entity-Relationship Diagram (ERD)**

@startuml
Entity(Project) {
    projectId : int
    projectName : string
    projectDescription : string
    dueDate : date
}

Entity(Section) {
    sectionId : int
    sectionName : string
    content : string
    projectId : int
}

Entity(Report) {
    reportId : int
    reportName : string
    project : int
    generationDate : date
}

Entity(Document) {
    documentId : int
    documentType : string
    filePath : string
    projectId : int
    content : string
}

Report "1" -- "*" Section : contains
Project "1" -- "*" Report : has
Project "1" -- "*" Document : has
Document "1" -- "1" Section : references

@enduml

**2.1.2 Explanation of the ERD:**

The ERD models the relationships between key entities in the system. `Project` represents a student's graduation project, including details like name, description, and due date.  `Section` represents different sections of the report (Introduction, System Analysis, etc.).  `Report` encapsulates the complete generated report and the date it was created.  `Document` signifies any input documents related to the project, such as text files or spreadsheets.  Relationships are defined to show how these entities connect, for example, a `Project` has many `Reports`, and a `Report` contains multiple `Sections`.  A `Project` can have multiple `Documents` that contribute to the `Report` generation.

### 2.2 System Requirements

**2.2.1 Clients, Customers, and Users:**

* **Students:** Input project documentation, access generated reports, and potentially customize the report's layout.
* **Faculty Advisors:** Review generated reports, provide feedback on the report content, and potentially modify report parameters.
* **System Administrators:** Manage user accounts, control access to the system, and potentially configure report templates.

**2.2.2 Functional and Data Requirements:**

* **Input:** The system must accept various document formats (e.g., .docx, .pdf, .txt, .csv).
* **Processing:** The system will use NLP (Natural Language Processing) techniques to extract relevant information from input documents. The Gemini API will parse these documents, categorize information, and summarize sections. PlantUML will generate system diagrams based on defined project requirements.
* **Output:** The system must generate a complete project report in various formats (e.g., .pdf, .docx, .html).
* **Storage:** The system needs a database to store project data, user profiles, and generated reports.

**2.2.3 Non-functional Requirements:**

* **Look and Feel Requirements:**  The FlutterFlow interface will use a modern, intuitive design, clear labeling, and color schemes to make it visually appealing and easy to navigate.
* **Usability Requirements:** The system must be easy to learn and use for both students and faculty.  Navigation should be logical, and error messages should be clear.
* **Security Requirements:**  Authentication using user accounts and passwords, encryption of stored data, and restricted access to system functions will protect sensitive project information.
* **Performance Requirements:**  The system must generate reports within a reasonable timeframe, depending on the size of the input documents and the complexity of the project.
* **Portability Requirements:**  The system will be designed to run on various platforms (web-based application) for improved accessibility and adaptability to different environments.

### 2.3 Proposed Solutions

The proposed solution involves leveraging the Gemini API for NLP tasks, FlutterFlow for the user interface, and PlantUML to generate diagrams. The input documents are parsed to extract meaningful project information, then the Gemini API is used to generate text for different report sections. PlantUML diagrams will be automatically generated based on the parsed information.

### 2.4 Alternative Solutions

Other solutions considered included using open-source NLP libraries and custom web frameworks for the UI.  However, FlutterFlow's pre-built components and user-friendly interface, coupled with the Gemini API's strong natural language processing capabilities, offered a more efficient and user-friendly solution with faster development time, while retaining sufficient customization capabilities.
