import sqlite3
import os

browsers = [
    ("Opera GX", "F:/New Appdata/AppData/Roaming/Opera Software/Opera GX Stable/Default/Network/Cookies"),
    ("Chrome Default", "F:/New Appdata/AppData/Local/Google/Chrome/User Data/Default/Network/Cookies"),
    ("Chrome Profile 6", "F:/New Appdata/AppData/Local/Google/Chrome/User Data/Profile 6/Network/Cookies"),
]

print("=" * 80)
print("SEARCHING FOR GITHUB COOKIES IN ALL BROWSERS")
print("=" * 80)

for browser_name, cookies_db in browsers:
    print(f"\n[{browser_name}]")
    
    if not os.path.exists(cookies_db):
        print(f"  ✗ Database not found")
        continue
    
    try:
        conn = sqlite3.connect(cookies_db)
        cursor = conn.cursor()
        
        cursor.execute("""
            SELECT host_key, name, value, expires_utc 
            FROM cookies 
            WHERE host_key LIKE '%github%'
            ORDER BY expires_utc DESC
        """)
        
        results = cursor.fetchall()
        
        if results:
            print(f"  ✓ Found {len(results)} GitHub cookies!")
            for row in results[:5]:  # Show first 5
                host, name, value, expires = row
                print(f"    - {name} @ {host} (expires: {expires})")
                if name in ['user_session', 'dotcom_user', '__Host-user_session_same_site']:
                    print(f"      VALUE: {value[:100]}...")
        else:
            print(f"  ✗ No GitHub cookies found")
        
        conn.close()
        
    except Exception as e:
        print(f"  ✗ Error: {e}")

print("\n" + "=" * 80)
