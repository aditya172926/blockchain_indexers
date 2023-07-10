use ethers::abi::ParamType;


#[derive(Debug)]
pub struct MethodParam<'a> {
    pub name: &'a String,
    pub kind: &'a ParamType,
    pub internal_type: &'a std::option::Option<std::string::String>,
    pub value: ethers::abi::Token
}