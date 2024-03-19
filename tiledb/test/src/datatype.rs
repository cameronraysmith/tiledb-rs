use proptest::prelude::*;

pub fn arbitrary() -> impl Strategy<Value = tiledb::Datatype> {
    prop_oneof![
        Just(tiledb::Datatype::Int8),
        Just(tiledb::Datatype::Int16),
        Just(tiledb::Datatype::Int32),
        Just(tiledb::Datatype::Int64),
        Just(tiledb::Datatype::UInt8),
        Just(tiledb::Datatype::UInt16),
        Just(tiledb::Datatype::UInt32),
        Just(tiledb::Datatype::UInt64),
        Just(tiledb::Datatype::Float32),
        Just(tiledb::Datatype::Float64),
        Just(tiledb::Datatype::Char),
        Just(tiledb::Datatype::StringAscii),
        Just(tiledb::Datatype::StringUtf8),
        Just(tiledb::Datatype::StringUtf16),
        Just(tiledb::Datatype::StringUtf32),
        Just(tiledb::Datatype::StringUcs2),
        Just(tiledb::Datatype::StringUcs4),
        Just(tiledb::Datatype::Any),
        Just(tiledb::Datatype::DateTimeYear),
        Just(tiledb::Datatype::DateTimeMonth),
        Just(tiledb::Datatype::DateTimeWeek),
        Just(tiledb::Datatype::DateTimeDay),
        Just(tiledb::Datatype::DateTimeHour),
        Just(tiledb::Datatype::DateTimeMinute),
        Just(tiledb::Datatype::DateTimeSecond),
        Just(tiledb::Datatype::DateTimeMillisecond),
        Just(tiledb::Datatype::DateTimeMicrosecond),
        Just(tiledb::Datatype::DateTimeNanosecond),
        Just(tiledb::Datatype::DateTimePicosecond),
        Just(tiledb::Datatype::DateTimeFemtosecond),
        Just(tiledb::Datatype::DateTimeAttosecond),
        Just(tiledb::Datatype::TimeHour),
        Just(tiledb::Datatype::TimeMinute),
        Just(tiledb::Datatype::TimeSecond),
        Just(tiledb::Datatype::TimeMillisecond),
        Just(tiledb::Datatype::TimeMicrosecond),
        Just(tiledb::Datatype::TimeNanosecond),
        Just(tiledb::Datatype::TimePicosecond),
        Just(tiledb::Datatype::TimeFemtosecond),
        Just(tiledb::Datatype::TimeAttosecond),
        Just(tiledb::Datatype::Blob),
        Just(tiledb::Datatype::Boolean),
        Just(tiledb::Datatype::GeometryWkb),
        Just(tiledb::Datatype::GeometryWkt),
    ]
}
