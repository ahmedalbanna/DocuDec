from google.api_core import retry, exceptions


@retry.Retry(
    predicate=retry.if_transient_error,
    initial=2.0,
    maximum=64.0,
    multiplier=2.0,
    timeout=600,
)
def generate_content_first_fail(model, prompt):
    if not hasattr(generate_content_first_fail, "call_counter"):
        generate_content_first_fail.call_counter = 0

    generate_content_first_fail.call_counter += 1

    try:
        if generate_content_first_fail.call_counter == 1:
            raise exceptions.ServiceUnavailable("Service Unavailable")

        response = model.generate_content(prompt)
        return response.text
    except exceptions.ServiceUnavailable as e:
        print(f"Error: {e}")
        raise
