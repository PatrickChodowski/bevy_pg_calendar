# Calendar plugin for Bevy

(PG stands for PatchGames)

Calendar/Time tracking for Bevy Games that I use in my own projects. 

Its quite simple, it tracks Years, Weeks, Days and hours passed during the game, according to hour_length attribute (in seconds). That is if hour_length=1 then 1 second of real-time will be one hour in the game.

Feel free to use the code it as a template, I dont plan to extend it massively unless I need it in my own project.


## Plugin Attributes:
- active (bool): Toggle if the time is tracked or not
- hour_length (u64): in seconds, how many realtime seconds is in one in-game hour
- start_hour: (u8): Starting hour in game, from 0 to 23
- start_weekday: (u8): Starting weekday in game, from 1 to 7, 1 is Monday, 7 is Sunday.
- start_date (String): String of date that will be converted to NaiveDate. Its in yyyy-mm-dd format.

Default:
```{rust}

        PGCalendarPlugin{
            active:        true,
            hour_length:   5,
            start_hour:    6, // from 0 to 23 
            start_weekday: 1, // from 1 to 7, 1 is Monday, 7 is Sunday
            start_date:    "2000-01-01".to_string()
        }

```

TODO: more docs


## API

### Calendar resource

### CalendarNewDayEvent

### CalendarNewHourEvent

### Weekdays resource

### format_time

### if_calendar_active

### if_calendar_hour_length_changed

