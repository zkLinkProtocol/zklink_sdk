use crate::{TimeStampMicro, UniffiCustomTypeConverter};
use chrono::{TimeZone, Utc};

impl UniffiCustomTypeConverter for TimeStampMicro {
    type Builtin = u64;
    fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
        let date_time = Utc.timestamp_nanos(val as i64 * 1000);
        Ok(date_time)
    }
    fn from_custom(obj: Self) -> Self::Builtin {
        (obj.timestamp_nanos() / 1000) as u64
    }
}
