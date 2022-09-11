Author: Mycodee
----

# Database System
```
  The Database System that powers the bot permanent storage, which itself uses SQLX and SQLB for some Queries and a file which is only for configuration and secrets only.
```

## Elements

Warn
```
  id: String
  member: DiscordId
  staff: DiscordId
  reason: String
  valid_until: Date
  created_at: Date
```
----

Report
```
  id: String
  member: DiscordId
  message: String 
  completed: Boolean
  created_at: Date
```
