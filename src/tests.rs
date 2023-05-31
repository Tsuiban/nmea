use std::{assert_eq, fs};
use std::io::{BufRead, BufReader};
use super::*;

fn create_test_val_1() -> NmeaBaseSentence {
    let base_sentence = "$GPGGA,173617,4844.8683,N,12313.7709,W,2,11,1.00,2,M,-17.0,M,,*52".to_string();
    NmeaBaseSentence::from(&base_sentence)
}

fn create_test_val_hex() -> NmeaBaseSentence {
    let base_sentence = "$IIHDG,108.4,,,15.7,E*1C".to_string();
    NmeaBaseSentence::from(&base_sentence)
}

#[test]
fn test_u8() {
    let testval = create_test_val_1();
    let n = testval.get::<u8>(5);
    assert_eq!(n, Some(2));
}

#[test]
fn test_u16() {
    let testval = create_test_val_1();
    let n = testval.get::<u16>(5);
    assert_eq!(n, Some(2));
}

#[test]
fn test_u32() {
    let testval = create_test_val_1();
    let n = testval.get::<u32>(0);
    assert_eq!(n, Some(173617));
}

#[test]
fn test_string() {
    let testval = create_test_val_1();
    let n = testval.get::<String>(4);
    assert_eq!(n, Some("W".to_string()));
}

#[test]
fn test_f32() {
    let testval = create_test_val_1();
    let n = testval.get::<f32>(1);
    assert_eq!(n, Some(4844.8683));
}

#[test]
fn test_char() {
    let testval = create_test_val_1();
    let c = testval.get::<char>(2);
    assert_eq!(c, Some('N'));
}

#[test]
fn test_past_end () {
   let testval = create_test_val_1();
   let n = testval.get::<f32>(20);
   assert_eq!(n, None);
}

#[test]
fn test_non_existent() {
    let testval = create_test_val_1();
    let n = testval.get::<String>(12);
    assert_eq!(n, None);
}

#[test]
fn test_hex_u8_1 () {
    let testval = create_test_val_hex();
    let n = testval.get_hex::<u8>(4);
    assert_eq!(n, Some(14));
}

#[test]
fn test_n_fail () {
    let testval = create_test_val_1();
    let n = testval.get::<u8>(3);
    assert_eq!(n, None);
}

#[test]
fn test_hex_f32 () {
    let testval = create_test_val_hex();
    let n = testval.get_hex::<f32>(4);
    assert_eq!(n, Some(14.0));
}

#[test]
fn test_aam () {
    let s = "$YDAAM,A,A,3.2,N,waypoint*1E".to_string();
    let d = AamData::from(&s);
    let arrival_status = d.arrival_status();
    assert_eq!(arrival_status, Some('A'));
    let perpendicular_status = d.perpendicular_status();
    assert_eq!(perpendicular_status, Some('A'));
    let circle_radius = d.arrival_circle_radius().unwrap();
    assert_eq!(circle_radius.value, 3.2);
    assert_eq!(circle_radius.unit, 'N');
    let waypoint_id = d.waypoint_id();
    assert_eq!(waypoint_id.unwrap(), "waypoint".to_string());
}

#[test]
fn test_aam_perpendicular_status() {

}

#[test]
fn test_checksum () {
    let s = "$AAAAA,,*41".to_string();
    let d = NmeaBaseSentence::from(&s);
    assert_eq!(d.checksum(), 65);
}

#[test]
fn big_test () {
    if let Ok(f) = fs::File::open("test.log") {
        let bufreader = BufReader::new(f);
        for line in bufreader.lines() {
            assert!(line.is_ok());
            let sentence = NmeaBaseSentence::from(&line.unwrap());
            if sentence.nfields() == 0 {
                eprintln!("{:?}", sentence);
            }
            assert_ne!(sentence.nfields(), 0);
        }
    } else { assert!(false) }
}
