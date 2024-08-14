
use bevy::app::{App, Plugin, PreUpdate};
use bevy::ecs::system::{Resource, ResMut, Res};
use bevy::ecs::event::{Event, EventWriter};
use bevy::ecs::schedule::{IntoSystemConfigs, SystemSet};
use bevy::time::{Time, TimerMode, Timer};
use bevy::reflect::Reflect;
use bevy::ecs::reflect::ReflectResource;
use serde::{Serialize,Deserialize};

use std::ops::Add;
use std::time::Duration;
use chrono::NaiveDate;
use chrono::naive::Days;
use bevy::utils::HashMap;

pub struct PGCalendarPlugin {
    pub active:         bool,
    pub hour_length:    u64,       // How many real-time seconds the in-game hour will last
    pub start_hour:     u8,
    pub start_weekday:  u8,
    pub start_date:     String     // Date String in YYYY-MM-DD format
}

impl Default for PGCalendarPlugin {
    fn default() -> Self {
        PGCalendarPlugin{
            active:        true,
            hour_length:   5,
            start_hour:    6, // from 0 to 23 
            start_weekday: 1, // from 1 to 7, 1 is Monday, 7 is Sunday
            start_date:    "2000-01-01".to_string()
        }
    }
}

impl Plugin for PGCalendarPlugin {
    fn build(&self, app: &mut App) {
        app
        .register_type::<Calendar>()
        .add_event::<CalendarNewDayEvent>()
        .add_event::<CalendarNewHourEvent>()
        .configure_sets(PreUpdate, PGCalendarSet::Calendar)
        .insert_resource(
            Calendar::new(
                self.active,
                self.start_hour, 
                self.start_weekday, 
                self.hour_length,
                &self.start_date
            )
        )
        .insert_resource(CalendarTimer::new(self.hour_length))
        .insert_resource(Weekdays::new())
        .add_systems(PreUpdate,  
            (
                update_settings.run_if(if_calendar_hour_length_changed), 
                update_time.run_if(if_calendar_active)
            ).chain()
             .in_set(PGCalendarSet::Calendar)
        );
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum PGCalendarSet {
    Calendar
}

pub fn if_calendar_active(
    calendar: Res<Calendar>
) -> bool {
    calendar.active
}
pub fn if_calendar_hour_length_changed(
    calendar: Res<Calendar>
) -> bool {
    calendar.old_hour_length != calendar.hour_length
}

#[derive(Event)]
pub struct CalendarNewDayEvent {
    pub weekday:    u8
}

#[derive(Event)]
pub struct CalendarNewHourEvent {
    pub hour: u8
}


// Updates calendar
fn update_time(
    time:                Res<Time>, 
    mut calendar_timer:  ResMut<CalendarTimer>, 
    mut calendar:        ResMut<Calendar>,
    mut new_day_event:   EventWriter<CalendarNewDayEvent>,
    mut new_hour_event:  EventWriter<CalendarNewHourEvent>
){

    calendar_timer.timer.tick(time.delta());
    calendar_timer.calc();

    if calendar_timer.timer.finished(){
        calendar.current_hour += 1;

        if calendar.current_hour == 24 {
            calendar.current_hour = 0;
            calendar.days_passed += 1;
            calendar.current_weekday += 1;
            let new_date: NaiveDate = calendar.current_date.add(Days::new(1));
            calendar.current_date = new_date;

            if calendar.current_weekday > 7 {
                calendar.current_weekday = 1;
            } 
            new_day_event.send(CalendarNewDayEvent {weekday: calendar.current_weekday });
        }

        new_hour_event.send(CalendarNewHourEvent { hour: calendar.current_hour });
    } 
}

fn update_settings(mut calendar: ResMut<Calendar>,
                   mut timer:    ResMut<CalendarTimer>
){
    if calendar.old_hour_length != calendar.hour_length {
        timer.set_hour_length(calendar.hour_length);
        calendar.old_hour_length = calendar.hour_length;
    }
    
}


#[derive(Resource, Clone, Serialize, Deserialize, Reflect)]
#[reflect(Resource)]
pub struct Calendar {
    days_passed:        u128,
    current_hour:       u8,         // from 0 to 24
    current_weekday:    u8,         // from 1 to 7
    #[reflect(ignore)]
    current_date:       NaiveDate,
    old_hour_length:    u64,
    hour_length:        u64,
    start_hour:         u8,      // from 0 to 23 
    start_weekday:      u8,      // from 1 to 7, 1 is Monday, 7 is Sunday
    start_date:         String,  // in yyyy-mm-dd format
    active:             bool,
}
impl Calendar {
    pub fn new(active: bool, start_hour: u8, start_weekday: u8, hour_length: u64, start_date: &str) -> Self {
        Calendar{
            active,
            old_hour_length: hour_length,
            hour_length,
            start_hour,
            start_weekday,
            start_date:      start_date.to_string(),
            days_passed:     0, 
            current_hour:    start_hour,
            current_weekday: start_weekday,
            current_date:    NaiveDate::parse_from_str(start_date, "%Y-%m-%d").ok().unwrap()
        }
    }
    pub fn get_days_passed(&self) -> u128 {
        self.days_passed
    }
    pub fn add_days_passed(&mut self){
        self.days_passed += 1;
    }
    pub fn get_current_hour(&self) -> u8 {
        self.current_hour
    }
    pub fn set_current_hour(&mut self, hour: u8){
        self.current_hour = hour;
    }
    pub fn get_current_weekday(&self) -> u8 {
        self.current_weekday
    }
    pub fn get_currrent_date(&self) -> NaiveDate {
        self.current_date
    }
    pub fn set_hour_length(&mut self, hour_length: u64) {
        self.old_hour_length = self.hour_length;
        self.hour_length = hour_length
    }
    pub fn activate(&mut self){
        self.active = true;
    }
    pub fn deactivate(&mut self){
        self.active = false;
    }
    pub fn get_hour_length(&self) -> u64 {
        self.hour_length
    }
    pub fn get_active(&self) -> bool {
        self.active
    }
    pub fn reset(&mut self) {
        self.days_passed = 0;
        self.current_hour = self.start_hour;
        self.current_weekday = self.start_weekday;
        self.current_date = NaiveDate::parse_from_str(&self.start_date, "%Y-%m-%d").ok().unwrap();
    }
}

#[derive(Resource)]
struct CalendarTimer {
    timer:          Timer,
    elapsed_pct:    f32
}

impl CalendarTimer {
    fn new(hour_length: u64) -> Self {
        CalendarTimer{
            timer: Timer::new(Duration::from_secs(hour_length), TimerMode::Repeating), 
            elapsed_pct: 0.0
        }
    }
    fn calc(&mut self){
        self.elapsed_pct = self.timer.elapsed_secs()/self.timer.duration().as_secs() as f32;
    }
    fn set_hour_length(&mut self, hour_length: u64) {
        self.timer.set_duration(Duration::from_secs(hour_length));
        let elapsed_time = (self.elapsed_pct * hour_length as f32) as u64;
        self.timer.set_elapsed(Duration::from_secs(elapsed_time));
    }
}

// Formats time of day to nice format
pub fn format_time(t: u8) -> String {
    if t <= 12 {
        return format!("{:.0}:00 AM", t);
    } else {
        return format!("{:.0}:00 PM", t-12);
    }
}

#[derive(Default, Resource)]
pub struct Weekdays{
  pub data: HashMap<u8, String>
}
impl Weekdays {
    pub fn new() -> Self {
        let mut weekdays = Weekdays { data: HashMap::new() };
        weekdays.data.insert(1, "Mon".to_string());
        weekdays.data.insert(2, "Tue".to_string());
        weekdays.data.insert(3, "Wed".to_string());
        weekdays.data.insert(4, "Thu".to_string());
        weekdays.data.insert(5, "Fri".to_string());
        weekdays.data.insert(6, "Sat".to_string());
        weekdays.data.insert(7, "Sun".to_string());
        return weekdays;
    }
}
