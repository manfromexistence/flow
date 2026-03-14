import sqlite3
import os

cookies_db = "C:/Users/Computer/AppData/Roaming/Opera Software/Opera GX Stable/Default/Network/Cookies"

print("=" * 80)
print("CHECKING CURRENT OPERA GX INSTALLATION")
print("=" * 80)

if not os.path.exists(cookies_db):
    print(f"\n✗ Cookies database not found at: {cookies_db}")
else:
    try:
        conn = sqlite3.connect(cookies_db)
        cursor = conn.cursor()
        
        cursor.execute("""
            SELECT host_key, name, value, path, expires_utc, is_secure
            FROM cookies 
            WHERE host_key LIKE '%github%'
            ORDER BY expires_utc DESC
        """)
        
        results = cursor.fetchall()
        
        if results:
            print(f"\n✓✓✓ FOUND {len(results)} GITHUB COOKIES! ✓✓✓\n")
            print("=" * 80)
            for row in results:
                host, name, value, path, expires, secure = row
                print(f"Host: {host}")
                print(f"Cookie Name: {name}")
                if name in ['user_session', 'dotcom_user', '__Host-user_session_same_site', 'logged_in']:
                    print(f"Value: {value[:80]}...")
                    print(f"*** THIS IS A SESSION COOKIE! ***")
                else:
                    print(f"Value: {value[:50]}...")
                print(f"Path: {path}, Expires: {expires}, Secure: {secure}")
                print("-" * 80)
        else:
            print(f"\n✗ No GitHub cookies found in current Opera installation")
        
        conn.close()
        
    except Exception as e:
        print(f"\n✗ Error: {e}")

print("\n" + "=" * 80)
