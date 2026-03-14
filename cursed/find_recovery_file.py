import os
import sys

target_filename = "github-recovery-codes.txt"

# Search all drives
drives = ['C:/', 'D:/', 'E:/', 'F:/', 'G:/', 'H:/']

print("=" * 80)
print(f"SEARCHING ENTIRE SYSTEM FOR: {target_filename}")
print("=" * 80)

found_files = []

for drive in drives:
    if not os.path.exists(drive):
        continue
    
    print(f"\nSearching drive: {drive}")
    
    try:
        for root, dirs, files in os.walk(drive):
            # Skip system directories that would cause permission errors
            dirs[:] = [d for d in dirs if d not in [
                '$RECYCLE.BIN', 'System Volume Information', 
                'Windows', 'WinSxS', 'Recovery',
                'ProgramData', 'Program Files', 'Program Files (x86)',
                'node_modules', '.git'
            ]]
            
            for file in files:
                if file.lower() == target_filename.lower():
                    file_path = os.path.join(root, file)
                    found_files.append(file_path)
                    print(f"\n  ✓✓✓ FOUND: {file_path}")
                    
                    # Try to read and display content
                    try:
                        with open(file_path, 'r', encoding='utf-8', errors='ignore') as f:
                            content = f.read()
                            print(f"\n  FILE CONTENT:")
                            print("  " + "=" * 76)
                            print(content)
                            print("  " + "=" * 76)
                    except Exception as e:
                        print(f"  Could not read file: {e}")
            
            # Progress indicator every 1000 directories
            if len(os.listdir(root)) > 0:
                sys.stdout.write('.')
                sys.stdout.flush()
                
    except Exception as e:
        pass

print("\n\n" + "=" * 80)
if found_files:
    print(f"✓✓✓ FOUND {len(found_files)} FILE(S)! ✓✓✓")
    for f in found_files:
        print(f"  - {f}")
else:
    print("✗ File not found on any drive")
print("=" * 80)
