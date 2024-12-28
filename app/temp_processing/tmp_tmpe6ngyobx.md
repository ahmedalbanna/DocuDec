## Chapter 2: System Analysis and Requirements

This chapter details the system analysis, data modeling, and requirements for the Automated Generation of Graduation Project Reports system.  It outlines the intended users, their needs, the functionalities, and the critical non-functional aspects that ensure the system's success.

### 2.1 Data Modeling Diagrams

The following Entity-Relationship Diagram (ERD) depicts the core data structures and relationships within the project report generation system.

![PlantUML Diagram 1](G:\Docudec\app\temp_processing\images_folder\block1.png)

**Diagram Explanation:**

The ERD models the relationships between `Project`, `Student`, `DocumentSection`, and `Report` entities.  Each `Project` has multiple `DocumentSection` instances (e.g., Introduction, Methodology, Results). A `Project` generates a single `Report`, and a `Student` can submit multiple `Project`s. The diagram showcases the structured data flow crucial for the automated report generation process.  Crucially, the `sectionContent` for each `DocumentSection` is crucial input to the Gemini API and the report generation process. The `reportFormat` attribute allows for customization of the final report format.

### 2.2 System Requirements

#### 2.2.1 Clients, Customers, and Users

* **Project Supervisors:**  Need a system that quickly generates professional-looking reports from student submissions, facilitating review and feedback.  They expect accuracy and clarity in the generated text and visuals.  Potential pain points include conflicting formatting preferences and differing project complexities.
* **Students:**  Require a user-friendly interface to input their project documents.  They need a system that handles different document formats and provides clear feedback on errors or inconsistencies.
* **System Administrators:** Responsible for maintaining the system's infrastructure, security, and updates.  They need access to configuration settings, logs, and system performance metrics.

#### 2.2.2 Functional and Data Requirements

* **Input:** The system must accept various document formats (e.g., .docx, .pdf, .txt).  Conversion to a consistent internal data format is required.
* **Storage:** Data must be stored securely and efficiently in a database. JSON will be used for intermediate representation for data exchange between the various system components.
* **Manipulation:**  The Gemini API will be used to process the input content, extract key information, and synthesize report sections.  This includes content summarization, formatting, and referencing.
* **Output:**  The system should generate project reports in a predefined format (e.g., .pdf, .docx), with clear and consistent structure.  Generated reports should include accurate citations and references (if present in the input).

#### 2.2.3 Non-functional Requirements

* **Look and Feel Requirements:** The FlutterFlow application should have a modern, intuitive design, aligned with the university branding. The system should be responsive across different screen sizes. Accessibility guidelines (WCAG) will be adhered to.
* **Usability Requirements:** The system should be user-friendly, allowing easy navigation and input of project documents.  Clear error messages and prompts will prevent user confusion and expedite the input process.
* **Security Requirements:** Authentication and authorization mechanisms will be implemented to protect user accounts and project data.  Encryption will be used for sensitive data transmission and storage.
* **Performance Requirements:** The system must generate reports within a reasonable time frame for all document sizes and complexities.  Scalability considerations will be factored in for future growth.
* **Portability Requirements:**  The system should be platform-independent, functioning on different operating systems (Windows, macOS, Linux) and supported browsers.


### 2.3 Proposed Solutions

Our proposed solution involves leveraging FlutterFlow for the user interface, the Gemini API for natural language processing, and a backend structure to integrate these elements.  This approach allows flexibility in input formats and customization of report layouts.

### 2.4 Alternative Solutions

Other options, such as using simpler NLP models or custom Python scripts for report generation, were considered but discarded due to the need for greater accuracy, the potential cost and complexity of implementing a completely custom solution, and the need for a readily usable and visually appealing front-end. The Gemini API offers a balanced solution for both accuracy and ease of integration.
