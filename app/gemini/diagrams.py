from ai_setup import *

def classDiagram(input):
    history = [
        {
        "role": "user",
        "parts": [
            "Generate Class Diagram\nUser:\nStores user information (name, email, phone number, etc.)\nManages user login and registration\nTracks user booking history and preferences\nMovie:\nStores movie details (title, genre, duration, release date, etc.)\nManages showtimes and screen assignments\nShow:\nRepresents a specific screening of a movie\nStores showtime, screen number, and available seats\nHandles seat booking and cancellation\nTicket:\nRepresents a purchased ticket\nStores ticket ID, user information, movie details, showtime, and seat number\nGenerates ticket confirmation and barcode\nPayment:\nHandles payment processing (online or offline)\nStores payment details (amount, payment method, transaction ID, etc.)\nKey Relationships:\n\u2022\tUser can book many Tickets.\n\u2022\tMovie can have many Shows.\n\u2022\tShow can have many Tickets.\n\u2022\tTicket is associated with a User and a Show.\n\u2022\tTicket can involve a Payment.\n"
        ]
    },
    {
        "role": "model",
        "parts": [
            "```plantuml\n@startuml\nclass User {\n    - userID : int <<key>>\n    - name : string\n    - email : string\n    - phoneNumber : string\n    - bookingHistory : List<Ticket>\n    - preferences : Map<string, string>\n    + login()\n    + register()\n    + bookTicket(show : Show, seats : List<int>) : Ticket\n    + cancelTicket(ticket : Ticket)\n    + viewBookingHistory() : List<Ticket>\n    + updatePreferences()\n}\n\nclass Movie {\n    - movieID : int <<key>>\n    - title : string\n    - genre : string\n    - duration : int\n    - releaseDate : Date\n    - showtimes : List<Show>\n    + addShowtime(showtime : Show)\n    + removeShowtime(showtime : Show)\n}\n\n\nclass Show {\n    - showID : int <<key>>\n    - movie : Movie\n    - showtime : DateTime\n    - screenNumber : int\n    - availableSeats : Set<int>\n    + bookSeats(seats : List<int>) : boolean\n    + cancelSeats(seats : List<int>)\n}\n\nclass Ticket {\n    - ticketID : int <<key>>\n    - user : User\n    - show : Show\n    - seats : List<int>\n    - payment : Payment\n    + generateConfirmation()\n    + generateBarcode() : string\n}\n\nclass Payment {\n    - paymentID : int <<key>>\n    - amount : double\n    - paymentMethod : string\n    - transactionID : string\n    + processPayment()\n}\n\n\n\nUser \"1\" *-- \"*\" Ticket : books\nMovie \"1\" *-- \"*\" Show : has\nShow \"1\" *-- \"*\" Ticket : has\nTicket \"1\" -- \"0..1\" Payment : involves\n\n\n@enduml\n```\n\n\n\n"
        ]
    },
    ]
    class_chat_session = model.start_chat(history)
    response = class_chat_session.send_message(input)
    return response.text

def archimateDiagram(input):
    history = [
        {
        "role": "user",
        "parts": [
            "Generate Archimate Diagram\no\tStudent Dashboard: Personalized progress tracking, upcoming activities and deadlines, performance analytics, assignment submission and grading, message and notification system.\no\tTeacher Dashboard: Class management, student progress tracking, assignment creation and grading, communication tools, resource library.\no\tContent Management: Upload and manage educational resources, create and organize courses and modules, version control.\no\tCalendar and Scheduling: Integrated calendar, automated reminders.\no\tSecure User Authentication and Authorization: Role-based access control, data privacy and security.\no\tMobile App: Core functionality on mobile devices."
        ]
    },
    {
        "role": "model",
        "parts": [
            """```plantuml
@startuml
!define AWSSkin https://raw.githubusercontent.com/plantuml-stdlib/plantuml-stdlib/master/aws/aws.puml
!define DEVICONS https://raw.githubusercontent.com/tupadr3/plantuml-icon-font-sprites/v2.5.0/devicons/devicons.puml
!define ICONURL https://raw.githubusercontent.com/RicardoNiepel/PlantUML-Icon-Sprite/master/icons
!define ACTOR_COLOR lightblue
!define APPLICATION_COLOR lightgreen
!define BUSINESS_COLOR lightyellow
!define TECHNOLOGY_COLOR lightgrey
!define PHYSICAL_COLOR lightpink
!define DATAA_COLOR lightsalmon

left to right direction
'top to bottom direction
skinparam nodesep 100
skinparam ranksep 500
'skinparam linetype polyline
skinparam linetype ortho

' --- Business Layer ---
package "Business Layer" {
    [Personalized Progress Tracking] as ProgressTracking <<BusinessFunction>> #BUSINESS_COLOR
    [Upcoming Activities and Deadlines] as ActivitiesDeadlines <<BusinessFunction>> #BUSINESS_COLOR
    [Performance Analytics] as PerformanceAnalytics <<BusinessFunction>> #BUSINESS_COLOR
    [Assignment Submission and Grading] as AssignmentGrading <<BusinessFunction>> #BUSINESS_COLOR
    [Message and Notification System] as MessageNotification <<BusinessFunction>> #BUSINESS_COLOR
    [Class Management] as ClassManagement <<BusinessFunction>> #BUSINESS_COLOR
    [Assignment Creation] as AssignmentCreation <<BusinessFunction>> #BUSINESS_COLOR
    [Communication Tools] as Communication <<BusinessFunction>> #BUSINESS_COLOR
    [Resource Library] as ResourceLibrary <<BusinessFunction>> #BUSINESS_COLOR
    [Upload and Manage Educational Resources] as UploadResources <<BusinessFunction>> #BUSINESS_COLOR
    [Create and Organize Courses and Modules] as CourseOrganization <<BusinessFunction>> #BUSINESS_COLOR
    [Version Control] as VersionControl <<BusinessFunction>> #BUSINESS_COLOR
    [Integrated Calendar] as IntegratedCalendar <<BusinessFunction>> #BUSINESS_COLOR
    [Automated Reminders] as AutomatedReminders <<BusinessFunction>> #BUSINESS_COLOR
    [Role-Based Access Control] as AccessControl <<BusinessFunction>> #BUSINESS_COLOR
    [Data Privacy and Security] as DataPrivacy <<BusinessFunction>> #BUSINESS_COLOR
    [Core Functionality on Mobile] as MobileFunctionality <<BusinessFunction>> #BUSINESS_COLOR
    
   
}

' ========================== Application Layer ==========================
package "Application Layer" {
    [Student Dashboard] as StudentDashboard <<ApplicationComponent>> #APPLICATION_COLOR
    [Teacher Dashboard] as TeacherDashboard <<ApplicationComponent>> #APPLICATION_COLOR
    [Content Management] as ContentManagement <<ApplicationComponent>> #APPLICATION_COLOR
    [Calendar and Scheduling] as CalendarScheduling <<ApplicationComponent>> #APPLICATION_COLOR
    [Secure User Authentication and Authorization] as SecureAuth <<ApplicationComponent>> #APPLICATION_COLOR
    [Mobile App] as MobileApp <<ApplicationComponent>> #APPLICATION_COLOR
}

' ========================== Technology Layer ==========================
package "Technology Layer" {
     [Database] as Database <<TechnologyComponent>> #TECHNOLOGY_COLOR
     [Web Server] as WebServer <<TechnologyComponent>> #TECHNOLOGY_COLOR
     [API Gateway] as APIGateway <<TechnologyComponent>> #TECHNOLOGY_COLOR
     [Mobile Server] as MobileServer <<TechnologyComponent>> #TECHNOLOGY_COLOR
     [Authentication Server] as AuthServer <<TechnologyComponent>> #TECHNOLOGY_COLOR
    
}
package "Data Layer" {
    [Student Data] as StudentData <<DataObject>> #DATAA_COLOR
    [Teacher Data] as TeacherData <<DataObject>> #DATAA_COLOR
     [Course Data] as CourseData <<DataObject>> #DATAA_COLOR
     [Assignment Data] as AssignmentData <<DataObject>> #DATAA_COLOR
    [Resource Data] as ResourceData <<DataObject>> #DATAA_COLOR
     [User Data] as UserData <<DataObject>> #DATAA_COLOR
     [Session Data] as SessionData <<DataObject>> #DATAA_COLOR
}
' ========================== Relationships ==========================


StudentDashboard -r-> ProgressTracking : realizes
StudentDashboard -r-> ActivitiesDeadlines : realizes
StudentDashboard -r-> PerformanceAnalytics : realizes
StudentDashboard -r-> AssignmentGrading : realizes
StudentDashboard -r-> MessageNotification : realizes

TeacherDashboard -r-> ClassManagement : realizes
TeacherDashboard -r-> AssignmentCreation : realizes
TeacherDashboard -r-> AssignmentGrading : realizes
TeacherDashboard -r-> Communication : realizes
TeacherDashboard -r-> ResourceLibrary : realizes

ContentManagement -u-> UploadResources : realizes
ContentManagement -u-> CourseOrganization : realizes
ContentManagement -u-> VersionControl : realizes

CalendarScheduling -d-> IntegratedCalendar : realizes
CalendarScheduling -d-> AutomatedReminders : realizes

SecureAuth -d-> AccessControl : realizes
SecureAuth -d-> DataPrivacy : realizes

MobileApp -d-> MobileFunctionality : realizes

StudentDashboard -u-|> APIGateway : access
TeacherDashboard -u-|> APIGateway : access
ContentManagement -u-|> APIGateway : access
CalendarScheduling -u-|> APIGateway : access
MobileApp -u-|> APIGateway : access

WebServer --|> APIGateway : access
MobileServer --|> APIGateway : access
APIGateway --> AuthServer : access

AuthServer --> SecureAuth : provides

APIGateway --> Database : read/write


Database .> StudentData : stores
Database .> TeacherData : stores
Database .> CourseData : stores
Database .> AssignmentData : stores
Database .> ResourceData : stores
Database .> UserData : stores
Database .> SessionData : stores


' ========================== Actors ==========================
actor Student as S
actor Teacher as T
actor Admin as A

' ========================== Actor Relationships ==========================

S -- StudentDashboard : "Uses"
T -- TeacherDashboard : "Uses"
A -- ContentManagement : "Manages"
S -- CalendarScheduling : "Uses"
T -- CalendarScheduling : "Uses"
S -- MobileApp: "Uses"
T -- MobileApp: "Uses"
A -- WebServer: "Uses"

' Annotations
note top of StudentDashboard : Personalized learning experience
note top of TeacherDashboard : Efficient class and student management
note top of ContentManagement : Organized and up-to-date learning resources
note top of CalendarScheduling : Enhanced time management
note top of SecureAuth : Secure access and data protection
note top of MobileApp : Access anywhere

@enduml
```"""
        ]
    },
    ]
    archimate_chat_session = model.start_chat(history)
    response = archimate_chat_session.send_message(input)
    return response.text