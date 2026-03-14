import sqlite3
import sys

# Path to Opera cookies database
cookies_db = "F:/New Appdata/AppData/Roaming/Opera Software/Opera GX Stable/Default/Network/Cookies"

try:
    # Connect to the database
    conn = sqlite3.connect(cookies_db)
    cursor = conn.cursor()
    
    # Query for GitHub cookies
    cursor.execute("""
        SELECT host_key, name, value, path, expires_utc, is_secure, is_httponly 
        FROM cookies 
        WHERE host_key LIKE '%github%'
        ORDER BY host_key, name
    """)
    
    results = cursor.fetchall()
    
    if results:
        print(f"\n✓ Found {len(results)} GitHub cookies!\n")
        print("=" * 80)
        for row in results:
            host, name, value, path, expires, secure, httponly = row
            print(f"Host: {host}")
            print(f"Name: {name}")
            print(f"Value: {value[:50]}..." if len(value) > 50 else f"Value: {value}")
            print(f"Path: {path}")
            print(f"Expires: {expires}")
            print(f"Secure: {secure}, HttpOnly: {httponly}")
            print("-" * 80)
    else:
        print("\n✗ No GitHub cookies found in the database.")
    
    conn.close()
    
except Exception as e:
    print(f"Error: {e}")
    sys.exit(1)
