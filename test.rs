fn main() {
    if let Ok(met) = std::env::var("REQUEST_METHOD")
        && met == "POST"
    {
        println!("neve mind")
    }
    let (tz_off, dst) = simtime::get_local_timezone_offset_dst();
    println! {"local tz id: {:03}{:02} {dst}", tz_off/60, tz_off%60}
    assert_eq!(
        simtime::seconds_from_epoch(1970, 2025, 5, 31, 22, 33, 44).unwrap_or(0),
        1748730824
    );
    assert_eq!(
        simtime::seconds_from_epoch(1970, 2024, 2, 29, 13, 33, 35).unwrap_or(0),
        1709213615
    );
    assert_eq!(
        simtime::seconds_from_epoch(1970, 2025, 2, 29, 22, 33, 44).unwrap(),
        1740836015
    )
}
