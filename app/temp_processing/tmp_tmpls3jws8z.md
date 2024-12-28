## Chapter 2: System Analysis and Requirements

This chapter analyzes the requirements of the Automated Graduation Project Report Generator system. It details the data structures, functionalities, and non-functional requirements, along with proposed and alternative solutions.

### 2.1 Data Modeling Diagrams

The following Entity-Relationship Diagram (ERD) illustrates the core data structures and relationships within the system.

![PlantUML Diagram 1](G:\Docudec\app\temp_processing\images_folder\block1.png)

**Explanation:**

The ERD depicts three primary entities: `Project`, `Chapter`, and `Diagram`.  `Project` holds general project information. `Chapter` represents individual chapters within a project report.  `Diagram` stores diagram data (as PlantUML code). The relationships demonstrate how projects contain chapters, diagrams, and report sections.  Future expansion could include entities for tables, figures, and other report elements.


### 2.2 System Requirements

#### 2.2.1 Clients, Customers, and Users

The intended users are faculty advisors, project supervisors, and graduating students.

* **Faculty Advisors/Supervisors:** Need a system to easily generate comprehensive reports on student projects, saving time and effort. They may need customization options for specific report sections or formatting.
* **Graduating Students:** Need a system to efficiently input project details and generate their report, which is crucial for meeting deadlines.
* **Pain Points:** Difficulty in managing multiple reports, time-consuming manual report writing, lack of a unified system for generating report sections, inconsistencies in report formats across different projects, and difficulties in incorporating various diagrams.


#### 2.2.2 Functional and Data Requirements

* **Functional Requirements:**
    * Input project details, chapter content, and diagrams via a web interface.
    * Generate the report structure (sections like introduction, methodology, results).
    * Generate and incorporate project-specific diagrams using PlantUML code input.
    * Generate report in PDF or HTML format.
    * Option to save and retrieve previous reports.
    * Option for various report templates.

* **Data Requirements:**
    * Input data: Project details, chapter content, diagram code, and report section data.
    * Data Storage: Use a database to store project data, chapter content, and diagram information.
    * Data Manipulation: The system should properly process and format the input data to generate the desired report.
    * Output data:  Formatted report (PDF or HTML), report content, and diagram files (PNG/SVG).


#### 2.2.3 Non-functional Requirements

* **Look and Feel:** A visually appealing, user-friendly interface. Use a consistent color scheme and clear labelling.  Use responsive design principles for cross-platform compatibility.
* **Usability:** The system should be intuitive and efficient.  Clear instructions and intuitive navigation should be incorporated. User feedback mechanisms should be integrated to enable system improvements.
* **Security:** Authentication to protect project data. Secure storage and transmission of data. Strong encryption for sensitive information.
* **Performance:** Fast report generation times, and scalable architecture to handle large projects and reports.
* **Portability:** The system should be able to be adapted to different web browsers and operating systems.

### 2.3 Proposed Solutions

The primary solution revolves around using FlutterFlow for the user interface, Gemini API for natural language report generation, and a backend framework (e.g., Python with Flask) for data processing. PlantUML for diagram handling will be integrated into the FlutterFlow front-end. This approach utilizes readily available, powerful APIs and technologies to deliver a streamlined solution.


### 2.4 Alternative Solutions

Alternative solutions considered included using solely GPT-3/other LLMs for complete report generation. However, this approach poses risks related to generating inaccurate or illogical sections.  A combination of user-defined input and automated generation was deemed a more robust solution to ensure the accuracy and quality of the output.

**(Note: Further details on each aspect of the system requirements, including specific API integrations and technical specifications, will be explored in subsequent chapters.)**
