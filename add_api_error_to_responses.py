#!/usr/bin/env python3
"""
Script to add ApiError to all 200 responses in the OpenAPI schema.
This wraps existing response schemas in oneOf with ApiError as an alternative.
"""

import json
import sys

def add_api_error_to_responses(openapi_data):
    """Add ApiError as a oneOf option to all 200 responses."""
    modified_count = 0
    
    # Iterate through all paths and operations
    for path, path_item in openapi_data.get('paths', {}).items():
        for method, operation in path_item.items():
            if method.startswith('x-') or method in ['parameters', 'servers', 'summary', 'description']:
                continue
                
            responses = operation.get('responses', {})
            if '200' in responses:
                response = responses['200']
                content = response.get('content', {})
                
                # Check if response has application/json
                if 'application/json' in content:
                    json_content = content['application/json']
                    schema = json_content.get('schema', {})
                    
                    # Skip if already has oneOf with ApiError
                    if 'oneOf' in schema:
                        has_api_error = any(
                            ref.get('$ref') == '#/components/schemas/ApiError'
                            for ref in schema.get('oneOf', [])
                        )
                        if has_api_error:
                            continue
                    
                    # Wrap existing schema in oneOf with ApiError
                    original_schema = schema.copy()
                    json_content['schema'] = {
                        'oneOf': [
                            {'$ref': '#/components/schemas/ApiError'},
                            original_schema
                        ]
                    }
                    modified_count += 1
                    print(f"Modified: {method.upper()} {path}")
    
    return modified_count

def main():
    input_file = 'openapi.json'
    output_file = 'openapi.json'
    
    # Read the OpenAPI spec
    print(f"Reading {input_file}...")
    with open(input_file, 'r') as f:
        openapi_data = json.load(f)
    
    # Add ApiError to responses
    print("\nAdding ApiError to 200 responses...")
    modified_count = add_api_error_to_responses(openapi_data)
    
    # Write back to file
    print(f"\nWriting to {output_file}...")
    with open(output_file, 'w') as f:
        json.dump(openapi_data, f, indent=2)
    
    print(f"\n✓ Modified {modified_count} responses")
    print("✓ OpenAPI schema updated successfully")

if __name__ == '__main__':
    main()
