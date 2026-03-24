import os
import re

for root, dirs, files in os.walk('listings/ch12-an-io-project'):
    if 'target' in root:
        continue
    for file in files:
        if file.endswith('.rs') or file.endswith('.txt'):
            path = os.path.join(root, file)
            with open(path, 'r', encoding='utf-8') as f:
                content = f.read()
            
            # Revert bad `env::argumanlar()` to `env::args()`
            new_content = content.replace('env::argumanlar()', 'env::args()')
            
            # Add missing replacements:
            # config -> yapilandirma
            new_content = re.sub(r'\bconfig\b', 'yapilandirma', new_content)
            # err -> hata
            new_content = re.sub(r'\|err\|', '|hata|', new_content)
            new_content = re.sub(r'\{err\}', '{hata}', new_content)
            
            if new_content != content:
                with open(path, 'w', encoding='utf-8') as f:
                    f.write(new_content)
                print(f"Updated {path}")
