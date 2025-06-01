use std::{process::Command,

os::raw::{c_int,
c_char,
c_long,
//c_longlong
},

time::{SystemTime, UNIX_EPOCH}};

pub const DAYS_OF_WEEK: &[&str] = &[
    "Thursday",
    "Friday",
    "Saturday",
    "Sunday",
    "Monday",
    "Tuesday",
    "Wednesday",
];

#[repr(C)]
    pub struct tm {
        pub tm_sec: c_int,
        pub tm_min: c_int,
        pub tm_hour: c_int,
        pub tm_mday: c_int,
        pub tm_mon: c_int,
        pub tm_year: c_int,
        pub tm_wday: c_int,
        pub tm_yday: c_int,
        pub tm_isdst: c_int,
        pub tm_gmtoff: c_long,
        pub tm_zone: *const c_char,
    }

   #[cfg(unix)]
   extern "C" {
        fn localtime(time: *const c_long) -> *mut tm;
    }
    
    #[cfg(target_os = "windows")]
    extern "C" {
        fn _localtime64(time: *const c_longlong) -> *mut tm;
    }
    
pub const MON_DAYS : [u32;12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

pub fn get_datetime(epoch_year: u32, duration_sec: u64) -> (u32, u32, u32, u32, u32, u32, u8) {
    // year, month, day, hour, minute, second, a day in a week after epoch day
    let mut mon_days = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

    let mut days: u32 = (duration_sec / 86400) as u32;
    let sec_in_day = (duration_sec % 86400) as u32;
    let mins_in_day = sec_in_day / 60;
    let sec_in_min = sec_in_day % 60;
    let hour_in_day = mins_in_day / 60;
    let min_in_hour = mins_in_day % 60;
    let mut curr_year = epoch_year;
    let remaining_days_week: u8 = (days % 7) as u8;

    if days > year_len(curr_year) {
        loop {
            days -= year_len(curr_year);
            curr_year += 1;
            if days < year_len(curr_year) {
                break;
            }
        }
    }
    if year_len(curr_year) == 366 {
        mon_days[1] = 29;
    }
    let mut current_month: u32 = 0;
    if days > 0 {
        loop {
            if days < mon_days[current_month as usize] {
                break;
            }
            days -= mon_days[current_month as usize];
            current_month += 1;
        }
    }
    (
        curr_year,
        current_month + 1,
        days + 1,
        hour_in_day,
        min_in_hour,
        sec_in_min,
        remaining_days_week,
    )
}

const ZERO: u8 = b'0';
pub fn get_local_timezone_offset_ext() -> i16 {
    // returns 0 in a case of exception
    match Command::new("date").arg("+%z").output() {
        Ok(output) if output.status.success() => {
           let pos = output.stdout[0] == '+' as u8; 
           let mut hour : i16 = (&output.stdout[1] - ZERO) as i16;
           hour = hour * 10 + (&output.stdout[2] - ZERO) as i16;
           let mut min : i16 = (output.stdout[3] - ZERO) as i16;
           min = min * 10 + (&output.stdout[4] - ZERO) as i16;
           if pos {
               hour * 60 + min
           } else {
               -hour * 60 - min
           }
        },
        _ => 0
    }
}

pub fn get_local_timezone_offset() -> i16 {
    get_local_timezone_offset_dst().0
}

pub fn seconds_from_epoch(epoch: u32, year: u32, month: u32, day: u32, hour: u32, minute: u32, second: u32) -> Result<u64,&'static str> {
    if epoch > year { return Err("year too early") }
    if month < 1 || month > 12 { return Err("month out of range 1..12") }
    let mut full_years = 0u64;
    for y in epoch..year {
        full_years += year_len(y) as u64
    }
    for m in 1..month {
        full_years += MON_DAYS[(m as usize)-1usize] as u64;
        if m == 2 && year_len(year) == 366 {
            full_years += 1
        }
    }
    if day < 1 || day > MON_DAYS[(month as usize)-1] + if month == 2 && year_len(year) == 366 {1} else {0} { return Err("day out of month length") }
    full_years += (day as u64) - 1;
    let mut seconds = full_years * 24 * 60 * 60;
    if hour > 23 { return Err("the hour out of the day length") }
    seconds += (hour as u64) * 60 * 60;
    if minute > 59 { return Err("the minute out of the hour length") }
    seconds += (minute as u64) * 60;
    if second > 59 { return Err("the second out of the minute length") }
    seconds += second as u64;
    Ok(seconds)
}

#[cfg(unix)]
pub fn get_local_timezone_offset_dst() -> (i16, bool) {
    let now = 
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64;
    let local_time = unsafe { localtime(&now) };
    let tz_offset = unsafe {(*local_time).tm_gmtoff};
    ((tz_offset / 60) as i16, unsafe {(*local_time).tm_isdst != 0})
}

#[cfg(target_os = "windows")]
pub fn get_local_timezone_offset_dst() -> (i16, bool) {
    let now =SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;
    // https://learn.microsoft.com/en-us/cpp/c-runtime-library/reference/localtime-localtime32-localtime64?view=msvc-170
    let local_time = unsafe { _localtime64(&now) };
    let gmt_time = get_datetime(1970, now as _);
    let local_day = unsafe {(*local_time).tm_mday};
    let gmt_day = gmt_time.2 as i32;
    let mut day_diff:i32 = gmt_day - local_day;
    if day_diff > 1 {
        day_diff = -1;
    } else if day_diff < -1 {
        day_diff = 1;
    }
    let off_min = -((gmt_time.3 as i32 + 24 * day_diff) * 60 + gmt_time.4 as i32)  
      + ((unsafe {(*local_time).tm_hour} - 0) * 60 + unsafe {(*local_time).tm_min});
      
    (off_min as _, unsafe {(*local_time).tm_isdst > 0})
}
  

#[inline]
fn year_len(year: u32) -> u32 {
    if (year % 4) == 0 && (year % 100) != 0 || (year % 400) == 0 {
        366
    } else {
        365
    }
}
