import unittest

from app.gemini.misc import create_contact_information_markdown
from app.schemas.schemas import ProjectDetails


class TestContactInformationMarkdown(unittest.TestCase):
    def test_create_contact_information_markdown_with_all_fields(self):
        project_details = ProjectDetails(
            student_name="Ahmed",
            email_address="ahmed@example.com",
            phone_number="123-456-7890",
            student_id_number="1234567",
            university="University of Example",
            supervisor_name="Dr. Supervisor",
            supervisor_email="supervisor@example.com",
            supervisor_phone="987-654-3210",
            head_of_department="Head of Department",
            acknowledgement="Many people",
            date = "2025/2/5"

        )
        markdown = create_contact_information_markdown(project_details)
        self.assertIn("Ahmed", markdown)
        self.assertIn("ahmed@example.com", markdown)
        self.assertIn("123-456-7890", markdown)
        self.assertIn("1234567", markdown)
        self.assertIn("University of Example", markdown)
        self.assertIn("Dr. Supervisor", markdown)
        self.assertIn("supervisor@example.com", markdown)
        self.assertIn("987-654-3210", markdown)
        self.assertIn("Head of Department", markdown)
        self.assertIn("Many people", markdown)
        self.assertIn("2025/2/5", markdown)

    def test_create_contact_information_markdown_with_missing_fields(self):
        project_details = ProjectDetails(
            student_name="Ahmed",
            email_address=None,
            phone_number=None,
            student_id_number=None,
            university=None,
            supervisor_name=None,
            supervisor_email=None,
            supervisor_phone=None,
            head_of_department=None,
            acknowledgement=None,
            date = None
        )
        markdown = create_contact_information_markdown(project_details)
        self.assertIn("Ahmed", markdown)
        self.assertNotIn("None", markdown)
