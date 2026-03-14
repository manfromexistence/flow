import sqlite3
from datetime import datetime

cookies_db = "C:/Users/Computer/AppData/Roaming/Opera Software/Opera GX Stable/Default/Network/Cookies"

try:
    conn = sqlite3.connect(cookies_db)
    cursor = conn.cursor()
    
    # Get all GitHub cookies
    cursor.execute("""
        SELECT host_key, name, value, expires_utc, creation_utc, last_access_utc
        FROM cookies 
        WHERE host_key LIKE '%github%'
        ORDER BY last_access_utc DESC
    """)
    
    results = cursor.fetchall()
    
    print("\n" + "=" * 80)
    print("ALL GITHUB COOKIES WITH DETAILS")
    print("=" * 80 + "\n")
    
    session_found = False
    
    for row in results:
        host, name, value, expires, created, last_access = row
        
        print(f"Cookie: {name}")
        print(f"Host: {host}")
        print(f"Has Value: {'YES' if value else 'NO'}")
        print(f"Value Length: {len(value) if value else 0} chars")
        
        if name in ['user_session', 'dotcom_user', '__Host-user_session_same_site']:
            print(f"*** SESSION COOKIE ***")
            if value:
                print(f"Value: {value}")
                session_found = True
        
        print(f"Created: {created}")
        print(f"Last Access: {last_access}")
        print(f"Expires: {expires}")
        print("-" * 80)
    
    if session_found:
        print("\n✓✓✓ ACTIVE SESSION FOUND! ✓✓✓")
        print("\nYour GitHub session is still active in Opera GX!")
        print("\nACTION REQUIRED:")
        print("1. Open Opera GX browser NOW")
        print("2. Navigate to: https://github.com/settings/security")
        print("3. You should be logged in automatically")
        print("4. Scroll to 'Two-factor authentication'")
        print("5. Click 'Edit' or 'Disable' to manage 2FA")
        print("6. Generate NEW recovery codes and save them!")
    else:
        print("\n✗ No active session cookies found")
        print("The session may have expired or been cleared.")
    
    conn.close()
    
except Exception as e:
    print(f"Error: {e}")
