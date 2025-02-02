

# db
  - get_record(short_url)
  - insert(shorturl, originalurl, expirrationdate, max_uses)
  - delete(shorturl)

  - get_last_id(range);
    - increment and return
# api
  - get
    - check uses
    - original link
  - post
    - body {expiration_date, max_uses, original_link}
    - get_last_id
    - convert
    - insert

# server
  - start sever
  - maintain threads
  - parse requests
  - response
