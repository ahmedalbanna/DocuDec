# DocuDec

Document generation pipeline: Python service that turns structured inputs
and LLM prompts into generated documents and reports.

## Components

- `main.py` / `prompts.py` — generation entry points and prompt templates
- `api/` — HTTP endpoints for login, generation, and report download
- notebooks — interactive exploration of the generation flow

## Notes

- Generation runs through LLM providers selected via the prompt layer
- Report download endpoint serves generated artifacts
