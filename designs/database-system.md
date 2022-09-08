Author: Mycodee
----

# Database System
```
  The Database System that powers the bot permanent storage, which itself uses CouchDB and file which is for configuration and secrets only.
```

## Elements
Warn
```
  member: DiscordId
  staff: DiscordId
  reason: String
  valid_until: Date
  created_at: Date
```
----
Report
```
  member: DiscordId
  message: String 
  completed: Boolean
  created_at: Date
```
 