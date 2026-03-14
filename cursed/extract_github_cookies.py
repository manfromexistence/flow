import sqlite3
import sys
from datetime import datetime

# Path to Opera cookies database
cookies_db = "F:/New Appdata/AppData/Roaming/Opera Software/Opera GX Stable/Default/Network/Cookies"

try:
    # Connect to the database
    conn = sqlite3.connect(cookies_db)
    cursor = conn.cursor()
    
    # Query for GitHub cookies
    query = """
    SELECT host_key, name, value, path, expires_utc, is_secure, is_httponly, last_access_utc
    FROM cookies 
    WHERE host_key LIKE '%github%'
    ORDER BY h