use ethers::abi::{ParamType, Token};
use serde::Serialize;


#[derive(Serialize, Debug)]
pub struct MethodParam<'a> {
    pub name: &'a String,
    pub kind: String,
    pub internal_type: &'a std::option::Option<std::string::String>,
    pub value: Token
}