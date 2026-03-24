import os
import glob

replacements = {
    r'\bsearch\b': 'ara',
    r'\bsearch_case_insensitive\b': 'buyuk_kucuk_harf_duyarsiz_ara',
    r'\bConfig\b': 'Yapilandirma',
    r'\bbuild\b': 'olustur',
    r'\bquery\b': 'sorgu',
    r'\bfile_path\b': 'dosya_yolu',
    r'\bignore_case\b': 'buyuk_kucuk_harf_yoksay',
    r'\bargs\b': 'argumanlar',
    r'\bcontents\b': 'icerik',
    r'\bresults\b': 'sonuclar',
    r'\brun\b': 'calistir',
    r'\bparse_config\b': 'yapilandirma_ayristir',
    '"not enough arguments"': '"yeterli argüman yok"',
    '"Problem parsing arguments: {err}"': '"Argümanları ayrıştırırken problem oluştu: {hata}"',
    '"Application error: {e}"': '"Uygulama hatası: {e}"',
    '"Searching for {}"': '"Aranan: {}"',
    '"In file {}"': '"Dosya: {}"',
    '"Searching for {query}"': '"Aranan: {sorgu}"',
    '"In file {file_path}"': '"Dosya: {dosya_yolu}"',
    r'"With text:\\n\{\}"': r'"Metin içeriği:\\n{}"',
    r'"With text:\\n\{contents\}"': r'"Metin içeriği:\\n{icerik}"',
    '"Should have been able to read the file"': '"Dosya okunamadı"',
    r'\bcase_sensitive\b': 'buyuk_kucuk_harf_duyarli',
    r'\bcase_insensitive\b': 'buyuk_kucuk_harf_duyarsiz',
    r'\bone_result\b': 'tek_sonuc',
    r'\|err\|': '|hata|',
    r'\{err\}': '{hata}',
    r'\bline\b': 'satir',
}

import re

for root, dirs, files in os.walk('listings/ch12-an-io-project'):
    if 'target' in root:
        continue
    for file in files:
        if file.endswith('.rs') or file.endswith('.txt'):
            path = os.path.join(root, file)
            with open(path, 'r', encoding='utf-8') as f:
                content = f.read()
            
            new_content = content
            for k, v in replacements.items():
                if k.startswith(r'\b') or k.startswith('^'):
                    new_content = re.sub(k, v, new_content)
                else:
                    new_content = new_content.replace(k.replace('\\n', '\n'), v.replace('\\n', '\n'))
            
            if new_content != content:
                with open(path, 'w', encoding='utf-8') as f:
                    f.write(new_content)
                print(f"Updated {path}")
