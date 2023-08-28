use ethers::types::H256;
use std::collections::{BTreeMap, VecDeque};

use super::eip712_standard::{EncodedMember, Structuralization};

/// Trait that collects members of the structure into the structure of the EIP-712 standard.
pub trait AbsorbMember {
    fn absorb<MEMBER: Structuralization>(&mut self, name: &str, member: &MEMBER);
}

// Builder that encodes type information and structure data for for hashing the structure according to the EIP-712 standard.
#[derive(Clone, Default, Debug)]
pub(crate) struct MembersCollecter {
    pub members: Vec<EncodedMember>,
}

impl AbsorbMember for MembersCollecter {
    fn absorb<MEMBER: Structuralization>(&mut self, name: &str, member: &MEMBER) {
        self.members.push(EncodedMember::build(name, member));
    }
}

impl MembersCollecter {
    /// Returns the concatenation of the encoded member values in the order that they appear in the type.
    pub fn encode_data(&self) -> Vec<H256> {
        // encodeData(s : 𝕊) = enc(value₁) ‖ enc(value₂) ‖ … ‖ enc(valueₙ).
        self.members
            .iter()
            .map(|member| member.encode_value)
            .collect()
    }

    // Return the encoded structure type as `name ‖ "(" ‖ member₁ ‖ "," ‖ member₂ ‖ "," ‖ … ‖ memberₙ ")"`.
    pub fn encode_type(&self, struct_name: &str) -> String {
        let mut outer_members_builder = OuterTypeBuilder::default();
        for member in self.members.iter() {
            outer_members_builder.add_member(member.clone());
        }
        let outer_members = outer_members_builder.build();

        // Collecting all members of the structure as a coded structure.
        let inner_member = {
            let member_type = struct_name.to_string();
            let inner_members = self.members.clone();
            EncodedMember {
                member_type,
                name: String::default(),
                is_reference_type: true,
                sub_members: inner_members,
                encode_value: Default::default(),
            }
        };

        let mut encode = inner_member.get_encoded_type();
        for (_, outer_member) in outer_members {
            encode.push_str(&outer_member.get_encoded_type());
        }
        encode
    }
}

#[derive(Clone, Default, Debug)]
struct OuterTypeBuilder {
    inner_members_queue: VecDeque<EncodedMember>,
}

impl OuterTypeBuilder {
    fn add_member(&mut self, encoded_member: EncodedMember) {
        // If the type is not used by the structure, then it is possible not
        // to process it as it is not included in the list of types of nested structures.
        if encoded_member.is_reference_type {
            self.inner_members_queue.push_back(encoded_member);
        }
    }

    fn build(mut self) -> BTreeMap<String, EncodedMember> {
        // All nested structures must be added to the encoded type alphabetically,
        // so we will support a red-black tree with a key by the name of the structure type.
        let mut result = BTreeMap::new();

        while let Some(front_element) = self.inner_members_queue.pop_front() {
            if result.get(&front_element.member_type).is_some() {
                continue;
            }

            result.insert(front_element.member_type.clone(), front_element.clone());
            for inner_member in front_element.sub_members {
                if inner_member.is_reference_type && result.get(&inner_member.member_type).is_none()
                {
                    self.inner_members_queue.push_back(inner_member);
                }
            }
        }
        result
    }
}
