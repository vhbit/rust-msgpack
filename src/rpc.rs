use super::{Value,Encoder,Decoder, _invalid_input};
use serialize;
use serialize::{Encodable,Decodable};
use std::io::{IoError, IoResult};


pub enum RpcMessage {
  Request      {msgid: u32, method: String, params: Vec<Value>}, // 0
  Response     {msgid: u32, error: Value, result: Value}, // 1
  Notification {method: String, params: Vec<Value>} // 2
}

impl<'a> serialize::Encodable<Encoder<'a>, IoError> for RpcMessage {
  fn encode(&self, s: &mut Encoder<'a>) -> IoResult<()> {
    match *self {
      RpcMessage::Request {msgid, ref method, ref params} => {
        (0u, msgid, method, params).encode(s)
      }
      RpcMessage::Response {msgid, ref error, ref result} => {
        (1u, msgid, error, result).encode(s)
      }
      RpcMessage::Notification {ref method, ref params} => {
        (2u, method, params).encode(s)
      }
    }
  }
}

impl<R: Reader> serialize::Decodable<Decoder<R>, IoError> for RpcMessage {
  fn decode(s: &mut Decoder<R>) -> IoResult<RpcMessage> {
    let len = try!(s._read_vec_len());
    let ty: uint = try!(Decodable::decode(s));

    match ty {
      0 => {
        if len != 4 { return Err(_invalid_input("Invalid msgpack-rpc message array length")) }
        let msgid = try!(Decodable::decode(s));
        let method = try!(Decodable::decode(s));
        let params = try!(Decodable::decode(s));
        Ok(RpcMessage::Request {msgid: msgid, method: method, params: params})
      }
      1 => {
        if len != 4 { return Err(_invalid_input("Invalid msgpack-rpc message array length")) }
        let msgid = try!(Decodable::decode(s));
        let error = try!(Decodable::decode(s));
        let result = try!(Decodable::decode(s));
        Ok(RpcMessage::Response {msgid: msgid, error: error, result: result})
      }
      2 => {
        if len != 3 { return Err(_invalid_input("Invalid msgpack-rpc message array length")) }
        let method = try!(Decodable::decode(s));
        let params = try!(Decodable::decode(s));
        Ok(RpcMessage::Notification {method: method, params: params})
      }
      _ => {
        Err(_invalid_input("Invalid msgpack-rpc message type"))
      }
    }

  }
}
