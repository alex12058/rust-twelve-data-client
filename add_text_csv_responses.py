#!/usr/bin/env python3
"""
Script to add text/csv response type to operations that support format parameter.
"""

import json

def has_format_parameter(operation):
    """Check if operation has a 'format' query parameter."""
    parameters = operation.get('parameters', [])
    for param in parameters:
        # Handle both inline and $ref parameters
        if isinstance(param, dict):
            if param.get('name') == 'format' and param.get('in') == 'query':
                return True
            # Check if it's a $ref to a format parameter
            if '$ref' in param and 'format' in param['$ref'].lower():
                return True
    return False

def add_text_csv_to_responses(openapi_data):
    """Add text/csv content type to responses of operations with format parameter."""
    modified_count = 0
    
    # Iterate through all paths and operations
    for path, path_item in openapi_data.get('paths', {}).items():
        for method, operation in path_item.items():
            if method.startswith('x-') or method in ['parameters', 'servers', 'summary', 'description']:
                continue
            
            # Check if operation has format parameter
            if not has_format_parameter(operation):
                continue
                
            responses = operation.get('responses', {})
            if '200' in responses:
                response = responses['200']
                content = response.get('content', {})
                
                # Add text/csv if not already present
                if 'text/csv' not in content:
                    content['text/csv'] = {
                        'schema': {
                            'type': 'string'
                        }
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
    
    # Add text/csv to responses
    print("\nAdding text/csv to responses with format parameter...")
    modified_count = add_text_csv_to_responses(openapi_data)
    
    # Write back to file
    print(f"\nWriting to {output_file}...")
    with open(output_file, 'w') as f:
        json.dump(openapi_data, f, indent=2)
    
    print(f"\n✓ Modified {modified_count} responses")
    print("✓ OpenAPI schema updated successfully")

if __name__ == '__main__':
    main()
