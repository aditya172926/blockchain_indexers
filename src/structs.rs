use ethers::abi::ParamType;
use serde::Serialize;


#[derive(Serialize)]
pub struct MethodParam<'a> {
    pub name: &'a String,
    pub kind: &'a ParamType,
    pub internal_type: &'a std::option::Option<std::string::String>,
    pub value: ethers::abi::Token
}