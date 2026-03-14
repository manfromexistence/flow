import os
import sys

target_filename = "GITHUB.md"

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
            # Skip system directories
            dirs[:] = [d for d in dirs if d not in [
                '$RECYCLE.BIN', 'System Volume Information', 
                'Windows', 'WinSxS', 'Recovery',
                'ProgramData', 'Program Files', 'Program Files (x86)',
                'node_modules', '.git'
            ]]
            
            for file in files:
                if file.upper() == target_filename.upper():
                    file_path = os.path.join(root, file)
                    found_files.append(file_path)
                    print(f"\n  ✓ FOUND: {file_path}")
            
            # Progress indicator
            sys.stdout.write('.')
            sys.stdout.flush()
                
    except Exception as e:
        pass

print("\n\n" + "=" * 80)
if found_files:
    print(f"✓✓✓ FOUND {len(found_files)} FILE(S) NAMED '{target_filename}'! ✓✓✓")
    print("=" * 80)
    for f in found_files:
        print(f"  {f}")
else:
    print(f"✗ No files named '{target_filename}' found on any drive")
print("=" * 80)
