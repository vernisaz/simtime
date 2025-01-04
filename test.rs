fn main() {
    let (tz_off, dst) = simtime::get_local_timezone_offset_dst();
    println!{"local tz id: {:03}{:02} {dst}", tz_off/60, tz_off%60}
}