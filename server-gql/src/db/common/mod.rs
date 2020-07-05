// Propagate any error
use failure::Error;

use serde::{
  Deserialize, Serialize, Serializer //, Deserializer,
  de::DeserializeOwned, ser::SerializeStruct
};