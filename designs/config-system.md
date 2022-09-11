Author: Mycodee
----

# Config System
```
  This the design for the Bot Configuration System which itself is the core of this discord bot
``` 

## Elements
Main Configuration
```
  discord: Discord Configuration
```

---

Discord Configuration
```
  token: String
  app_id: String
```

Database Configuration 
```
  host: String
  port: Optional u32 DEFAULT 5432
  user: Optional String DEFAULT "postgres"
  password Optional String DEFAULT ""
  database Optional String DEFAULT user
```