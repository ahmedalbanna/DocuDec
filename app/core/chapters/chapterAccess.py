import json
with open(".\\table_of_contents.json", "r") as f:
    table_of_content_data = json.load(f)

# Path Expressions
# [
#   "$['TableOfContents']",
#   "$['TableOfContents']['Appendices']",
#   "$['TableOfContents']['Chapter 1']",
#   "$['TableOfContents']['Chapter 2']",
#   "$['TableOfContents']['Chapter 3']",
#   "$['TableOfContents']['Chapter 4']",
#   "$['TableOfContents']['Chapter 5']",
#   "$['TableOfContents']['Chapter 6']",
#   "$['TableOfContents']['Chapter 7']",
#   "$['TableOfContents']['Glossary']",
#   "$['TableOfContents']['References']",
#   "$['TableOfContents']['Appendices']['Additive Appendices']",
#   "$['TableOfContents']['Appendices']['Attachments']",
#   "$['TableOfContents']['Appendices']['Code Listing']",
#   "$['TableOfContents']['Appendices']['Testing Results']",
#   "$['TableOfContents']['Appendices']['User Manual']",
#   "$['TableOfContents']['Chapter 1']['Sections']",
#   "$['TableOfContents']['Chapter 1']['Title']",
#   "$['TableOfContents']['Chapter 1']['Sections']['1.1']",
#   "$['TableOfContents']['Chapter 1']['Sections']['1.2']",
#   "$['TableOfContents']['Chapter 1']['Sections']['1.3']",
#   "$['TableOfContents']['Chapter 1']['Sections']['1.4']",
#   "$['TableOfContents']['Chapter 2']['Sections']",
#   "$['TableOfContents']['Chapter 2']['Title']",
#   "$['TableOfContents']['Chapter 2']['Sections']['2.1']",
#   "$['TableOfContents']['Chapter 2']['Sections']['2.2']",
#   "$['TableOfContents']['Chapter 2']['Sections']['2.2.1']",
#   "$['TableOfContents']['Chapter 2']['Sections']['2.2.2']",
#   "$['TableOfContents']['Chapter 2']['Sections']['2.2.3']",
#   "$['TableOfContents']['Chapter 2']['Sections']['2.2.3.1']",
#   "$['TableOfContents']['Chapter 2']['Sections']['2.2.3.2']",
#   "$['TableOfContents']['Chapter 2']['Sections']['2.2.3.3']",
#   "$['TableOfContents']['Chapter 2']['Sections']['2.2.3.4']",
#   "$['TableOfContents']['Chapter 2']['Sections']['2.2.3.5']",
#   "$['TableOfContents']['Chapter 2']['Sections']['2.3']",
#   "$['TableOfContents']['Chapter 2']['Sections']['2.4']",
#   "$['TableOfContents']['Chapter 3']['Sections']",
#   "$['TableOfContents']['Chapter 3']['Title']",
#   "$['TableOfContents']['Chapter 3']['Sections']['3.1']",
#   "$['TableOfContents']['Chapter 3']['Sections']['3.1.1']",
#   "$['TableOfContents']['Chapter 3']['Sections']['3.1.2']",
#   "$['TableOfContents']['Chapter 3']['Sections']['3.2']",
#   "$['TableOfContents']['Chapter 3']['Sections']['3.2.1']",
#   "$['TableOfContents']['Chapter 3']['Sections']['3.2.2']",
#   "$['TableOfContents']['Chapter 3']['Sections']['3.2.3']",
#   "$['TableOfContents']['Chapter 3']['Sections']['3.2.3.1']",
#   "$['TableOfContents']['Chapter 3']['Sections']['3.2.3.2']",
#   "$['TableOfContents']['Chapter 3']['Sections']['3.2.3.3']",
#   "$['TableOfContents']['Chapter 3']['Sections']['3.2.3.4']",
#   "$['TableOfContents']['Chapter 3']['Sections']['3.2.4']",
#   "$['TableOfContents']['Chapter 3']['Sections']['3.2.5']",
#   "$['TableOfContents']['Chapter 3']['Sections']['3.3']",
#   "$['TableOfContents']['Chapter 4']['Sections']",
#   "$['TableOfContents']['Chapter 4']['Title']",
#   "$['TableOfContents']['Chapter 4']['Sections']['4.1']",
#   "$['TableOfContents']['Chapter 4']['Sections']['4.1.1']",
#   "$['TableOfContents']['Chapter 4']['Sections']['4.1.2']",
#   "$['TableOfContents']['Chapter 4']['Sections']['4.2']",
#   "$['TableOfContents']['Chapter 4']['Sections']['4.2.1']",
#   "$['TableOfContents']['Chapter 4']['Sections']['4.3']",
#   "$['TableOfContents']['Chapter 4']['Sections']['4.3.1']",
#   "$['TableOfContents']['Chapter 4']['Sections']['4.3.2']",
#   "$['TableOfContents']['Chapter 4']['Sections']['4.3.3']",
#   "$['TableOfContents']['Chapter 5']['Sections']",
#   "$['TableOfContents']['Chapter 5']['Title']",
#   "$['TableOfContents']['Chapter 5']['Sections']['5.1']",
#   "$['TableOfContents']['Chapter 5']['Sections']['5.1.1']",
#   "$['TableOfContents']['Chapter 5']['Sections']['5.1.2']",
#   "$['TableOfContents']['Chapter 5']['Sections']['5.2']",
#   "$['TableOfContents']['Chapter 5']['Sections']['5.2.1']",
#   "$['TableOfContents']['Chapter 5']['Sections']['5.2.2']",
#   "$['TableOfContents']['Chapter 5']['Sections']['5.2.3']",
#   "$['TableOfContents']['Chapter 5']['Sections']['5.2.4']",
#   "$['TableOfContents']['Chapter 5']['Sections']['5.3']",
#   "$['TableOfContents']['Chapter 5']['Sections']['5.4']",
#   "$['TableOfContents']['Chapter 6']['Sections']",
#   "$['TableOfContents']['Chapter 6']['Title']",
#   "$['TableOfContents']['Chapter 6']['Sections']['6.1']",
#   "$['TableOfContents']['Chapter 6']['Sections']['6.1.1']",
#   "$['TableOfContents']['Chapter 6']['Sections']['6.1.2']",
#   "$['TableOfContents']['Chapter 6']['Sections']['6.2']",
#   "$['TableOfContents']['Chapter 6']['Sections']['6.2.1']",
#   "$['TableOfContents']['Chapter 6']['Sections']['6.2.2']",
#   "$['TableOfContents']['Chapter 6']['Sections']['6.2.3']",
#   "$['TableOfContents']['Chapter 6']['Sections']['6.3']",
#   "$['TableOfContents']['Chapter 6']['Sections']['6.3.n']",
#   "$['TableOfContents']['Chapter 6']['Sections']['6.3.n.1']",
#   "$['TableOfContents']['Chapter 6']['Sections']['6.3.n.2']",
#   "$['TableOfContents']['Chapter 6']['Sections']['6.3.n.3']",
#   "$['TableOfContents']['Chapter 6']['Sections']['6.3.n.4']",
#   "$['TableOfContents']['Chapter 7']['Sections']",
#   "$['TableOfContents']['Chapter 7']['Title']",
#   "$['TableOfContents']['Chapter 7']['Sections']['7.1']",
#   "$['TableOfContents']['Chapter 7']['Sections']['7.2']",
#   "$['TableOfContents']['Chapter 7']['Sections']['7.3']"
# ]
# Access individual elements
chapter_1 = table_of_content_data['TableOfContents']['Chapter 1']
print(chapter_1)
chapter_2 = table_of_content_data['TableOfContents']['Chapter 2']
print(chapter_2)
chapter_3 = table_of_content_data['TableOfContents']['Chapter 3']
print(chapter_3)
chapter_4 = table_of_content_data['TableOfContents']['Chapter 4']
print(chapter_4)
chapter_5 = table_of_content_data['TableOfContents']['Chapter 5']
print(chapter_5)
chapter_6 = table_of_content_data['TableOfContents']['Chapter 6']
print(chapter_6)
chapter_7 = table_of_content_data[table_of_content_data['TableOfContents']['Chapter 7']]
print(chapter_7)

appendix_attachments = table_of_content_data['TableOfContents']['Appendices']['Attachments']
print(appendix_attachments)

glossary = table_of_content_data['TableOfContents']['Glossary']
print(glossary)

references = table_of_content_data['TableOfContents']['References']
print(references)
