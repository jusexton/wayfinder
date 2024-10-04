# Wayfinder

Json path property tracing software.

## Usage

Suppose we have some json:
```json
{
    "name": "John Doe",
    "phones": [
        {
            "type": "home",
            "number": "555-1234"
        },
        {
            "type": "work",
            "number": "555-5678"
        }
    ]
}
```

Provide the raw json or via file and specify the target properties:
```sh
wayfinder --json-file my-file.json --target number --target name

wayfinder --raw-json "{\"number\":\"test\"}" --target number --target name
```

The resulting output are the full paths of all provided targets:
```sh
Paths to 'number':
  $root.phones[0]
  $root.phones[1]

Paths to 'name':
  $root
```

This is useful for finding the full path to a property for large and deeply nested models.

