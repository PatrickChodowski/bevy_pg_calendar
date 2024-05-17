mod bevy_pg_calendar;

pub mod prelude {
    pub use crate::bevy_pg_calendar::{PGCalendarPlugin, Calendar, Weekdays,
        CalendarNewDayEvent, CalendarNewHourEvent, format_time};
}
