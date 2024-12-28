from pydantic import BaseModel


# Cover Pages: These are the initial pages that introduce the project and provide basic information.
class CoverPages(BaseModel):
    project_title: str


def cover_pages(cover_pages: CoverPages):
    return


# Title Page: The title page should include the project title, your name, department, university, and the date.
class TitlePage(BaseModel):
    student_name: str
    department: str
    university: str
    date: str


def title_page(title_page: TitlePage):
    return


# Contact Information: This section provides your contact information, including email address and phone number.
class ContactInformation(BaseModel):
    email_address: str
    phone_number: str


def contact_information(contact_information: ContactInformation):
    return


# Intellectual Property Right Declaration: A statement affirming that the project is your original work and that you have obtained necessary permissions for any third-party resources used.
class IntellectualPropertyRightDeclaration(BaseModel):
    intellectual_property_right_declaration: str


def intellectual_property_right_declaration(
        intellectual_property_right_declaration: IntellectualPropertyRightDeclaration):
    return


# Anti-Plagiarism Declaration: A statement affirming that the project is your own work and that you have not plagiarized any content.
class AntiPlagiarismDeclaration(BaseModel):
    anti_plagiarism_declaration: str


def anti_plagiarism_declaration(anti_plagiarism_declaration: AntiPlagiarismDeclaration):
    return


# Acknowledgement: A section expressing gratitude to individuals who have supported you in completing the project.
class Acknowledgement(BaseModel):
    acknowledgement: str


def acknowledgement(acknowledgement: Acknowledgement):
    return


# Abstract: A brief summary of the project, outlining its purpose, methods, and key findings.
class Abstract(BaseModel):
    abstract: str


def abstract(abstract: Abstract):
    return


# Table of Contents: A list of all chapters, sections, and sub-sections included in the project report, along with their corresponding page numbers.
class TableOfContents(BaseModel):
    table_of_contents: str


def table_of_contents(table_of_contents: TableOfContents):
    return
