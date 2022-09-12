Author: Mycodee
---

# Plugin System 
```
  The Plugin System which should power the bot itself. 
  It consits of an Event Loop, it has Discord Events, Custom and Lifetime Events.
  
  On Startup the Start Level provided by the plugin determines the startup order of the plugins, 
  the order is from the lowest to the higest, so the lowest level starts first. 
  
  
  On Shutdown the Stop Level also provided by the plugin determines the stop order of the plugins,
  the order is like from the startup, the lowest stops last.
  
  The Plugin defines it's config and the methods for handling start, start and events 
```

## Event Types
Lifetime Events
```
  In this type events are located, which declare the lifetime cycle of the bot and its plugins. So and an Event fire if the bot or feature starts, shutsdown or errors.
```
----

Custom Events 
```
  In this type events are located, which are predefined and custom. So these Events are created on runtime and created by plugins. 
```
----

Discord Events
```
  In this type events are located, which are triggerd by there discord counter parts.
```

## Elements
Event Kind
```
  Custom Event {
    name: String
    arguments: EventArguments
  }
  
  Lifecycle Event {
    name: LifecycleHook
    arguments: EventArguments
  }
  
  Discord Event {
    name: DiscordEvent
    arguments: EventArguments
    
  }
```
---

Lifecycle Hook
```
  BotStart
  BotStop
    
  Start
  Stop
  Error {
    event: EventKind
    stack: Optional String
    message: String
  }
  
  PluginStart {
    name: String
  }
  
  PluginStop {
    name: String
  }
  
  PluginError {
    plugin_name: String
    event: EventKind
    stack: Optional String
    message: String
  }
```
----
Event Names
```
  event_type: "custom" or "lifecycle" or "discord"
  event_name: Static String
```
----

RuntimeConfigurationEntry
```
  name: Static String,
  error_message: Static Optional String,
  optional: Static boolean
```
----

Plugin Configuration
```
  name: Static String
  description: Static String
  runtime_config: Static Array of Runtime Configuration Entry
  required_events: Static Array of EventNames
  start_level: Static u8
  stop_level: Static u8
```
----

Plugin 
```
  fn plugin_config() -> PluginConfiguration
  
  fn handle_event(&self, event: EventKind) -> Future of Result of Nothing or Error
  fn handle_start(&mut self, bot: Bot) -> Future of Result of Nothing or Error
  fn handle_stop(&mut self, bot: Bot) -> Future of Result of Nothing or Error
```
