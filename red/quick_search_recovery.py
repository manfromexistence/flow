import os
import re

# Specific recovery codes to search for
target_codes = [
    'b22df-4f229', 'b43af-36d53', '60226-24ba1', '281cf-5e254',
    'f95e1-d5621', 'b67e4-7204e', 'd5ed3-319de', 'b2f71-6b32d',
    '5420c-58935', 'da650-80830', 'b8527-c6a15', 'fa89c-b644b',
    '0fb43-e447d', '21fef-e021e', '5d565-c7253', '3c846-afb91'
]

# Priority search locations
priority_paths = [
    'F:/Desktop',
    'F:/Documents', 
    'F:/Downloads',
    'F:/Backups',
    'C:/Users/Computer/Desktop',
    'C:/Users/Computer/Documents',
    'C:/Users/Computer/Downloads',
]

print("=" * 80)
print("SEARCHING FOR YOUR SPECIFIC RECOVERY CODES")
print("=" * 80)

found = False

for search_path in priority_paths:
    if not os.path.exists(search_path):
        continue
    
    print(f"\nSearching: {search_path}")
    
    try:
        for root, dirs, files in os.walk(search_path):
            # Skip large dirs
            dirs[:] = [d for d in dirs if d not in ['node_modules', '.git', 'Cache', 'cache']]
            
            for file in files:
                if file.endswith('.txt') or file.endswith('.md'):
                    file_path = os.path.join(root, file)
                    
                    try:
                        with open(file_path, 'r', encoding='utf-8', errors='ignore') as f:
                            content = f.read()
                            
                            # Check if any of the target codes are in the file
                            matches = [code for code in target_codes if code in content]
                            
                            if matches:
                                print(f"\n  ✓✓✓ FOUND RECOVERY CODES! ✓✓✓")
                                print(f"  File: {file_path}")
                                print(f"  Matching codes: {len(matches)}")
                                print(f"  Codes: {', '.join(matches[:5])}")
                                found = True
                                
                                # Show file content
                                print(f"\n  FILE CONTENT:")
                                print("  " + "-" * 76)
                                for line in content.split('\n')[:50]:
                                    print(f"  {line}")
                                print("  " + "-" * 76)
                    except:
                        pass
            
            # Limit depth to 3 levels
            if root.count(os.sep) - search_path.count(os.sep) >= 3:
                dirs[:] = []
                
    except Exception as e:
        pass

if not found:
    print("\n✗ Recovery codes not found in priority locations")
    print("\nTry checking:")
    print("  - Email (search for 'GitHub recovery')")
    print("  - Screenshots folder")
    print("  - Cloud storage (Google Drive, Dropbox, OneDrive)")
    print("  - Password manager")
    print("  - Phone notes/photos")

print("\n" + "=" * 80)
