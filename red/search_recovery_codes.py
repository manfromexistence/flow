import os
import re
from pathlib import Path

# Pattern for recovery codes (5 chars - 5 chars format)
recovery_pattern = r'[a-f0-9]{5}-[a-f0-9]{5}'

search_paths = [
    'F:/',
    'C:/Users/Computer',
]

print("=" * 80)
print("SEARCHING FOR GITHUB RECOVERY CODES")
print("Pattern: xxxxx-xxxxx (hex format)")
print("=" * 80)

found_files = []

for search_path in search_paths:
    if not os.path.exists(search_path):
        continue
    
    print(f"\nSearching: {search_path}")
    
    try:
        for root, dirs, files in os.walk(search_path):
            # Skip system and large directories
            dirs[:] = [d for d in dirs if d not in [
                '.git', 'node_modules', '__pycache__', '.venv', 'venv',
                'Windows', 'Program Files', 'Program Files (x86)',
                '$RECYCLE.BIN', 'System Volume Information',
                'ProgramData', 'Cache', 'cache', '.cache'
            ]]
            
            for file in files:
                if file.endswith('.txt') or file.endswith('.md'):
                    file_path = os.path.join(root, file)
                    
                    try:
                        with open(file_path, 'r', encoding='utf-8', errors='ignore') as f:
                            content = f.read()
                            
                            # Find all recovery code patterns
                            matches = re.findall(recovery_pattern, content)
                            
                            if len(matches) >= 3:  # If file has 3+ codes, likely a recovery file
                                found_files.append((file_path, matches))
                                print(f"  ✓ FOUND: {file_path}")
                                print(f"    Codes found: {len(matches)}")
                                
                                # Check if it contains "github" or "recovery"
                                if 'github' in content.lower() or 'recovery' in content.lower():
                                    print(f"    *** Contains 'github' or 'recovery' keywords! ***")
                    except:
                        pass
            
            # Limit depth
            if root.count(os.sep) - search_path.count(os.sep) > 5:
                dirs[:] = []
                
    except Exception as e:
        print(f"  Error: {e}")

print("\n" + "=" * 80)
if found_files:
    print(f"✓✓✓ FOUND {len(found_files)} FILE(S) WITH RECOVERY CODE PATTERNS! ✓✓✓")
    print("=" * 80)
    for file_path, codes in found_files:
        print(f"\nFile: {file_path}")
        print(f"Number of codes: {len(codes)}")
        print(f"First few codes: {', '.join(codes[:5])}")
        print("-" * 80)
else:
    print("✗ No files with recovery code patterns found")
print("=" * 80)
