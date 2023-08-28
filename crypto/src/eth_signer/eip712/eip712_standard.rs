use ethers::types::H256;
use ethers::utils::keccak256;

use super::{AbsorbMember, MembersCollecter};

/// Abstract the EIP712.
pub trait EIP712 {
    const STRUCT_NAME: &'static str;

    fn absorb_member<BUILDER: AbsorbMember>(&self, builder: &mut BUILDER);

    fn encode_type(&self) -> String {
        let mut builder = MembersCollecter::default();
        self.absorb_member(&mut builder);

        builder.encode_type(Self::STRUCT_NAME)
    }

    fn type_hash(&self) -> H256 {
        let encode_type = self.encode_type();
        keccak256(encode_type).into()
    }

    // enc(value₁) ‖ enc(value₂) ‖ … ‖ enc(valueₙ)
    fn encode_data(&self) -> Vec<H256> {
        let mut builder = MembersCollecter::default();
        self.absorb_member(&mut builder);

        builder.encode_data()
    }

    fn hash_struct(&self) -> H256 {
        // hashStruct(s : 𝕊) = keccak256(keccak256(EncodeType(TypeOf(s))) ‖ EncodeData(s)).
        let type_hash = self.type_hash();
        let encode_data = self
            .encode_data()
            .into_iter()
            .flat_map(|e| e.to_fixed_bytes())
            .collect::<Vec<_>>();

        let mut eip712_encode = Vec::with_capacity(encode_data.len() + 1);
        eip712_encode.extend(type_hash.to_fixed_bytes());
        eip712_encode.extend(encode_data);
        keccak256(eip712_encode).into()
    }
}

pub trait Structuralization {
    const MEMBER_TYPE: &'static str;
    const IS_REFERENCE_TYPE: bool;

    fn member_type(&self) -> String {
        Self::MEMBER_TYPE.to_string()
    }

    fn sub_members(&self) -> Vec<EncodedMember> {
        Vec::new()
    }

    fn encode_member_data(&self) -> H256;
}

impl<TypedStructure: EIP712> Structuralization for TypedStructure {
    const MEMBER_TYPE: &'static str = Self::STRUCT_NAME;
    const IS_REFERENCE_TYPE: bool = true;

    fn sub_members(&self) -> Vec<EncodedMember> {
        let mut builder = MembersCollecter::default();
        self.absorb_member(&mut builder);
        builder.members
    }

    fn encode_member_data(&self) -> H256 {
        self.hash_struct()
    }
}

#[derive(Debug, Clone)]
pub struct EncodedMember {
    pub name: String,
    pub member_type: String,
    pub encode_value: H256,
    pub sub_members: Vec<EncodedMember>,
    pub is_reference_type: bool,
}

impl EncodedMember {
    pub fn build<MEMBER: Structuralization>(name: &str, member: &MEMBER) -> Self {
        Self {
            name: name.to_string(),
            member_type: member.member_type(),
            encode_value: member.encode_member_data(),
            is_reference_type: MEMBER::IS_REFERENCE_TYPE,
            sub_members: member.sub_members(),
        }
    }

    /// Encodes the structure as `name ‖ "(" ‖ member₁ ‖ "," ‖ member₂ ‖ "," ‖ … ‖ memberₙ ")".
    pub fn get_encoded_type(&self) -> String {
        let mut encoded_type = String::new();
        encoded_type.push_str(&self.member_type);
        encoded_type.push('(');

        let mut members = self.sub_members.iter();

        if let Some(member) = members.next() {
            encoded_type.push_str(&member.member_type);
            encoded_type.push(' ');
            encoded_type.push_str(&member.name);
        }
        for member in members {
            encoded_type.push(',');
            encoded_type.push_str(&member.member_type);
            encoded_type.push(' ');
            encoded_type.push_str(&member.name);
        }

        encoded_type.push(')');

        encoded_type
    }
}
