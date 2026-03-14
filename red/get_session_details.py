import sqlite3

cookies_db = "C:/Users/Computer/AppData/Roaming/Opera Software/Opera GX Stable/Default/Network/Cookies"

try:
    conn = sqlite3.connect(cookies_db)
    cursor = conn.cursor()
    
    # Get the logged_in cookie value
    cursor.execute("""
        SELECT name, value, expires_utc
        FROM cookies 
        WHERE host_key LIKE '%github%' AND name = 'logged_in'
    """)
    
    result = cursor.fetchone()
    
    if result:
        name, value, expires = result
        print(f"\n✓ GitHub logged_in cookie found!")
        print(f"Value: {value}")
        print(f"Expires: {expires}")
        
        if value == "yes":
            print("\n*** YOU ARE LOGGED IN TO GITHUB! ***")
            print("\nNext steps:")
            print("1. Open Opera GX browser")
            print("2. Go to https://github.com")
            print("3. You should be logged in!")
            print("4. Go to Settings → Password and authentication → Two-factor authentication")
            print("5. Disable 2FA or generate new recovery codes")
        else:
            print(f"\nSession status unclear (value: {value})")
    else:
        print("\n✗ No logged_in cookie found")
    
    conn.close()
    
except Exception as e:
    print(f"Error: {e}")
