pub const PROJECT_DETAILS_TEMPLATE: &str = r#"
{
        "projectDetails": {
            "deploymentPhase": {
                "deploymentPlan": "The deployment plan includes a staged roll-out, starting with a pilot group of students and faculty. The system will be deployed on the university's server, with initial testing on a staging environment before moving to production. A detailed rollback plan will also be available. The system will be accessed through a web interface by users. Any specific security protocols will be setup prior to launch.",
                "userTraining": "User training sessions will be conducted for both students and faculty. Training materials, including user manuals and video tutorials, will be made available. Specific training will be provided based on the specific roles users have in the system (such as student or faculty roles). A help desk will be setup for support."
            },
            "designPhase": {
                "databaseSchema": "The database schema will consist of tables for users, project proposals, faculty reviews, and system logs. A relational database like PostgreSQL will be used. Entity relationship diagrams will show table relationships. Specific column types, keys and constraints will be setup following database design best practices.",
                "systemArchitectureDiagrams": "The system architecture will follow a three-tier approach with a presentation layer (frontend), an application layer (backend), and a data layer (database). Diagrams will use a layered style and show component interactions. The APIs will be well-defined for data transfer and operations.",
                "userInterfaceMockups": "User interface mockups will be created using wireframing and prototyping tools. These mockups will illustrate the user flows for submitting proposals, reviewing proposals and managing user accounts. Specific attention will be made to ease of use and accessibility. User feedback will be incorporated during design."
            },
            "documentation": {
                "technicalDocumentation": "Technical documentation will include detailed information on the system's architecture, database schema, API endpoints, and code structure. Code comments will be comprehensive. Diagrams of the system will also be included in the document. This will be targeted to IT and developers for maintenance and improvement.",
                "userManual": "The user manual will provide step-by-step instructions on how to use the system, including how to submit project proposals, review proposals, and track progress. This document will be made available to all users of the system, specifically to students and faculty. It will include screen shots and specific examples for various scenarios."
            },
            "hardwareRequirements": {
                "computingPower": {
                    "cpu": "Server-grade multi-core processor (e.g., Intel Xeon or AMD EPYC)",
                    "ram": "Minimum 16 GB RAM, recommended 32 GB or more",
                    "storage": "Minimum 500 GB SSD for the system and database, more if necessary"
                },
                "networkConnectivity": "Reliable high-speed internet connection with sufficient bandwidth to handle user traffic.",
                "peripheralDevices": {
                    "inputDevices": "Standard keyboards, mice, and touch-screen devices.",
                    "outputDevices": "Standard monitors and printers (optional)"
                },
                "scalability": "The hardware should be able to scale up with user and data growth. Scalable server configurations will be implemented based on usage patterns. Cloud infrastructure may be used to facilitate horizontal scaling as necessary."
            },
            "implementationPhase": {
                "codeDevelopment": "The code will be developed using Python with the Django framework for the backend and HTML, CSS, and JavaScript for the frontend. A Git repository will be used for version control and collaborative coding practices. All development will follow agreed coding standards.",
                "hardwareSetup": "The server will be set up with the specified hardware, including installing necessary drivers and configuring the network. All server hardware will be configured to specifications. Specific server settings will be setup with necessary backups and security protocols.",
                "softwareInstallation": {
                    "installApplications": "All necessary software applications, including the web server (e.g., Apache or Nginx), database system (e.g., PostgreSQL), and development tools, will be installed and configured.",
                    "installOs": "A compatible server operating system, such as Linux (e.g., Ubuntu or CentOS), will be installed. Specific OS versions will be selected for best stability and security."
                }
            },
            "maintenanceAndSupport": {
                "maintenancePlan": "The maintenance plan includes regular backups, software updates, security patching, and performance monitoring. It will be scheduled and managed by IT admin staff. Regular reviews will be made on the maintenance tasks to ensure system is running efficiently.",
                "supportServices": "Support services will include a help desk for user support, bug fixes, and ongoing technical support for system maintenance and improvements. Support will be provided to both students, faculty and IT admin staff."
            },
            "overview": "The Automated Graduation Project System aims to streamline and automate the process of managing graduation project proposals, from submission to final approval. It seeks to replace the current manual process, which is inefficient and time-consuming.",
            "projectPlanning": {
                "allocateResources": "Resources will be allocated to the development team, including software tools, hardware, and development time. A detailed schedule will outline these resources for best use. Specific team members will be given clear responsibilities and deadlines.",
                "defineScope": "The project scope will focus on the submission, review, and approval of graduation project proposals. The system will not cover project execution or final submissions. A clearly defined project scope will minimize scope creep.",
                "establishTimeline": "The project will be completed over a six-month period, with key milestones for design, development, testing, and deployment. A Gantt chart will be used to visualize the project schedule. Specific timelines for tasks will be set with reasonable buffers.",
                "identifyStakeholders": "The stakeholders include students, faculty, the university's IT department, and the academic committee overseeing graduation projects. Stakeholder roles will be clearly defined. Regular meetings and updates will be held to keep stakeholders informed."
            },
            "requirementsGathering": {
                "compileUserRequirements": "User requirements will be compiled from interviews, documentation review, and stakeholder feedback. Documentation of use cases and user stories will be created. User requirements will be prioritized based on value to stakeholders.",
                "conductInterviews": "Interviews will be conducted with students, faculty, and administrators to gather information on their needs and expectations for the system. All interviews will be documented and analyzed. Specific user roles will be represented.",
                "reviewDocumentation": "Existing documentation related to graduation project management, such as university guidelines and procedures, will be reviewed to understand current processes. Any documentation related to existing IT systems will also be reviewed to understand any integrations needed. Key policies will be documented."
            },
            "softwareRequirements": {
                "databaseRequirements": "A relational database management system (RDBMS) such as PostgreSQL will be required to store data related to users, projects, and reviews. The database must be reliable, scalable and secure. It will be designed with best database design principles. Specific tables, columns, keys and relationships will be configured.",
                "operatingSystem": "A server operating system like Linux (e.g., Ubuntu or CentOS) will be required for hosting the web application and database server. Specific version needs to be compatible with all other system software. Regular security patches will be setup.",
                "securitySoftwareNeeds": "Security software, including firewalls and intrusion detection systems, will be required to protect the system from cyber threats. SSL certificates will be setup. Secure coding practices will be followed and a plan for updates and patching will be in place.",
                "softwareApplications": {
                    "listApplications": "The software applications will include Python (with Django), PostgreSQL, and a web server (such as Apache or Nginx). Specific IDEs and code editors will be used for development work. Testing and bug tracking tools will be used.",
                    "versionCompatibility": "All software applications must be compatible with each other and with the operating system. Software versions will be managed and tested prior to deployment. All dependencies must be clearly documented. The system must be tested before deployment."
                }
            },
            "testingPhase": {
                "createTestPlan": "A comprehensive test plan will be created that includes unit tests, integration tests, and user acceptance tests (UAT). Specific test cases and scenarios will be created. The test plan will be created based on requirements and user stories.",
                "integrationTesting": "Integration testing will be performed to ensure that the various components of the system work together seamlessly. APIs will be tested for correct data transfer and any dependencies will be tested. Specific integration scenarios will be setup.",
                "unitTesting": "Unit tests will be written for each individual component of the system to ensure that they function correctly in isolation. Code quality will also be tested. Automated test scripts will be generated for testing core functionalities. All bugs found will be documented.",
                "userAcceptanceTesting": "User acceptance testing (UAT) will be conducted with a group of students and faculty to ensure that the system meets their needs and expectations. Any user feedback will be reviewed. Specific feedback mechanisms will be created. The system will only be approved after successful UAT."
            }
        }
}
"#;
