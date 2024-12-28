from enum import Enum
from typing import Dict, List

import json

from pathlib import Path


class Methodoligy(Enum):
    WaterFlow = ("WaterFlow",)
    Ageal = "Ageal"


class Stylies(Enum):
    APA = "APA"
    EEI = "EEI"


class Diagram(Enum):
    ER = "ER"
    Class = "Class"
    UseCase = "UseCase"
    Sequence = "Sequence"
    Activity = "Activity"
    Object = "Object"
    Archimet = "Archimet"


class Progress(Enum):
    Still = "Still"
    Complete = "Complete"


class Report:
    def __init__(self, abstract: str, style: Stylies, diagram: Diagram) -> None:
        self.abstract = abstract
        self.style = style
        self.diagram = diagram

        self.accepted = False
        self.__current_chapter_content: Dict[str, str] = {}

        self.__chapters: List[Dict[str, str]] = []

        self.__current_chapter_number: int = 1

    def _get_chap_api(self):
        self.__current_chapter_content = {
            f"chap{self.__current_chapter_number}": "hello"
        }  # get from the api with the chapter number

        print(self.__current_chapter_content)

        if self.accepted:
            self.__current_chapter_number = self.__current_chapter_number + 1

    def gen_chap(self):
        self._get_chap_api()

    def reject(self):
        self.accepted = False
        self.__current_chapter_content = {}
        self.gen_chap()

    def accept(self) -> Progress:
        self.accepted = True
        if self.__current_chapter_number > 8:
            return Progress.Complete
        else:
            self.__chapters.append(self.__current_chapter_content)
            self.__current_chapter_content = {}
            return Progress.Still

    def gather_chaps(self):
        project_dir = Path(__file__).parent / "projects"
        reports_dir = project_dir / "reports"

        project_dir.mkdir(exist_ok=True)
        reports_dir.mkdir(exist_ok=True)

        for i, chapter in enumerate(self.__chapters, start=1):
            with open(reports_dir / f"chapter_{i}.md", "w", encoding="utf-8") as file:

                text: str = ""

                for key, value in chapter.items():
                    text = text + f"##{key}\n\n{value}\n\n"

                file.write(text)

            with open(
                reports_dir / f"chapter_{i}.json", "w", encoding="utf-8"
            ) as json_file:
                json.dump(chapter, json_file, ensure_ascii=False, indent=4)


class Project:
    def __init__(
        self, title: str, objective: str, methodoligy: Methodoligy, audince: list[str]
    ) -> None:
        self.title = title
        self.objective = objective
        self.methodoligy = methodoligy
        self.audince = audince

        # whether the user accepted the abstraction or not
        self.accepted = False
        self.__current_abstraction: str = ""

    def gen_project(self):
        self.__current_abstraction: str = (
            "initial abstraction"  # posts the project to the api to get the abstraction
        )
        print(self.__current_abstraction)

    def accept(self, style: Stylies, diagram: Diagram) -> Report:
        self.accepted = True
        report = Report(self.__current_abstraction, style, diagram)
        return report

    def reject(self):
        self.gen_project()


project = Project("hotel", "skjdn", Methodoligy.Ageal, ["a"])

project.gen_project()
report = project.accept(Stylies.EEI, Diagram.Archimet)

report.gen_chap()
report.accept()

report.gen_chap()
report.accept()
report.gather_chaps()
