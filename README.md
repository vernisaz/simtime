# crate simtime

The crate provides different time display
abilities for Rust programs and crates.

## examples

```
fn time() -> String {
    let dur = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let tz_off = simtime::get_local_timezone_offset();
    let (y,m,d,h,min,s,w) = simtime::get_datetime(1970, ((dur.as_secs() as i64) + (tz_off as i64)*60) as u64);
    
    format!("{m:0>2}-{:0>2}-{:0>2} {}, {:0>2}:{:0>2}:{:0>2} {:03}{:02}",
         d,y, DAYS_OF_WEEK[w as usize],h,min,s, tz_off/60, tz_off%60)
}
```
