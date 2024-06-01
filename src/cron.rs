use crate::calendar::Calendar;
use bevy::ecs::system::Res;
use serde::{Deserialize,Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Cron {
    pub formula:          String,
    pub hours:            Option<Vec<u8>>,
    pub days_month:       Option<Vec<u8>>,
    pub months:           Option<Vec<u8>>,
    pub days_week:        Option<Vec<u8>>,
}

impl Cron {
    pub fn new(formula: &str) -> Self {
        let mut c = Cron{formula: formula.to_string(), hours: None, days_month: None, months: None, days_week: None};
        c.parse();
        return c;
    }

    pub fn is_time(&self, calendar: &Res<Calendar>) -> bool {
        self.hours.as_ref().unwrap().contains(&calendar.get_current_hour()) && 
        self.days_week.as_ref().unwrap().contains(&calendar.get_current_weekday())
    }

    pub fn parse(&mut self){

        let mut a = self.formula.splitn(4, " ");
        let hour_ = a.next().unwrap();
        let day_month_ = a.next().unwrap();
        let month_ = a.next().unwrap();
        let day_week_ = a.next().unwrap();
    
        self.hours = parse_part_cron(hour_, 0, 23);
        self.days_month = parse_part_cron(day_month_, 1, 30);
        self.months = parse_part_cron(month_, 1, 12);
        self.days_week = parse_part_cron(day_week_, 1, 7);

    }
}

fn parse_range_cron(s: &str) -> Option<Vec<u8>> {
    let mut h = s.splitn(2, "-");
    let error_str = format!("[CRON] [ERROR] parsing range: {}", s);
    let min_h: u8 = h.next().unwrap().parse().expect(&error_str);
    let max_h: u8 = h.next().unwrap().parse().expect(&error_str);
    return Some((min_h..=max_h).collect());
}


fn parse_list_cron(s: &str) -> Option<Vec<u8>> {
    let mut v: Vec<u8> = Vec::new();
    let error_str = format!("[CRON] [ERROR] parsing list: {}", s);
    let numbs = s.split(";");
    for n in numbs {
        v.push(n.parse().expect(&error_str));
    }
    return Some(v);
}


fn parse_part_cron(s: &str, minl: u8, maxl: u8) -> Option<Vec<u8>> {
    if s == "*" {
        return Some((minl..=maxl).collect());
    } else if s.contains("-"){
        return parse_range_cron(s);
    } else if s.contains(";"){
        return parse_list_cron(s);
    } else {
        let error_str = format!("[CRON] [ERROR] parsing value: {}", s);
        let v = vec![s.parse().expect(&error_str)];
        return Some(v);
    }
}
