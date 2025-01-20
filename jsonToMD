def generate_markdown_report(data, title, indent="  "):
    """Generates a formatted report in Markdown based on nested JSON data."""

    def format_dict(d, level_indent):
        md = ""
        for key, value in d.items():
            if isinstance(value, dict):
                md += f"{level_indent}- **{key.replace('_', ' ').title()}**:\n"
                md += format_dict(value, level_indent + indent)
            elif isinstance(value, list):
                md += f"{level_indent}- **{key.replace('_', ' ').title()}**:\n"
                md += format_list(value, level_indent + indent)
            else:
                md += f"{level_indent}- **{key.replace('_', ' ').title()}**: {value}\n"
        return md

    def format_list(lst, level_indent):
        md = ""
        for item in lst:
            if isinstance(item, dict):
                md += format_dict(item, level_indent + "\n- ")
            else:
                md += f"{level_indent}- {item}\n"
        return md

    md_report = f"# {title}\n\n"
    md_report += format_dict(data, indent)
    return md_report


# Example usage:
# markdown_report = generate_markdown_report(data, "Project Data Report")
# Markdown(markdown_report)
