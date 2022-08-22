use std::fmt::{self, Debug, Formatter};

use super::*;

//블록체인에서 이  블록의 위치
//이 블록에 포함된 정보
//타임스탬프 페이로드에 저장된 정보를 특정 시점에 연결
//nonce 마이닝을 찾는 값
//이전블록해시
//블록의 해시
pub struct Block {
    pub index: u32,
    pub timestamp: u128,
    pub hash: Hash,
    pub prev_block_hash: Hash,
    pub nonce: u64,
    pub payload: String,
    pub difficulty: u128,
}

impl Debug for Block {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "Block[{}]:{}at:{} with:{} nonce: {}",
            &self.index,
            &hex::encode(&self.hash),
            &self.timestamp,
            &self.payload,
            &self.nonce,
        )
    }
}
impl Block {
    pub fn new(
        index: u32,
        timestamp: u128,
        prev_block_hash: Hash,
        nonce: u64,
        payload: String,
        difficulty: u128,
    ) -> Self {
        Block {
            index,
            timestamp,
            hash: vec![0; 32],
            prev_block_hash,
            nonce,
            payload,
            difficulty,
        }
    }
    pub fn mine(&mut self) {
        for nonce_attempt in 0..(u64::max_value()) {
            self.nonce = nonce_attempt;
            let hash = self.hash();
            if check_difficulty(&hash, self.difficulty) {
                self.hash = hash;
                return;
            }
        }
    }
}

impl Hashable for Block {
    fn bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend(&u32_bytes(&self.index));
        bytes.extend(&u128_bytes(&self.timestamp));
        bytes.extend(&self.prev_block_hash);
        bytes.extend(&u64_bytes(&self.nonce));
        bytes.extend(self.payload.as_bytes());
        bytes.extend(&u128_bytes(&self.difficulty));
        bytes
    }
}
pub fn check_difficulty(hash: &Hash, difficulty: u128) -> bool {
    difficulty > difficulty_bytes_as_u128(&hash)
}
//해시란
//해시 알고리즘의 결과로데이터에 대한 식별자 또는 지문을 생성
//동일한 해시를 가진 두개의 데이터를 찾는것은 거의 불가능
//모든 데이터의 해시를 계산하는것은 쉬움
//MD5("GeekLaunch") = "e76485e55ba4c16aac30bd446b73d96e"
//SHA1("GeekLaunch") = "c333e84f729c67d6b591e056e1b51e0077a9c030
//SHA256("GeekLaunch") = "a17d5669f2148e2982baab7c0b4c7d81100c7cf52c45a8d7deb429aeba156ea6"
//비트코인 이 SHA256사용

//해싱블록
//지금까지 블록 구조에서 인덱스 페이로드 타임스탬프 nonce이전블록해시
