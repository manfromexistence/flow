import os
import re
from pathlib import Path

# GitHub token patterns
patterns = [
    (r'ghp_[a-zA-Z0-9]{36}', 'Personal Access Token (classic)'),
    (r'github_pat_[a-zA-Z0-9]{22}_[a-zA-Z0-9]{59}', 'Fine-grained Personal Access Token'),
    (r'gho_[a-zA-Z0-9]{36}', 'OAuth Access Token'),
    (r'ghu_[a-zA-Z0-9]{36}', 'User-to-Server Token'),
    (r'ghs_[a-zA-Z0-9]{36}', 'Server-to-Server Token'),
    (r'ghr_[a-zA-Z0-9]{36}', 'Refresh Token'),
]

search_paths = [
    'F:/Desktop',
    'F:/Documents',
    'F:/Dev',
    'F:/New Appdata/AppData/Roaming',
    os.path.expanduser('~/Documents'),
    os.path.expanduser('~/Desktop'),
    os.path.expanduser('~/.config'),
]

file_extensions = ['.env', '.txt', '.md', '.json', '.yaml', '.yml', '.config', '.sh', '.bat', '.ps1']

print("=" * 80)
print("SEARCHING FOR GITHUB TOKENS")
print("=" * 80)

found_tokens = []

for search_path in search_paths:
    if not os.path.exists(search_path):
        continue
    
    print(f"\nSearching: {search_path}")
    
    try:
        for root, dirs, files in os.walk(search_path):
            # Skip node_modules, .git, etc.
            dirs[:] = [d for d in dirs if d not in ['.git', 'node_modules', '__pycache__', '.venv', 'venv']]
            
            for file in files:
                if any(file.endswith(ext) or file.startswith('.env') for ext in file_extensions):
                    file_path = os.path.join(root, file)
                    
                    try:
                        with open(file_path, 'r', encoding='utf-8', errors='ignore') as f:
                            content = f.read()
                            
                            for pattern, token_type in patterns:
                                matches = re.findall(pattern, content)
                                if matches:
                                    for match in matches:
                                        found_tokens.append((file_path, token_type, match))
                                        print(f"  ✓ FOUND: {token_type}")
                                        print(f"    File: {file_path}")
                                        print(f"    Token: {match[:20]}...")
                    except:
                        pass
            
            # Limit depth to avoid too long search
            if root.count(os.sep) - search_path.count(os.sep) > 3:
                break
                
    except Exception as e:
        print(f"  Error: {e}")

print("\n" + "=" * 80)
if found_tokens:
    print(f"✓✓✓ FOUND {len(found_tokens)} GITHUB TOKEN(S)! ✓✓✓")
    print("=" * 80)
    for file_path, token_type, token in found_tokens:
        print(f"\nType: {token_type}")
        print(f"File: {file_path}")
        print(f"Token: {token}")
else:
    print("✗ No GitHub tokens found")
print("=" * 80)
