from prompts import (
Cover_Pages_Prompt,
Chapter_1_Prompt,
Chapter_2_Prompt,
Chapter_3_Prompt,
References_Prompt
)

chapter_prompts = [
    #Cover_Pages_Prompt,
    Chapter_1_Prompt,
    Chapter_2_Prompt,
    Chapter_3_Prompt,
    # Chapter_4_Prompt,
    # Chapter_5_Prompt,
    # Chapter_6_Prompt,
    # Chapter_7_Prompt,
    References_Prompt,
    # Glossary_Prompt,
    # Appendices_Prompt,
]


def get_chapter_prompts():
    return chapter_prompts
