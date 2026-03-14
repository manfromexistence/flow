import sqlite3

# Path to Opera history database
history_db = "F:/New Appdata/AppData/Roaming/Opera Software/Opera GX Stable/Default/History"

try:
    conn = sqlite3.connect(history_db)
    cursor = conn.cursor()
    
    # Query for GitHub URLs
    cursor.execute("""
        SELECT url, title, visit_count, last_visit_time 
        FROM urls 
        WHERE url LIKE '%github%'
        ORDER BY last_visit_time DESC
        LIMIT 30
    """)
    
    results = cursor.fetchall()
    
    if results:
        print(f"\n✓ Found {len(results)} GitHub URLs in history!\n")
        print("=" * 80)
        for row in results:
            url, title, visits, last_visit = row
            print(f"URL: {url}")
            print(f"Title: {title}")
            print(f"Visits: {visits}, Last: {last_visit}")
            print("-" * 80)
    else:
        print("\n✗ No GitHub URLs found in history.")
    
    conn.close()
    
except Exception as e:
    print(f"Error: {e}")
