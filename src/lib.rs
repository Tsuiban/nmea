extern crate core;

use std::io::{BufRead};
use std::str::FromStr;
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use num_traits::Num;
use crate::NmeaSentence::{AAM, ABK, ABM, ACA, ACK, ACS, ADS, AIR, AKD, ALA, ALM, ALR, APA, APB, ASD, BBM, BEC, BOD, BWC, BWR, BWW, CEK, COP, CUR, DBK, DBS, DBT, DCN, DCR, DDC, DOR, DPT, DSC, DSE, DSI, DSR, DTM, ETL, EVE, FIR, FSI, GBS, GGA, GLC, GLL, GMP, GNS, GRS, GSA, GST, GSV, GTD, GXA, HDG, HDM, HDT, HFB, HMR, HMS, HSC, HTC, HTD, ITS, LCD, LR1, LR2, LR3, LRF, MDA, MLA, MSK, MSS, MTW, MWD, MWV, OLN, OSD, R00, RLM, RMA, RMB, RMC, ROT, RPM, RSA, RSD, RTE, SF1, SSD, STN, TDS, TFI, TLB, TLL, TPC, TPR, TPT, TRF, TTM, TUT, TXT, VBW, VDM, VDO, VDR, VHW, VLW, VPW, VSD, VTG, VWR, WCV, WDC, WDR, WNC, WPL, XDR, XTE, XTR, ZDA, ZDL, ZFO, ZTG};

macro_rules! make_data {
    ($i:ident) => {
        #[derive(Debug)]
        pub struct $i {
            base : NmeaBaseSentence,
        }
        impl NmeaBaseTrait for $i {
            fn get_base(&self) -> &NmeaBaseSentence {
                &self.base
            }
        }
        impl From<&String> for $i {
            fn from(value : &String) -> Self {
                Self {
                    base : NmeaBaseSentence::from(value),
                }
            }
        }
    };
}

macro_rules! make_number_field {
    ($a:ident, $b:ident, $c:expr) => {
        pub fn $a(&self) -> Option<$b> {
            self.base.get::<$b>($c)
        }
    };
    ($a:ident, $b:ident, $c:expr, $d:expr) => {
        pub fn $a(&self) -> Option<ValueWithUnit<$b>> {
            self.base.get_pair::<$b>($c, $d)
        }
    }
}

macro_rules! make_char_field {
    ($a:ident, $b:expr) => {
        pub fn $a(&self) -> Option<char> {
            self.base.get::<char>($b)
        }
    }
}

macro_rules! make_string_field {
    ($a:ident, $b:expr) => {
        pub fn $a(&self) -> Option<String> {
            self.base.get::<String>($b)
        }
    }
}

macro_rules! make_time_field {
    ($a:ident, $b:expr) => {
        pub fn $a(&self) -> Option<NaiveTime> {
            self.base.get_time($b)
        }
    }
}

macro_rules! make_date_field {
    ($a:ident, $b:expr) => {
        pub fn $a(&self) -> Option<NaiveDate> {
            self.base.get_date($b)
        }
    }
}

macro_rules! make_coordinate_field {
    ($a:ident, $b:expr, $c:expr) => {
        pub fn $a(&self) -> Option<f32> {
            self.base.get_coordinate($b, $c)
        }
    }
}

macro_rules! make_hex_field {
    ($a:ident, $b:ident, $c:expr) => {
        pub fn $a(&self) -> Option<$b> {
            self.base.get_hex::<$b>($c)
        }
    }
}

pub trait NmeaBaseTrait {
    fn get_base(&self) -> &NmeaBaseSentence;

    fn sender(&self) -> &String {
        &self.get_base()._sender
    }

    fn message(&self) -> &String {
        &self.get_base()._message_type
    }

    fn nfields(&self) -> usize {
        self.get_base()._fields.len()
    }

    fn checksum(&self) -> u8 {
        self.get_base()._checksum
    }
}

#[derive(Debug)]
pub struct ValueWithUnit<T> {
    value: T,
    unit: char,
}

#[derive(Debug)]
pub struct NmeaBaseSentence {
    _sender: String,
    _message_type: String,
    _fields: Vec<String>,
    _checksum: u8,
    _original: String,
}

/// Used if there is an error in parsing an input string.
#[derive(Debug)]
pub struct ErrorData {
    pub error : String,
    pub message : String,
}

make_data!(AamData);
make_data!(AbkData);
make_data!(AcaData);
make_data!(AckData);
make_data!(AcsData);
make_data!(AdsData);
make_data!(AirData);
make_data!(AkdData);
make_data!(AlaData);
make_data!(AlmData);
make_data!(AlrData);
make_data!(ApaData);
make_data!(ApbData);
make_data!(AsdData);
make_data!(BecData);
make_data!(BodData);
make_data!(BwcData);
make_data!(BwrData);
make_data!(BwwData);
make_data!(CekData);
make_data!(CopData);
make_data!(CurData);
make_data!(DbkData);
make_data!(DbsData);
make_data!(DbtData);
make_data!(DcnData);
make_data!(DcrData);
make_data!(DdcData);
make_data!(DorData);
make_data!(DptData);
make_data!(DscData);
make_data!(DseData);
make_data!(DsiData);
make_data!(DsrData);
make_data!(DtmData);
make_data!(EtlData);
make_data!(EveData);
make_data!(FirData);
make_data!(FsiData);
make_data!(GbsData);
make_data!(GgaData);
make_data!(GlcData);
make_data!(GllData);
make_data!(GmpData);
make_data!(GnsData);
make_data!(GrsData);
make_data!(GsaData);
make_data!(GstData);
make_data!(GsvData);
make_data!(GtdData);
make_data!(GxaData);
make_data!(HdgData);
make_data!(HdmData);
make_data!(HdtData);
make_data!(HfbData);
make_data!(HmrData);
make_data!(HmsData);
make_data!(HscData);
make_data!(HtcData);
make_data!(HtdData);
make_data!(ItsData);
make_data!(LcdData);
make_data!(LrfData);
make_data!(Lr1Data);
make_data!(Lr2Data);
make_data!(Lr3Data);
make_data!(MdaData);
make_data!(MlaData);
make_data!(MskData);
make_data!(MssData);
make_data!(MtwData);
make_data!(MwdData);
make_data!(MwvData);
make_data!(OlnData);
make_data!(OsdData);
make_data!(R00Data);
make_data!(RlmData);
make_data!(RmaData);
make_data!(RmbData);
make_data!(RmcData);
make_data!(RotData);
make_data!(RpmData);
make_data!(RsaData);
make_data!(RsdData);
make_data!(RteData);
make_data!(Sf1Data);
make_data!(SsdData);
make_data!(StnData);
make_data!(TdsData);
make_data!(TfiData);
make_data!(TlbData);
make_data!(TllData);
make_data!(TpcData);
make_data!(TprData);
make_data!(TptData);
make_data!(TrfData);
make_data!(TtmData);
make_data!(TutData);
make_data!(TxtData);
make_data!(VbwData);
make_data!(VdrData);
make_data!(VhwData);
make_data!(VlwData);
make_data!(VpwData);
make_data!(VsdData);
make_data!(VtgData);
make_data!(VwrData);
make_data!(WcvData);
make_data!(WdcData);
make_data!(WdrData);
make_data!(WncData);
make_data!(WplData);
make_data!(XdrData);
make_data!(XteData);
make_data!(XtrData);
make_data!(ZdaData);
make_data!(ZdlData);
make_data!(ZfoData);
make_data!(ZtgData);
make_data!(AbmData);
make_data!(BbmData);
make_data!(VdmData);
make_data!(VdoData);

impl NmeaBaseSentence {
    pub fn new(value : String) -> Self {
        Self {
            _sender: "".to_string(),
            _message_type: "".to_string(),
            _fields: vec![],
            _checksum: 0,
            _original : value,
        }
    }

    pub fn default() -> Self {
        Self {
            _sender: "".to_string(),
            _message_type: "".to_string(),
            _fields: vec![],
            _checksum: 0,
            _original: "".to_string(),
        }
    }

    pub fn sender(&self) -> String {
        self._sender.clone()
    }

    pub fn message_type(&self) -> String {
        self._message_type.clone()
    }

    pub fn checksum(&self) -> u8 {
        self._checksum
    }

    pub fn get<T: FromStr>(&self, index: usize) -> Option<T> {
        if index < self.nfields() && !self._fields[index].is_empty() {
            match self._fields[index].parse::<T>() {
                Ok(v) => Some(v),
                _ => None
            }
        } else { None }
    }

    pub fn get_hex<T: Num>(&self, index: usize) -> Option<T> {
        if index < self.nfields() && !self._fields[index].is_empty() {
            match T::from_str_radix(&self._fields[index], 16) {
                Ok(v) => Some(v),
                _ => None
            }
        } else { None }
    }

    pub fn get_pair<T: FromStr>(&self, index1: usize, index2 : usize) -> Option<ValueWithUnit<T>> {
        if let Some(v) = self.get::<T>(index1) {
            if let Some(c) = self.get::<char>(index2) {
                return Some(ValueWithUnit {
                    value: v,
                    unit: c,
                });
            }
        };
        None
    }

    pub fn get_time(&self, index: usize) -> Option<NaiveTime> {
        if index >= self._fields.len() || self._fields[index].is_empty() {
            return None
        };
        let field = &self._fields[index];
        if let Ok(hours) = field[0..2].parse::<u32>() {
            if let Ok(minutes) = field[2..4].parse::<u32>() {
                if let Ok(seconds) = field[5..].parse::<f32>() {
                    let millis = ((seconds-seconds.floor()) * 1000.0).floor() as u32;
                    let seconds = seconds.floor() as u32;
                    return NaiveTime::from_hms_milli_opt(hours, minutes, seconds, millis);
                }
            }
        }
        None
    }

    pub fn get_coordinate(&self, index_n: usize, index_n_s_e_w: usize) -> Option<f32> {
        if let Some(coordinate) = self.get::<f32>(index_n) {
            if let Some(hemisphere) = self.get::<char>(index_n_s_e_w) {
                return if hemisphere == 'S' || hemisphere == 'W' {
                    Some(-coordinate)
                } else {
                    Some(coordinate)
                }
            }
        };
        None
    }

    pub fn get_date(&self, index: usize) -> Option<NaiveDate> {
        if let Some(t) = self.get::<String>(index)  {
            let days_s = &t[0..2];
            let month_s = &t[2..4];
            let year_s = &t[4..];
            let days = days_s.parse::<u32>().unwrap_or(0);
            let months = month_s.parse::<u32>().unwrap_or(0);
            let years = year_s.parse::<i32>().unwrap_or(0) + 2000;
            NaiveDate::from_ymd_opt(years, months, days)
        } else {
            None
        }
    }

	fn original(&self) -> Option<&String> {
		if self._original.is_empty() {
			None
		} else {
			self.get_base().original()
		}
	}
}

impl Clone for NmeaBaseSentence {
    fn clone(&self) -> Self {
        Self {
            _sender: self._sender.clone(),
            _message_type: self._message_type.clone(),
            _fields: self._fields.clone(),
            _checksum: self._checksum,
            _original: self._original.clone(),
        }
    }
}

impl From<&String> for NmeaBaseSentence {
    fn from(value: &String) -> Self {
        // Integrity checks...
        //    Is it at least 9 characters long? $SSMMM*HH
        //    or the third-to-last character is not an asterisk
        let value = value.clone();
        let message_length = value.len();
        if message_length < 9 || value[message_length-3..message_length-2] != *"*" {
            return NmeaBaseSentence::new(value);
        }
        let checksum_string = &value[message_length-2..];
        let calculated_checksum : u8;
        if let Ok(checksum) = u8::from_str_radix(&checksum_string, 16) {
            calculated_checksum=
                value[1..message_length-3]
                    .bytes().fold(0, |acc, x| acc ^ x);
            if checksum != calculated_checksum {
                return NmeaBaseSentence::new(value);
            }
        } else {
            return NmeaBaseSentence::new(value);
        }

        // All OK - time to get down to business...
        if let Some((prolog, epilog)) = value.split_once(',') {
            let prolog_length = prolog.len();
            let sender_length = prolog_length - 3;
            let sender = prolog[0..sender_length].to_string();
            let message = prolog[sender_length..].to_string();
            let mut fields = epilog.split(',')
                .map(|x| x.to_string())
                .collect::<Vec<String>>();
            let last = fields.pop().unwrap();
            let (body, _): (&str, &str) = last.split_once('*').unwrap_or(("", ""));
            fields.push(body.to_string());
            Self {
                _sender: sender,
                _message_type: message,
                _fields: fields,
                _checksum: calculated_checksum,
                _original: value.clone(),
            }
        } else {
            Self::new(value)
        }
    }
}

impl NmeaBaseTrait for NmeaBaseSentence {
    fn get_base(&self) -> &NmeaBaseSentence {
        self
    }
}

impl AamData {
    make_char_field!(arrival_status, 0);
    make_char_field!(perpendicular_status, 1);
    make_number_field!(arrival_circle_radius, f32, 2, 3);
    make_string_field!(waypoint_id, 4);
}

impl AbkData {
    make_string_field!(mmsi, 0);
    make_number_field!(channel, u16, 1);
    make_number_field!(message_id, u16, 2);
    make_number_field!(sequence_number, u16, 3);
    make_number_field!(acknowledgement, u8, 4);
}

impl AcaData {
    make_number_field!(sequence_number, u8, 0);
    make_coordinate_field!(ne_latitude, 1, 2);
    make_coordinate_field!(ne_longitude, 3, 4);
    make_coordinate_field!(sw_latitude, 5, 6);
    make_coordinate_field!(sw_longitude, 7, 8);
    make_number_field!(transition_zone_size, f32, 9);
    make_number_field!(channel_a, u16, 10);
    make_number_field!(channel_a_bandwidth, f32, 11);
    make_number_field!(channel_b, u16, 12);
    make_number_field!(channel_b_bandwidth, f32, 13);
    make_number_field!(tx_rx_mode_control, u8, 14);
    make_number_field!(power_level, u8, 15);
    make_char_field!(information_source, 16);
    make_number_field!(in_use_flag, u8, 17);
    make_time_field!(time, 18);
}

impl AckData {
    make_number_field!(alarm_number, u8, 0);
}

impl AcsData {
    make_number_field!(sequence_number, u8, 0);
    make_string_field!(mmsi, 1);
    make_time_field!(time, 2);
    make_number_field!(day, u8, 3);
    make_number_field!(month, u8, 4);
    make_number_field!(year, u16, 5);

    pub fn timestamp(&self) -> Option<DateTime<Utc>> {
        if let Some(t) = self.base.get_time(2) {
            if let Some(day) = self.base.get::<u32>(3) {
                if let Some(month) = self.base.get::<u32>(4) {
                    if let Some(year) = self.base.get::<i32>(5) {
                        if let Some(d) = NaiveDate::from_ymd_opt(year, month, day) {
                            let n = NaiveDateTime::new(d, t);
                            return Some(DateTime::<Utc>::from_utc(n, Utc));
                        }
                    }
                }
            }
        };
        None
    }
}

impl AirData {
    make_string_field!(mmsi, 0);
    make_number_field!(requested_message, u8, 1);
    make_number_field!(sub_section, u8, 2);
    make_number_field!(requested_message_2, u8, 3);
    make_number_field!(sub_section_2, u8, 4);
    make_string_field!(mmsi_station_2, 5);
    make_number_field!(requested_message_station_2, u8, 6);
    make_number_field!(sub_section_station_2, u8, 7);
}

impl AlmData {
    make_number_field!(sentence_count_total, u16, 0);
    make_number_field!(sentence_number, u16, 1);
    make_number_field!(satellite_prn, u8, 2);
    make_number_field!(gps_week_number, u16, 3);
    make_hex_field!(health, u8, 4);
    make_hex_field!(eccentricity, f32, 5);
    make_hex_field!(reference_time, u8, 6);
    make_hex_field!(inclination_angle, f32, 7);
    make_hex_field!(right_ascension_rate, f32, 8);
    make_hex_field!(root_semi_major_axis, f32, 9);
    make_hex_field!(perigee, f32, 10);
    make_hex_field!(ascension_node_longitude, f32, 11);
    make_hex_field!(mean_anomaly, f32, 12);
    make_hex_field!(clock_parameter_f0, f32, 13);
    make_hex_field!(clock_parameter_f1, f32, 14);
}

impl AlrData {
    make_time_field!(time, 0);
    make_number_field!(alarm_id, u16, 1);
    make_char_field!(condition, 2);
    make_char_field!(acknowledge_state, 3);
    make_string_field!(text, 4);
}

impl ApaData {
    make_char_field!(status_blink_snr_warning, 0);
    make_char_field!(status_cycle_lock_warning, 1);
    make_number_field!(xte, f32, 2, 4);
    make_char_field!(steer_direction, 3);
    make_char_field!(arrival_status, 5);
    make_char_field!(perpendicular_status, 6);
    make_number_field!(bearing_origin_to_destination, f32, 7, 8);
    make_string_field!(destination_id, 9);
}

impl ApbData {
    make_char_field!(status_a, 0);
    make_char_field!(status_v, 1);
    make_number_field!(xte, f32, 2, 4);
    make_char_field!(steer_direction, 3);
    make_char_field!(arrival_status, 5);
    make_char_field!(perpendicular_status, 6);
    make_number_field!(bearing_origin_to_destination, f32, 7, 8);
    make_string_field!(destination_id, 9);
    make_number_field!(bearing_present_to_destination, f32, 10, 11);
    make_number_field!(heading_to_steer_to_waypoint, f32, 12, 13);
}

impl BodData {
    make_number_field!(bearing_true, f32, 0, 1);
    make_number_field!(bearing_magnetic, f32, 2, 3);
    make_string_field!(destination_waypoint_id, 4);
    make_string_field!(origin_waypoint_id, 5);
}

impl BwcData {
    make_time_field!(time, 0);
    make_number_field!(latitude, f32, 1, 2);
    make_number_field!(longitude, f32, 3, 4);
    make_number_field!(bearing_true, f32, 5, 6);
    make_number_field!(bearing_magnetic, f32, 7, 8);
    make_number_field!(distance, f32, 9, 10);
    make_string_field!(waypoint_id, 11);
    make_char_field!(mode_indicator, 12);
}

impl BwrData {
    make_time_field!(time, 0);
    make_number_field!(latitude, f32, 1, 2);
    make_number_field!(longitude, f32, 3, 4);
    make_number_field!(bearing_true, f32, 5, 6);
    make_number_field!(bearing_magnetic, f32, 7, 8);
    make_number_field!(distance, f32, 9, 10);
    make_string_field!(waypoint_id, 11);
    make_char_field!(mode_indicator, 12);
}

impl BwwData {
    make_number_field!(bearing_true, f32, 0, 1);
    make_number_field!(bearing_magnetic, f32, 2, 3);
    make_string_field!(destination_waypoint_id, 4);
    make_string_field!(origin_waypoint_id, 5);
}

impl DbkData {
    make_number_field!(depth_feet, f32, 0, 1);
    make_number_field!(depth_meters, f32, 2, 3);
    make_number_field!(depth_fathoms, f32, 4, 5);
}

impl DbsData {
    make_number_field!(depth_feet, f32, 0, 1);
    make_number_field!(depth_meters, f32, 2, 3);
    make_number_field!(depth_fathoms, f32, 4, 5);
}

impl DbtData {
    make_number_field!(depth_feet, f32, 0, 1);
    make_number_field!(depth_meters, f32, 2, 3);
    make_number_field!(depth_fathoms, f32, 4, 5);
}

impl DcnData {
    make_number_field!(chain_identifier, f32, 0);
    make_string_field!(red_zone_identider, 1);
    make_number_field!(red_lop, f32, 2);
    make_char_field!(red_master_status, 3);
    make_string_field!(green_zone_identifier, 4);
    make_number_field!(green_lop, f32, 5);
    make_char_field!(green_master_status, 6);
    make_string_field!(purple_zone_identifier, 7);
    make_number_field!(purple_lop, f32, 8);
    make_char_field!(purple_master_status, 9);
    make_char_field!(red_navigation_use, 10);
    make_char_field!(green_navigation_use, 11);
    make_char_field!(purple_navigation_use, 12);
    make_number_field!(position_uncertainty, f32, 13, 14);
    make_number_field!(fix_data_basis, u8, 15);
}

impl DptData {
    make_number_field!(water_depth_meters, f32, 0);
    make_number_field!(transducer_offset, f32, 1);
    make_number_field!(maximum_range_scale, f32, 2);
}

impl DtmData {
    make_string_field!(datum_code, 0);
    make_number_field!(datum_subcode, u8, 1);
    make_number_field!(latitude, f32, 2, 3);
    make_number_field!(longitude, f32, 4, 5);
    make_number_field!(altitude, f32, 6);
    make_string_field!(datum, 7);
}

impl FsiData {
    make_number_field!(transmitting_frequency, f32, 0);
    make_number_field!(receiving_frequency, f32, 1);
    make_char_field!(communications_mode, 2);
    make_number_field!(power_level, u8, 3);
}

impl GbsData {
    make_time_field!(time, 0);
    make_number_field!(expected_latitude_error, f32, 1);
    make_number_field!(expected_longitude_error, f32, 2);
    make_number_field!(expected_altitude_error, f32, 3);
    make_number_field!(failed_satellite_id, u16, 4);
    make_number_field!(probability_missed_detection, f32, 5);
    make_number_field!(estimated_bias, f32, 6);
    make_number_field!(standard_deviation_estimated_bias, f32, 7);
}

impl GgaData {
    make_time_field!(time, 0);
    make_number_field!(latitude, f32, 1, 2);
    make_number_field!(longitude, f32, 3, 4);
    make_number_field!(quality_indicator, u8, 5);
    make_number_field!(satellites_in_use_count, u8, 6);
    make_number_field!(hdop, f32, 7);
    make_number_field!(antenna_altitude, f32, 8, 9);
    make_number_field!(geoidal_separation, f32, 10, 11);
    make_number_field!(age_differential_gps_data, f32, 12);
    make_number_field!(differential_station_id, u16, 13);
}

impl GlcData {
    make_number_field!(gri_microseconds, f32, 0);
    make_number_field!(master_toa_microseconds, f32, 1);
    make_char_field!(master_toa_status, 2);
    make_number_field!(time_difference_1_microseconds, f32, 3);
    make_char_field!(time_difference_1_signal_status, 4);
    make_number_field!(time_difference_2_microseconds, f32, 5);
    make_char_field!(time_difference_2_signal_status, 6);
    make_number_field!(time_difference_3_microseconds, f32, 7);
    make_char_field!(time_difference_3_signal_status, 8);
    make_number_field!(time_difference_4_microseconds, f32, 9);
    make_char_field!(time_difference_4_signal_status, 10);
    make_number_field!(time_difference_5_microseconds, f32, 11);
    make_char_field!(time_difference_5_signal_status, 12);
}

impl GllData {
    make_number_field!(latitude, f32, 0, 1);
    make_number_field!(longitude, f32, 2, 3);
    make_time_field!(time, 4);
    make_char_field!(status, 5);
    make_char_field!(mode, 6);
}

impl GnsData {
    make_time_field!(time, 0);
    make_number_field!(latitude, f32, 1, 2);
    make_number_field!(longitude, f32, 3, 4);
    make_char_field!(mode, 5);
    make_number_field!(satellites_in_use_count, u8, 6);
    make_number_field!(hdop, f32, 7);
    make_number_field!(antenna_altitude, f32, 8);
    make_number_field!(geoidal_separation, f32, 9);
    make_number_field!(age_differential_gps_data, f32, 10);
    make_number_field!(differential_station_id, u16, 11);
}

impl GrsData {
    make_time_field!(time, 0);
    make_number_field!(residual_calculation, u8, 1);
    make_number_field!(satellite_1_residual, u8, 2);
    make_number_field!(satellite_2_residual, u8, 3);
    make_number_field!(satellite_3_residual, u8, 4);
    make_number_field!(satellite_4_residual, u8, 5);
    make_number_field!(satellite_5_residual, u8, 6);
    make_number_field!(satellite_6_residual, u8, 7);
    make_number_field!(satellite_7_residual, u8, 8);
    make_number_field!(satellite_8_residual, u8, 9);
    make_number_field!(satellite_9_residual, u8, 10);
    make_number_field!(satellite_10_residual, u8, 11);
    make_number_field!(satellite_11_residual, u8, 12);
    make_number_field!(satellite_12_residual, u8, 13);
    make_number_field!(system_id_1, u8, 14);
    make_number_field!(system_id_2, u8, 15);
}

impl GsaData {
    make_char_field!(selection_mode, 0);
    make_number_field!(mode, u8, 1);
    make_number_field!(satellite_id_1, u8, 2);
    make_number_field!(satellite_id_2, u8, 3);
    make_number_field!(satellite_id_3, u8, 4);
    make_number_field!(satellite_id_4, u8, 5);
    make_number_field!(satellite_id_5, u8, 6);
    make_number_field!(satellite_id_6, u8, 7);
    make_number_field!(satellite_id_7, u8, 8);
    make_number_field!(satellite_id_8, u8, 9);
    make_number_field!(satellite_id_9, u8, 10);
    make_number_field!(satellite_id_10, u8, 11);
    make_number_field!(satellite_id_11, u8, 12);
    make_number_field!(satellite_id_12, u8, 13);
    make_number_field!(pdop, f32, 14);
    make_number_field!(hdop, f32, 15);
    make_number_field!(vdop, f32, 16);
    make_number_field!(system_id, u8, 17);
}

impl GstData {
    make_time_field!(time, 0);
    make_number_field!(total_rms_standard_deviation, f32, 1);
    make_number_field!(semi_major_error_standard_deviation, f32, 2);
    make_number_field!(semi_minor_errpr_standard_deviation, f32, 3);
    make_number_field!(semi_major_orientation, f32, 4);
    make_number_field!(latitude_error_standard_deviation, f32, 5);
    make_number_field!(longitude_error_standard_deviation, f32, 6);
    make_number_field!(altitude_error_standard_deviation, f32, 7);
}

pub struct SatelliteInfo {
    satellite_id : u16,
    elevation: f32,
    azimuth : f32,
    snr : f32,
}

impl GsvData {
    make_number_field!(total_number_of_sentences, u8, 0);
    make_number_field!(sentence_number, u8, 1);
    make_number_field!(total_number_of_satellites_in_view, u16, 2);

    pub fn satellite_info(&self) -> Option<Vec<SatelliteInfo>> {
        let mut returned_vec = Vec::new();
        for i in (3..self.base.nfields()).step_by(4) {
            if let Some(satellite_id) = self.base.get::<u16>(i) {
                if let Some(elevation) = self.base.get::<f32>(i+1) {
                    if let Some(azimuth) = self.base.get::<f32>(i+2) {
                        if let Some(snr) = self.base.get::<f32>(i+3) {
                            returned_vec.push(SatelliteInfo {
                                satellite_id,
                                elevation,
                                azimuth,
                                snr
                            });
                        }
                    }
                }
            }
        }
        if returned_vec.is_empty() {
            None
        } else {
            Some(returned_vec)
        }
    }
}

impl GtdData {
    pub fn time_differences(&self) -> Option<Vec<f32>> {
        let mut differences = Vec::new();
        for i in 0..self.nfields() {
            if let Some(n) = self.base.get::<f32>(i) {
                differences.push(n);
            }
        }
        if !differences.is_empty() {
            return Some(differences);
        };
        None
    }
}

impl GxaData {
    make_time_field!(time, 0);
    make_number_field!(latitude, f32, 1, 2);
    make_number_field!(longitude, f32, 3, 4);
    make_string_field!(waypoint_id, 5);
    make_number_field!(satellite_id, u16, 6);
}

impl HdgData {
    make_number_field!(sensor_heading_degrees, f32, 0);
    make_number_field!(deviation, f32, 1, 2);
    make_number_field!(variation, f32, 3, 4);
}

impl HdmData {
    make_number_field!(heading_magnetic, f32, 0, 1);
}

impl HdtData {
    make_number_field!(heading_true, f32, 0, 1);
}

impl HfbData {
    make_number_field!(distance_headrope_to_footrope, f32, 0, 1);
    make_number_field!(distance_headrope_to_bottom, f32, 2, 3);
}

impl HscData {
    make_number_field!(heading_true, f32, 0, 1);
    make_number_field!(heading_magnetic, f32, 2, 3);
}

impl ItsData {
    make_number_field!(second_spread_distance, f32, 0, 1);
}

impl LcdData {
    make_number_field!(gri_microseconds, u16, 0);
    make_number_field!(master_relative_snr, u16, 1);
    make_number_field!(master_relative_ecd, u16, 2);
    make_number_field!(time_difference_1_microseconds, u16, 3);
    make_number_field!(time_difference_1_status, u16, 4);
    make_number_field!(time_difference_2_microseconds, u16, 5);
    make_number_field!(time_difference_2_status, u16, 6);
    make_number_field!(time_difference_3_microseconds, u16, 7);
    make_number_field!(time_difference_3_status, u16, 8);
    make_number_field!(time_difference_4_microseconds, u16, 9);
    make_number_field!(time_difference_4_status, u16, 10);
    make_number_field!(time_difference_5_microseconds, u16, 11);
    make_number_field!(time_difference_5_status, u16, 12);

    pub fn time_differences(&self) -> Option<Vec<(f32, f32)>> {
        let mut differences = Vec::new();
        for i in (3..self.nfields()).step_by(2) {
            if let Some(micros) = self.base.get::<f32>(i) {
                if let Some(status) = self.base.get::<f32>(i+1) {
                    differences.push((micros, status));
                }
            }
        };
        if differences.is_empty() { None }
        else { Some(differences) }
    }
}

impl MdaData {
    make_number_field!(barometric_pressure_mercury, f32, 0, 1);
    make_number_field!(barometric_pressure_bars, f32, 2, 3);
    make_number_field!(air_temperature_c, f32, 4, 5);
    make_number_field!(water_temperature_c, f32, 6, 7);
    make_number_field!(relative_humidity, f32, 8);
    make_number_field!(absolute_humidity, f32, 9);
    make_number_field!(dew_point_c, f32, 10, 11);
    make_number_field!(wind_direction_true, f32, 12, 13);
    make_number_field!(wind_direction_magnetic, f32, 14, 15);
    make_number_field!(wind_speed_knots, f32, 16, 17);
    make_number_field!(wind_speed_mps, f32, 18, 19);
}

impl MskData {
    make_number_field!(beacon_frequency, f32, 0);
    make_char_field!(frequency_mode, 1);
    make_number_field!(beacon_bit_rate, u8, 2);
    make_char_field!(bitrate_mode, 3);
    make_number_field!(interval_for_mss_message, u16, 4);
}

impl MssData {
    make_number_field!(signal_strength, u8, 0);
    make_number_field!(snr, f32, 1);
    make_number_field!(beacon_frequency, f32, 2);
    make_number_field!(beacon_bit_rate, u8, 3);
    make_number_field!(channel_number, u16, 4);
}

impl MtwData {
    make_number_field!(temperature, f32, 0, 1);
}

impl MwvData {
    make_number_field!(wind_angle, f32, 0, 1);
    make_number_field!(wind_speed, f32, 2, 3);
    make_char_field!(status, 4);
}

pub struct OmegaData {
    name : String,
    first : f32,
    second : f32,
}

impl OlnData {
    make_string_field!(omega1_name, 0);
    make_number_field!(omega1_first, f32, 1);
    make_number_field!(omega1_second, f32, 2);
    make_string_field!(omega2_name, 3);
    make_number_field!(omega2_first, f32, 4);
    make_number_field!(omega2_second, f32, 5);
    make_string_field!(omega3_name, 6);
    make_number_field!(omega3_first, f32, 7);
    make_number_field!(omega3_second, f32, 8);

    pub fn omegas(&self) -> Option<Vec<OmegaData>> {
        let mut returned_vec = Vec::new();
        for i in (0..self.base.nfields()).step_by(3) {
            if let Some(name) = self.base.get::<String>(i) {
                if let Some(first) = self.base.get::<f32>(i+1) {
                    if let Some(second) = self.base.get::<f32>(i+2) {
                        returned_vec.push(OmegaData {
                            name,
                            first,
                            second,
                        });
                    }
                }
            }
        };

        if returned_vec.is_empty() {
            None
        } else {
            Some(returned_vec)
        }
    }
}

impl OsdData {
    make_number_field!(heading_true, f32, 0);
    make_char_field!(heading_true_value, 1);
    make_number_field!(course_true, f32, 2);
    make_char_field!(course_reference, 3);
    make_number_field!(speed, f32, 4, 8);
    make_char_field!(speed_reference, 5);
    make_number_field!(set_true, f32, 6);
    make_number_field!(drift, f32, 7, 8);
}

impl R00Data {
    pub fn waypoints(&self) -> Option<Vec<String>> {
        let mut returned_vec = Vec::new();
        for i in 0..self.nfields() {
            if let Some(w) = self.base.get::<String>(i) {
                returned_vec.push(w);
            }
        };
        if returned_vec.is_empty() {
            None
        } else {
            Some(returned_vec)
        }
    }
}

impl RlmData {
    make_string_field!(beacon_id, 0);
    make_time_field!(time, 1);
    make_hex_field!(message_code, u8, 2);
    make_string_field!(message_body, 3);
}

impl RmbData {
    make_char_field!(status, 0);
    make_number_field!(xte, f32, 1);
    make_char_field!(direction_to_steer, 2);
    make_string_field!(origin_waypoint_id, 3);
    make_string_field!(destination_waypoint_id, 4);
    make_number_field!(destination_latitude, f32, 5, 6);
    make_number_field!(destination_longitude, f32, 7, 8);
    make_number_field!(destination_range, f32, 9);
    make_number_field!(destination_bearing, f32, 10);
    make_number_field!(vmg_knots, f32, 11);
    make_char_field!(arrival_status, 12);
    make_number_field!(mode, u8, 13);
}

impl RmcData {
    make_time_field!(time, 0);
    make_char_field!(status, 1);
    make_number_field!(latitude, f32, 2, 3);
    make_number_field!(longitude, f32, 4, 5);
    make_number_field!(sog_knots, f32, 6);
    make_number_field!(track_made_good_true, f32, 7);
    make_date_field!(date, 8);
    make_number_field!(variation, f32, 9, 10);
    make_number_field!(mode, u16, 11);
    make_char_field!(nav_status, 12);
}

impl RotData {
    make_number_field!(rate, f32, 0);
    make_char_field!(valid, 1);
}

impl RpmData {
    make_char_field!(source, 0);
    make_number_field!(source_number, u8, 1);
    make_number_field!(rpms, f32, 2);
    make_number_field!(propeller_pitch, f32, 3);
    make_char_field!(status, 4);
}

impl RsaData {
    make_number_field!(angle, f32, 0); // Synonym for starboard rudder.
    make_number_field!(starboard_angle, f32, 0);
    make_char_field!(status, 1);        // Synonym for starboard rudder
    make_char_field!(starboard_status, 1);
    make_number_field!(port_angle, f32, 2);
    make_char_field!(port_status, 2);
}

impl RsdData {
    make_number_field!(origin_1_range, f32, 0);
    make_number_field!(origin_1_bearing, f32, 1);
    make_number_field!(vrm_1, f32, 2);
    make_number_field!(bearing_line_1, f32, 3);
    make_number_field!(origin_2_range, f32, 4);
    make_number_field!(origin_2_bearing, f32, 5);
    make_number_field!(vrm_3, f32, 6);
    make_number_field!(bearing_line_2, f32, 7);
    make_number_field!(cursor_range, f32, 8);
    make_number_field!(cursor_bearing, f32, 9);
    make_number_field!(range_scale, f32, 10, 11);
    make_char_field!(display_rotation, 12);
}

impl RteData {
    make_number_field!(total_sentence_count, u8, 0);
    make_number_field!(sentence_number, u8, 1);
    make_char_field!(sentence_mode, 2);
    make_string_field!(route_name, 3);

    pub fn waypoints(&self) -> Option<Vec<String>> {
        let mut returned_vec = Vec::new();
        for i in 4..self.nfields() {
            if let Some(w) = self.base.get::<String>(i) {
                returned_vec.push(w)
            }
        };
        if returned_vec.is_empty() {
            None
        } else {
            Some(returned_vec)
        }
    }
}

impl Sf1Data {
    make_number_field!(total_sentence_count, u8, 0);
    make_number_field!(sentence_number, u8, 1);
    pub fn frequencies (&self) -> Option<Vec<(f32, char)>> {
        let mut returned_vec = Vec::new();
        for i in (2..self.nfields()).step_by(2) {
            if let Some(f) = self.base.get::<f32>(i) {
                if let Some(mode) = self.base.get::<char>(i+1) {
                    returned_vec.push((f, mode));
                }
            }
        };
        if returned_vec.is_empty() {
            None
        } else {
            Some(returned_vec)
        }
    }
}

impl StnData {
    make_number_field!(talker_id, u8, 0);
}

impl TdsData {
    make_number_field!(distance_between_doors, f32, 0, 1);
}

impl TfiData {
    make_char_field!(sensor_1, 0);
    make_char_field!(sensor_2, 0);
    make_char_field!(sensor_3, 0);
}

impl TlbData {
    pub fn targets(&self) -> Option<Vec<(u8, String)>> {
        let mut returned_vec = Vec::new();
        for i in (0..self.nfields()).step_by(2) {
            if let Some(n) = self.base.get::<u8>(i) {
                if let Some(l) = self.base.get::<String>(i+1) {
                    returned_vec.push((n, l));
                }
            }
        };

        if returned_vec.is_empty() { None }
        else {Some(returned_vec) }
    }
}

impl TllData {
    make_number_field!(target_number, u8, 0);
    make_number_field!(latitude, f32, 1, 2);
    make_number_field!(longitude, f32, 3, 4);
    make_string_field!(name, 5);
    make_time_field!(time, 6);
    make_char_field!(status, 7);
    make_char_field!(reference, 8);
}

impl TpcData {
    make_number_field!(horizontal_distance_from_centerline, f32, 0, 1);
    make_number_field!(horizontal_distance_from_transducer, f32, 2, 3);
    make_number_field!(depth, f32, 4, 5);
}

impl TprData {
    make_number_field!(horizontal_range, f32, 0, 1);
    make_number_field!(bearing_to_target, u16, 2);
    make_char_field!(separator, 3);
    make_number_field!(depth, f32, 4, 5);
}

impl TptData {
    make_number_field!(horizontal_range, f32, 0, 1);
    make_number_field!(bearing_true, u16, 2);
    make_char_field!(separator, 3);
    make_number_field!(depth, f32, 4, 5);
}

impl TrfData {
    make_time_field!(time, 0);
    make_date_field!(date, 1);
    make_number_field!(latitude, f32, 2, 3);
    make_number_field!(longitude, f32, 4, 5);
    make_number_field!(elevation_angle, f32, 6);
    make_number_field!(iteration_count, u8, 7);
    make_number_field!(dopper_count, u8, 8);
    make_number_field!(update_distance, f32, 9);
    make_number_field!(satellite_id, u16, 10);
    make_char_field!(data_validity, 11);
}

impl TtmData {
    make_number_field!(target_number, u8, 0);
    make_number_field!(target_distance, f32, 1, 9);
    make_number_field!(target_bearing, f32, 2, 3);
    make_number_field!(target_speed, f32, 4, 9);
    make_number_field!(target_course, f32, 5, 6);
    make_number_field!(cpa, f32, 7);
    make_number_field!(tpa, f32, 8);
    make_string_field!(name, 10);
    make_char_field!(status, 11);
    make_char_field!(reference, 12);
    make_time_field!(time, 13);
    make_char_field!(report_type, 14);
}

impl VbwData {
    make_number_field!(longitudinal_water_speed, f32, 0);
    make_number_field!(transverse_water_speed, f32, 1);
    make_char_field!(status_water_speed, 2);
    make_number_field!(longitudinal_ground_speed, f32, 3);
    make_number_field!(transverse_ground_speed, f32, 4);
    make_char_field!(status_ground_speed, 5);
    make_number_field!(stern_traverse_water_speed_knots,f32, 6);
    make_char_field!(status_stern_traverse_water_speed, 7);
    make_number_field!(stern_traverse_ground_speed_knots,f32, 8);
    make_char_field!(status_stern_traverse_ground_speed, 9);
}

impl VdrData {
    make_number_field!(direction_true, f32, 0, 1);
    make_number_field!(direction_magnetic, f32, 2, 3);
    make_number_field!(current_speed, f32, 4, 5);
}

impl VhwData {
    make_number_field!(heading_true, f32, 0, 1);
    make_number_field!(heading_magnetic, f32, 2, 3);
    make_number_field!(stw_knots, f32, 4, 5);
    make_number_field!(stw_kph, f32, 6, 7);
}

impl VlwData {
    make_number_field!(water_distance_total, f32, 0, 1);
    make_number_field!(water_distance_since_reset, f32, 2, 3);
    make_number_field!(ground_distance_total, f32, 4, 5);
    make_number_field!(ground_distance_since_reset, f32, 6, 7);
}

impl VpwData {
    make_number_field!(speed_knots, f32, 0, 1);
    make_number_field!(speed_mps, f32, 1, 2);
}

impl VtgData {
    make_number_field!(cog_true, f32, 0, 1);
    make_number_field!(cog_magnetic, f32, 2, 3);
    make_number_field!(sog_knots, f32, 4, 5);
    make_number_field!(sog_kph, f32, 6, 7);
    make_char_field!(mode, 8);
}

impl VwrData {
    make_number_field!(wind_direction, f32, 0, 1);
    make_number_field!(speed_knots, f32, 2, 3);
    make_number_field!(speed_mps, f32, 4, 5);
    make_number_field!(speed_kph, f32, 6, 7);
}

impl WcvData {
    make_number_field!(velocity_knot, f32, 0, 1);
    make_string_field!(waypoint_id, 2);
    make_char_field!(mode, 3);
}

impl WncData {
    make_number_field!(distance_nm, f32, 0, 1);
    make_number_field!(distance_km, f32, 2, 3);
    make_string_field!(destination_waypoint_id, 4);
    make_string_field!(origin_waypoint_id, 5);
}

impl WplData {
    make_number_field!(latitude, f32, 0, 1);
    make_number_field!(longitude, f32, 2, 3);
    make_string_field!(waypoint_id, 4);
}

pub struct TransducerData {
    pub transducer_type : char,
    pub data : ValueWithUnit<f32>,
    pub name : String,
}

impl XdrData {
    pub fn measurements(&self) -> Option<Vec<TransducerData>> {
        let mut returned_vec = Vec::new();
        for i in (0..self.nfields()).step_by(4) {
            if let Some(transducer_type) = self.base.get::<char>(i) {
                if let Some(value) = self.base.get::<f32>(i+1) {
                    if let Some(unit) = self.base.get::<char>(i+2) {
                        if let Some(name) = self.base.get::<String>(i+3) {
                            returned_vec.push(TransducerData{
                                transducer_type,
                                data: ValueWithUnit {value, unit},
                                name,
                            })
                        }
                    }
                }
            }
        };

        if returned_vec.is_empty() { None }
        else { Some(returned_vec) }
    }
}

impl XteData {
    make_char_field!(status, 0);
    make_char_field!(loran_status, 1);
    make_number_field!(xte, f32, 2, 4);
    make_char_field!(direction_to_steer, 3);
    make_char_field!(mode, 5);
}

impl XtrData {
    make_number_field!(xte, f32, 0, 2);
    make_char_field!(direction_to_steer, 1);
}

impl ZdaData {
    make_time_field!(time, 0);
    make_number_field!(day, u32, 1);
    make_number_field!(month, u32, 2);
    make_number_field!(year, i32, 3);
    make_number_field!(local_zone_description, i8, 4);
    make_number_field!(local_zone_minutes, i8, 5);
}

impl ZfoData {
    make_time_field!(time, 0);
    make_time_field!(elapsed_time, 1);
    make_string_field!(origin_waypoint_id, 2);
}

impl ZtgData {
    make_time_field!(time, 0);
    make_time_field!(time_remaining, 1);
    make_string_field!(destination_waypoint_id, 2);
}

pub enum NmeaSentence {
    AAM(AamData),
    ABK(AbkData),
    ACA(AcaData),
    ACK(AckData),
    ACS(AcsData),
    ADS(AdsData),
    AIR(AirData),
    AKD(AkdData),
    ALA(AlaData),
    ALM(AlmData),
    ALR(AlrData),
    APA(ApaData),
    APB(ApbData),
    ASD(AsdData),
    BEC(BecData),
    BOD(BodData),
    BWC(BwcData),
    BWR(BwrData),
    BWW(BwwData),
    CEK(CekData),
    COP(CopData),
    CUR(CurData),
    DBK(DbkData),
    DBS(DbsData),
    DBT(DbtData),
    DCN(DcnData),
    DCR(DcrData),
    DDC(DdcData),
    DOR(DorData),
    DPT(DptData),
    DSC(DscData),
    DSE(DseData),
    DSI(DsiData),
    DSR(DsrData),
    DTM(DtmData),
    ETL(EtlData),
    EVE(EveData),
    FIR(FirData),
    FSI(FsiData),
    GBS(GbsData),
    GGA(GgaData),
    GLC(GlcData),
    GLL(GllData),
    GMP(GmpData),
    GNS(GnsData),
    GRS(GrsData),
    GSA(GsaData),
    GST(GstData),
    GSV(GsvData),
    GTD(GtdData),
    GXA(GxaData),
    HDG(HdgData),
    HDM(HdmData),
    HDT(HdtData),
    HFB(HfbData),
    HMR(HmrData),
    HMS(HmsData),
    HSC(HscData),
    HTC(HtcData),
    HTD(HtdData),
    ITS(ItsData),
    LCD(LcdData),
    LRF(LrfData),
    LR1(Lr1Data),
    LR2(Lr2Data),
    LR3(Lr3Data),
    MDA(MdaData),
    MLA(MlaData),
    MSK(MskData),
    MSS(MssData),
    MTW(MtwData),
    MWD(MwdData),
    MWV(MwvData),
    OLN(OlnData),
    OSD(OsdData),
    R00(R00Data),
    RLM(RlmData),
    RMA(RmaData),
    RMB(RmbData),
    RMC(RmcData),
    ROT(RotData),
    RPM(RpmData),
    RSA(RsaData),
    RSD(RsdData),
    RTE(RteData),
    SF1(Sf1Data),
    SSD(SsdData),
    STN(StnData),
    TDS(TdsData),
    TFI(TfiData),
    TLB(TlbData),
    TLL(TllData),
    TPC(TpcData),
    TPR(TprData),
    TPT(TptData),
    TRF(TrfData),
    TTM(TtmData),
    TUT(TutData),
    TXT(TxtData),
    VBW(VbwData),
    VDR(VdrData),
    VHW(VhwData),
    VLW(VlwData),
    VPW(VpwData),
    VSD(VsdData),
    VTG(VtgData),
    VWR(VwrData),
    WCV(WcvData),
    WDC(WdcData),
    WDR(WdrData),
    WNC(WncData),
    WPL(WplData),
    XDR(XdrData),
    XTE(XteData),
    XTR(XtrData),
    ZDA(ZdaData),
    ZDL(ZdlData),
    ZFO(ZfoData),
    ZTG(ZtgData),
    ABM(AbmData),
    BBM(BbmData),
    VDM(VdmData),
    VDO(VdoData),
    ERROR(ErrorData),
}

impl From<&NmeaBaseSentence> for NmeaSentence {
    fn from(value: &NmeaBaseSentence) -> Self {
        match value._message_type.as_str() {
            "AAM" => AAM(AamData { base: (*value).clone() }),
            "ABK" => ABK(AbkData { base: (*value).clone() }),
            "ACA" => ACA(AcaData { base: (*value).clone() }),
            "ACK" => ACK(AckData { base: (*value).clone() }),
            "ACS" => ACS(AcsData { base: (*value).clone() }),
            "ADS" => ADS(AdsData { base: (*value).clone() }),
            "AIR" => AIR(AirData { base: (*value).clone() }),
            "AKD" => AKD(AkdData { base: (*value).clone() }),
            "ALA" => ALA(AlaData { base: (*value).clone() }),
            "ALM" => ALM(AlmData { base: (*value).clone() }),
            "ALR" => ALR(AlrData { base: (*value).clone() }),
            "APA" => APA(ApaData { base: (*value).clone() }),
            "APB" => APB(ApbData { base: (*value).clone() }),
            "ASD" => ASD(AsdData { base: (*value).clone() }),
            "BEC" => BEC(BecData { base: (*value).clone() }),
            "BOD" => BOD(BodData { base: (*value).clone() }),
            "BWC" => BWC(BwcData { base: (*value).clone() }),
            "BWR" => BWR(BwrData { base: (*value).clone() }),
            "BWW" => BWW(BwwData { base: (*value).clone() }),
            "CEK" => CEK(CekData { base: (*value).clone() }),
            "COP" => COP(CopData { base: (*value).clone() }),
            "CUR" => CUR(CurData { base: (*value).clone() }),
            "DBK" => DBK(DbkData { base: (*value).clone() }),
            "DBS" => DBS(DbsData { base: (*value).clone() }),
            "DBT" => DBT(DbtData { base: (*value).clone() }),
            "DCN" => DCN(DcnData { base: (*value).clone() }),
            "DCR" => DCR(DcrData { base: (*value).clone() }),
            "DDC" => DDC(DdcData { base: (*value).clone() }),
            "DOR" => DOR(DorData { base: (*value).clone() }),
            "DPT" => DPT(DptData { base: (*value).clone() }),
            "DSC" => DSC(DscData { base: (*value).clone() }),
            "DSE" => DSE(DseData { base: (*value).clone() }),
            "DSI" => DSI(DsiData { base: (*value).clone() }),
            "DSR" => DSR(DsrData { base: (*value).clone() }),
            "DTM" => DTM(DtmData { base: (*value).clone() }),
            "ETL" => ETL(EtlData { base: (*value).clone() }),
            "EVE" => EVE(EveData { base: (*value).clone() }),
            "FIR" => FIR(FirData { base: (*value).clone() }),
            "FSI" => FSI(FsiData { base: (*value).clone() }),
            "GBS" => GBS(GbsData { base: (*value).clone() }),
            "GGA" => GGA(GgaData { base: (*value).clone() }),
            "GLC" => GLC(GlcData { base: (*value).clone() }),
            "GLL" => GLL(GllData { base: (*value).clone() }),
            "GMP" => GMP(GmpData { base: (*value).clone() }),
            "GNS" => GNS(GnsData { base: (*value).clone() }),
            "GRS" => GRS(GrsData { base: (*value).clone() }),
            "GSA" => GSA(GsaData { base: (*value).clone() }),
            "GST" => GST(GstData { base: (*value).clone() }),
            "GSV" => GSV(GsvData { base: (*value).clone() }),
            "GTD" => GTD(GtdData { base: (*value).clone() }),
            "GXA" => GXA(GxaData { base: (*value).clone() }),
            "HDG" => HDG(HdgData { base: (*value).clone() }),
            "HDM" => HDM(HdmData { base: (*value).clone() }),
            "HDT" => HDT(HdtData { base: (*value).clone() }),
            "HFB" => HFB(HfbData { base: (*value).clone() }),
            "HMR" => HMR(HmrData { base: (*value).clone() }),
            "HMS" => HMS(HmsData { base: (*value).clone() }),
            "HSC" => HSC(HscData { base: (*value).clone() }),
            "HTC" => HTC(HtcData { base: (*value).clone() }),
            "HTD" => HTD(HtdData { base: (*value).clone() }),
            "ITS" => ITS(ItsData { base: (*value).clone() }),
            "LCD" => LCD(LcdData { base: (*value).clone() }),
            "LRF" => LRF(LrfData { base: (*value).clone() }),
            "LR1" => LR1(Lr1Data { base: (*value).clone() }),
            "LR2" => LR2(Lr2Data { base: (*value).clone() }),
            "LR3" => LR3(Lr3Data { base: (*value).clone() }),
            "MDA" => MDA(MdaData { base: (*value).clone() }),
            "MLA" => MLA(MlaData { base: (*value).clone() }),
            "MSK" => MSK(MskData { base: (*value).clone() }),
            "MSS" => MSS(MssData { base: (*value).clone() }),
            "MTW" => MTW(MtwData { base: (*value).clone() }),
            "MWD" => MWD(MwdData { base: (*value).clone() }),
            "MWV" => MWV(MwvData { base: (*value).clone() }),
            "OLN" => OLN(OlnData { base: (*value).clone() }),
            "OSD" => OSD(OsdData { base: (*value).clone() }),
            "R00" => R00(R00Data { base: (*value).clone() }),
            "RLM" => RLM(RlmData { base: (*value).clone() }),
            "RMA" => RMA(RmaData { base: (*value).clone() }),
            "RMB" => RMB(RmbData { base: (*value).clone() }),
            "RMC" => RMC(RmcData { base: (*value).clone() }),
            "ROT" => ROT(RotData { base: (*value).clone() }),
            "RPM" => RPM(RpmData { base: (*value).clone() }),
            "RSA" => RSA(RsaData { base: (*value).clone() }),
            "RSD" => RSD(RsdData { base: (*value).clone() }),
            "RTE" => RTE(RteData { base: (*value).clone() }),
            "SF1" => SF1(Sf1Data { base: (*value).clone() }),
            "SSD" => SSD(SsdData { base: (*value).clone() }),
            "STN" => STN(StnData { base: (*value).clone() }),
            "TDS" => TDS(TdsData { base: (*value).clone() }),
            "TFI" => TFI(TfiData { base: (*value).clone() }),
            "TLB" => TLB(TlbData { base: (*value).clone() }),
            "TLL" => TLL(TllData { base: (*value).clone() }),
            "TPC" => TPC(TpcData { base: (*value).clone() }),
            "TPR" => TPR(TprData { base: (*value).clone() }),
            "TPT" => TPT(TptData { base: (*value).clone() }),
            "TRF" => TRF(TrfData { base: (*value).clone() }),
            "TTM" => TTM(TtmData { base: (*value).clone() }),
            "TUT" => TUT(TutData { base: (*value).clone() }),
            "TXT" => TXT(TxtData { base: (*value).clone() }),
            "VBW" => VBW(VbwData { base: (*value).clone() }),
            "VDR" => VDR(VdrData { base: (*value).clone() }),
            "VHW" => VHW(VhwData { base: (*value).clone() }),
            "VLW" => VLW(VlwData { base: (*value).clone() }),
            "VPW" => VPW(VpwData { base: (*value).clone() }),
            "VSD" => VSD(VsdData { base: (*value).clone() }),
            "VTG" => VTG(VtgData { base: (*value).clone() }),
            "VWR" => VWR(VwrData { base: (*value).clone() }),
            "WCV" => WCV(WcvData { base: (*value).clone() }),
            "WDC" => WDC(WdcData { base: (*value).clone() }),
            "WDR" => WDR(WdrData { base: (*value).clone() }),
            "WNC" => WNC(WncData { base: (*value).clone() }),
            "WPL" => WPL(WplData { base: (*value).clone() }),
            "XDR" => XDR(XdrData { base: (*value).clone() }),
            "XTE" => XTE(XteData { base: (*value).clone() }),
            "XTR" => XTR(XtrData { base: (*value).clone() }),
            "ZDA" => ZDA(ZdaData { base: (*value).clone() }),
            "ZDL" => ZDL(ZdlData { base: (*value).clone() }),
            "ZFO" => ZFO(ZfoData { base: (*value).clone() }),
            "ZTG" => ZTG(ZtgData { base: (*value).clone() }),
            "ABM" => ABM(AbmData { base: (*value).clone() }),
            "BBM" => BBM(BbmData { base: (*value).clone() }),
            "VDM" => VDM(VdmData { base: (*value).clone() }),
            "VDO" => VDO(VdoData { base: (*value).clone() }),
            _ => NmeaSentence::ERROR(ErrorData {
                error: "Invalid or unknown message type".to_string(),
                message: value._original.clone()
            }),
        }
    }
}

impl From<&String> for NmeaSentence {
    fn from(value: &String) -> Self {
        let base = NmeaBaseSentence::from(value);
        NmeaSentence::from(&base)
    }
}

#[cfg(test)]
mod tests;
