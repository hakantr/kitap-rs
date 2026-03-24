import os
import re

to_remove = [
    # Add a few more missing from the markdown files
    "functionality",
    "method signatures",
    "metadata",
    "return value",
    "implementation",
    "performances",
    "process",
    "contents",
    "performance"
]
patterns = [r' \(' + re.escape(p) + r'\)' for p in to_remove]

for root, dirs, files in os.walk('src'):
    for file in files:
        if file.endswith('.md'):
            path = os.path.join(root, file)
            with open(path, 'r', encoding='utf-8') as f:
                content = f.read()
            
            new_content = content
            for p in patterns:
                new_content = re.sub(p, '', new_content)
            
            if new_content != content:
                with open(path, 'w', encoding='utf-8') as f:
                    f.write(new_content)
                print(f"Updated {path}")
