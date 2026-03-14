import os
import re
import sys

# Pattern for recovery codes (5 chars - 5 chars format)
recovery_pattern = r'[a-f0-9]{5}-[a-f0-9]{5}'

# Search all drives
drives = ['C:/', 'D:/', 'E:/', 'F:/', 'G:/', 'H:/']

print("=" * 80)
print("SEARCHING FOR ALL FILES WITH GITHUB RECOVERY CODES")
print("=" * 80)

found_files = []

for drive in drives:
    if not os.path.exists(drive):
        continue
    
    print(f"\nSearching drive: {drive}")
    
    try:
        for root, dirs, files in os.walk(drive):
            # Skip system directories
            dirs[:] = [d for d in dirs if d not in [
                '$RECYCLE.BIN', 'System Volume Information', 
                'Windows', 'WinSxS', 'Recovery',
                'ProgramData', 'Program Files', 'Program Files (x86)',
                'node_modules', '.git', 'Cache', 'cache', '.cache'
            ]]
            
            for file in files:
                # Look for text and markdown files
                if file.endswith('.txt') or file.endswith('.md'):
                    file_path = os.path.join(root, file)
                    
                    try:
                        with open(file_path, 'r', encoding='utf-8', errors='ignore') as f:
                            content = f.read()
                            
                            # Find recovery code patterns
                            matches = re.findall(recovery_pattern, content)
                            
                            # If file has 10+ codes, it's likely a recovery file
                            if len(matches) >= 10:
                                # Check for github-related keywords
                                has_github = 'github' in content.lower()
                                has_recovery = 'recovery' in content.lower()
                                has_najmus = 'najmus' in content.lower()
                                has_manfrom = 'manfrom' in content.lower()
                                
                                found_files.append({
                                    'path': file_path,
                                    'codes': len(matches),
                                    'github': has_github,
                                    'recovery': has_recovery,
                                    'najmus': has_najmus,
                                    'manfrom': has_manfrom,
                                    'first_codes': matches[:3]
                                })
                                
                                print(f"\n  ✓ FOUND: {file_path}")
                                print(f"    Codes: {len(matches)}")
                                if has_github or has_recovery or has_najmus or has_manfrom:
                                    print(f"    Keywords: github={has_github}, recovery={has_recovery}, najmus={has_najmus}, manfrom={has_manfrom}")
                    except:
                        pass
            
            # Progress
            sys.stdout.write('.')
            sys.stdout.flush()
            
            # Limit depth
            if root.count(os.sep) - drive.count(os.sep) > 5:
                dirs[:] = []
                
    except Exception as e:
        pass

print("\n\n" + "=" * 80)
if found_files:
    print(f"✓✓✓ FOUND {len(found_files)} FILE(S) WITH RECOVERY CODES! ✓✓✓")
    print("=" * 80)
    for f in found_files:
        print(f"\nFile: {f['path']}")
        print(f"Codes: {f['codes']}")
        print(f"First 3: {', '.join(f['first_codes'])}")
        if f['github'] or f['recovery'] or f['najmus'] or f['manfrom']:
            print(f"*** Contains relevant keywords! ***")
        print("-" * 80)
else:
    print("✗ No recovery code files found")
print("=" * 80)
