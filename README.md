# nmea
NMEA 183 parser by a neophyte Rust programmer

There are two levels of abstraction here.  The first is the NmeaBaseSentence.  This is useful to work with if the sentence you are working for is not yet implemented in the system.  It will parse a string and break it into the various pieces that are useful to you.

The second level of abstraction is ways to access pieces in a more understandable way.  Rather than hard coding to particular field numbers, you can access the semantic fields.  By this, imagine: `$XXZZZ,102.3,103.4,C*80`.  you could get the fields with:
```
let sentence_string = "$XXZZZ,102.3,103.4,C*80";
let base_sentence = NmeaBaseSentence::from(&sentence_string);
let air_temperature = base_sentence.get::<f32>(0);
if (base_sentence.nfields() >= 1) {
  println!("Air temperature is {}", base_sentence.get::<f32>(1));
}
```
or you can do:
```
let sentence_string = "$XXZZZ,102.3,103.4,C*80";
let zzz_data = ZzzData::from(&sentence_string);
if let Some(p) = zzz_data.air_temperature() { println!("Air temperature is {}", p.value()); }
```

Note that access to values is through getters.  You do not access the fields of the structure directly!

## NmeaBaseSentence
The main workhorse is NmeaBaseSentence.  This is what parses the string into the sender, the message type, the checksum, and the fields.  Example code is:

let base_sentence = NmeaBaseSentence::from(&my_nmea_sentence_string); 

There are methods to access the information:

+ `NmeaBaseSentence::new()` -- create an empty NmeaBaseSentence, perhaps for filling out yourself
+ `NmeaBaseSentence::default()` -- Synonym for NmeaBaseSentence::new
+ `NmeaBaseSentence::sender(&self)` -- get the sender of the message.  This includes the introductory "$" or "!"
+ `NmeaBaseSentence::message_type(&self)` -- Get the message type. These are the last three characters of the base sentence.
+ `NmeaBaseSentence::checksum(&self)` -- Get the message checksum.
+ `NmeaBaseSentence::get<T>(&self, index: usize) -> Option<T>` -- gets the indicated field as type "T".  Supported types are u8, u16, u32, i8, i16, i32, usize, isize, f32, f64, char, string.
  + Note that NmeaBaseSentence has no idea of what the "type" of a field is.  Internally, they are stored as strings.  It's up to you! Example Code:
  ```
  let my_base_sentence = NmeaBaseSentence::from(&my_nmea_sentence_string);
  if let Some(my_field_value) = my_base_sentence.get::<u8>(1) {..........}
  ```
  
+ `NmeaBaseSentence::get_hex<T>(&self, index: usize) -> Option<T>` -- the same as NmeaBaseSentence.get<> except it assumed the field is hex encoded 
+ `NmeaBaseSentence::get_pair<T>(&self, index: usize, unit_index: usize) -> Option<ValueWithUnit<T>` -- Same as with NmeaBaseSentence::get except that it allowes you
  to specify a field that holds the character indicator for the units.  For example, a heading might be ...,130.5,M,....  You can get the value (130.5) and the unit
  (M) together with `NmeaBaseSentence::get_pair::<f32>(4, 5)`.
+ `NmeaBaseSentence::get_time(&self, index: usize) -> Option<NaiveTime>` -- converts the field to a time.  field must be in hhmmss.ssss format.
+ `NmeaBaseSentence::get_coordinate(&self, index:usize, index_direction:usize)` -> Option<f32> -- For those people that like their latitude/longitude to be negative when
  South or West and positive with North or East.
+ `NmeaBaseSentence::get_date(&self, index: usize) -> Option<NaiveDate>` -- Converts a field to a date.  Field must be in ddmmyy format.

## Higher-Levels of abstraction.
  
Making higher levels of abstraction is important.  This, there are abstractions such as `AamData`, `GsvData`, etc. that are built on top of NmeaBaseSentence.  To assist in making these, there are a couple of macros created.

### make_data! macro

The first step is identifying a message that can be created.  For example, if you want to define the AAM message, you would use
  ```
  make_data!(Aam);
  ```
  
This will do three things:

  1. Create the AamData structure which has a single, nonpublic, member called "base" that is used to hold the NmeaBaseSentence.
  1. Associate the AamData with the NmeaBaseTrait trait
  1. Associate the AamData structure with the From<&String> trait and have it create an NmeaBaseSentence from the passed-in string reference.

Thus, automagically would allow you to do something like `let my_aam_structure = AamData::from<&my_nmea_sentence_string>`.
  
### make_..._field! macros

The next thing to do is create the accessors for the fields of the structure.  Something like AamData doesn't actually store the fields.  What it does is ask the underlying NmeaBaseSentence to feed them up, generally using the various `NmeaBaseSentence::get...` functions.  To make that easier for you, several macros have been created.  Thus, the `impl AamData` section would look something like:
  ```
  impl AamData {
    make_char_field!(arrival_status, 0);
    make_char_field!(perpendicular_status, 1);
    make_number_field!(arrival_circle_radius, f32, 2);
    make_string_field!(waypoint_id, 3);
  }
  ```
  
  And now your AamData is up and running!
  
  #### make_char_field!(name, index);
  The specified field consists of one character (or, at least, the first character)
  
  #### make_number_field!(name, type, index);
  the specified field consists of a number of type "type".  type can be u8, u16, u32, u64, i8, i16, i32, i64, f32, f64, usize, or isize.
  In reality, you can also use char and string, but you're probably better off using the specialized versions of the macro.
  
  #### make_number_field!(name, type, index, index);
  a special version of the make_number_field macro that creates a ValueWithUnit rather than just a value.  For instance if you have something that looks like:
  `$YDBRG,102.3,M*80` in which it is showing a bearing that is magnetic (designated by the "M"), you can keep the value and unit together using
  `make_number_field!(bearing, 0, 1)`.  Note that the value and the unit do not have to be sequential in the fields.  For example: `$MYZZZ,102.3,104.7,C*00` might
  be two temperatures, both in degrees C.  The first one might be, say, air temperature, and the second might be exhaust temperature.  You can specify:
  ```
  impl ZzzData {
      make_number_field!(air_temperature, f32, 0, 2);
      make_number_field!(exhaust_temperature, f32, 1, 2);
  }
  ```
  Note that the unit field is used for both.
  
  #### make_time_field!(name, index); and make_date_field!(name, index);
  These are used to make NaiveTime and NaiveDate fields.
  
  ## Custom fields
  
  Sometimes you might want to do something that's out of the ordinary.  Let us, for example, say that there's a sentence that has latitude and longitude in it.  Rather than returning those with two separate accessors such as `my_sentence.latitude()` and `my_sentence.longitude()` you would like to bring them in as a combined latitude, longitude structure called `pub struct LatLon`.
  
  ```
  make_data!(MyData)
  
  pub struct LatLon {
      latitude: f32,
      longitude: f32,
  }
  
  impl MyData {
      pub fn position(&self) -> Option<LatLon> {
        // We are assuming that the latitude/longitude are in fields 3, 4, 5, and 6
        if let Some(latitude) = self.base.get::<f32>(3) {
          if let Some(latitude_indicator) = self.base.get::<char>(4) {
            if let Some(longitude) = self.base.get::<f32>(5) {
              if let Some(longitude_indicator) = self.base.get::<char>(6) {
                let latitude = if latitude_indicator == "S" { -latitude } else {latitude};
                let longitude = if longitude_indicator == "W" { -longitude } else {longitude};
                return Some(LatLon { latitude, longitude });
              }
            }
          }
        }
        None
      }
    }
  ```
