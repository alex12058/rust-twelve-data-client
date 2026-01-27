#!/usr/bin/env python3
"""
Fetch the Twelve Data OpenAPI spec and apply local transforms:
- Drop the queryParameter security scheme entry
- Ensure the ApiError component exists
- Wrap 200 application/json responses in oneOf with ApiError
- Add text/csv responses to operations that accept a format query param
- Rename *_200_response[_meta] schemas to *Data[*Meta]
"""

import copy
import json
import re
from pathlib import Path
from urllib.request import Request, urlopen

SOURCE_URL = "https://api.twelvedata.com/doc/swagger/openapi.json"
OUTPUT_PATH = Path(__file__).resolve().parent / "openapi.json"
API_ERROR_REF = "#/components/schemas/ApiError"


def fetch_openapi(url: str = SOURCE_URL) -> dict:
    req = Request(url, headers={"User-Agent": "openapi-updater"})
    with urlopen(req, timeout=60) as resp:
        return json.load(resp)


def remove_query_parameter_security(openapi: dict) -> bool:
    components = openapi.setdefault("components", {})
    security_schemes = components.setdefault("securitySchemes", {})
    removed = security_schemes.pop("queryParameter", None) is not None

    if "security" in openapi:
        openapi["security"] = [entry for entry in openapi["security"] if "queryParameter" not in entry]

    return removed


def ensure_api_error_component(openapi: dict) -> bool:
    schemas = openapi.setdefault("components", {}).setdefault("schemas", {})
    if "ApiError" in schemas:
        return False

    schemas["ApiError"] = {
        "type": "object",
        "properties": {
            "code": {
                "type": "integer",
                "format": "int64",
                "description": "HTTP status code",
                "examples": [400],
                "x-go-name": "Code",
                "x-order": 10,
            },
            "message": {
                "type": "string",
                "description": "Error message",
                "examples": [
                    "Invalid **interval** provided: 0.99min. Supported intervals: 1min, 5min, 15min, 30min, 45min, 1h, 2h, 4h, 8h, 1day, 1week, 1month",
                ],
                "x-go-name": "Message",
                "x-order": 20,
            },
            "status": {
                "type": "string",
                "description": "Response status",
                "examples": ["error"],
                "x-go-name": "Status",
                "x-order": 30,
            },
        },
        "required": ["code", "message", "status"],
    }
    return True


def _iter_parameters(path_item: dict, operation: dict):
    for param in path_item.get("parameters", []) + operation.get("parameters", []):
        yield param


def _resolve_param_ref(param: dict, openapi: dict) -> dict:
    ref = param.get("$ref")
    if not ref or not ref.startswith("#/components/parameters/"):
        return {}
    name = ref.split("/")[-1]
    return openapi.get("components", {}).get("parameters", {}).get(name, {})


def has_format_parameter(path_item: dict, operation: dict, openapi: dict) -> bool:
    for param in _iter_parameters(path_item, operation):
        if not isinstance(param, dict):
            continue
        if param.get("name") == "format" and param.get("in") == "query":
            return True
        if "$ref" in param:
            resolved = _resolve_param_ref(param, openapi)
            if resolved.get("name") == "format" and resolved.get("in") == "query":
                return True
            if "format" in param["$ref"].lower():
                return True
    return False


def add_text_csv_responses(openapi: dict) -> int:
    modified = 0
    for _path, path_item in openapi.get("paths", {}).items():
        if not isinstance(path_item, dict):
            continue
        for method, operation in path_item.items():
            if method in {"parameters", "servers", "summary", "description"} or method.startswith("x-"):
                continue
            if not isinstance(operation, dict):
                continue
            if not has_format_parameter(path_item, operation, openapi):
                continue

            responses = operation.get("responses", {})
            response_200 = responses.get("200")
            if not isinstance(response_200, dict):
                continue
            content = response_200.setdefault("content", {})
            if "text/csv" not in content:
                content["text/csv"] = {"schema": {"type": "string"}}
                modified += 1
    return modified


def add_api_error_to_responses(openapi: dict) -> int:
    modified = 0
    for _path, path_item in openapi.get("paths", {}).items():
        if not isinstance(path_item, dict):
            continue
        for method, operation in path_item.items():
            if method in {"parameters", "servers", "summary", "description"} or method.startswith("x-"):
                continue
            if not isinstance(operation, dict):
                continue

            responses = operation.get("responses", {})
            response_200 = responses.get("200")
            if not isinstance(response_200, dict):
                continue

            content = response_200.get("content", {})
            json_content = content.get("application/json")
            if not isinstance(json_content, dict):
                continue

            schema = json_content.get("schema")
            if not isinstance(schema, dict):
                continue

            if schema.get("$ref") == API_ERROR_REF:
                continue

            if "oneOf" in schema:
                one_of = schema.get("oneOf") or []
                if any(item.get("$ref") == API_ERROR_REF for item in one_of if isinstance(item, dict)):
                    continue
                json_content["schema"]["oneOf"] = [{"$ref": API_ERROR_REF}] + one_of
            else:
                json_content["schema"] = {
                    "oneOf": [
                        {"$ref": API_ERROR_REF},
                        copy.deepcopy(schema),
                    ]
                }
            modified += 1
    return modified


def rename_response_components(openapi: dict) -> dict:
    schemas = openapi.setdefault("components", {}).setdefault("schemas", {})
    rename_map: dict[str, str] = {}

    # Single pattern: optionally match "Get", capture base, match _200_response, capture suffix
    pattern = re.compile(r"(Get)?(.+)_200_response(.*)$")

    original_order = list(schemas.keys())

    for name in original_order:
        new_name = None
        if (m := pattern.match(name)):
            base = m.group(2)
            suffix = m.group(3)
            new_name = f"{base}{suffix}"

        if new_name and new_name != name:
            if new_name in schemas:
                raise ValueError(f"Cannot rename {name} to {new_name}: target already exists")
            rename_map[name] = new_name

    if rename_map:
        # Rebuild schemas dict to preserve original order while applying new names
        new_schemas: dict[str, dict] = {}
        for name in original_order:
            target = rename_map.get(name, name)
            if target in new_schemas:
                raise ValueError(f"Duplicate target schema name {target} while rebuilding")
            new_schemas[target] = schemas[name]

        openapi["components"]["schemas"] = new_schemas
        update_refs(openapi, rename_map)

    return rename_map

def make_object_values_required(openapi: dict, keys: list[str]):
    num_required_values = 0
    schemas = openapi.setdefault("components", {}).setdefault("schemas", {})

    for key in keys:
        schema_def = schemas.get(key)
        if not isinstance(schema_def, dict):
            raise ValueError(f"Did not find a component named {key}")
        
        schema_type = schema_def.get("type")
        if schema_type != "object":
            raise ValueError(f'Expected componnet {key} to be type "object" but found "{schema_type}"')
        
        properties = schema_def.get("properties")
        if isinstance(properties, dict):
            dict_keys = list(properties.keys())
            schema_def["required"] = dict_keys
            num_required_values += len(dict_keys)
    
    return num_required_values

def update_refs(node, rename_map: dict):
    if isinstance(node, dict):
        for key, value in node.items():
            if key == "$ref" and isinstance(value, str):
                if value.startswith("#/components/schemas/"):
                    name = value.split("/")[-1]
                    if name in rename_map:
                        node[key] = f"#/components/schemas/{rename_map[name]}"
            else:
                update_refs(value, rename_map)
    elif isinstance(node, list):
        for item in node:
            update_refs(item, rename_map)


def write_openapi(openapi: dict, path: Path = OUTPUT_PATH):
    path.write_text(json.dumps(openapi, indent=2))


def main():
    openapi = fetch_openapi()

    removed_security = remove_query_parameter_security(openapi)
    created_api_error = ensure_api_error_component(openapi)
    api_error_updates = add_api_error_to_responses(openapi)
    csv_updates = add_text_csv_responses(openapi)
    rename_map = rename_response_components(openapi)
    num_required_vals = make_object_values_required(
        openapi,
        # Add additional object component schema names here as needed to make all of their values
        # required. This should only be for schemas where all values are guaranteed to be defined.
        ["TimeSeriesItem"]
    )

    write_openapi(openapi)

    print("OpenAPI updated:")
    print(f"- queryParameter security removed: {removed_security}")
    print(f"- ApiError component created: {created_api_error}")
    print(f"- 200 responses wrapped with ApiError: {api_error_updates}")
    print(f"- text/csv responses added: {csv_updates}")
    print(f"- schemas renamed: {len(rename_map)}")
    print(f"- object values updated to be required: {num_required_vals}")


if __name__ == "__main__":
    main()
