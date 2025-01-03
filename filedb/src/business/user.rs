use std::collections::HashMap;

use crate::database::file_db;

pub struct User {
    pub id: i32,
    pub age: i32,
    pub name: String,
}

pub struct UserManager {
    user_map: HashMap<i32, User> // 사용자 id와 정보를 저장하는 hashmap
}

impl UserManager {
    pub fn new() -> UserManager {
        let mgr = UserManager {
            user_map: HashMap::new()
        };
        
        mgr
    }
    
    // 사용자 추가 메서드
    pub fn add_user(&mut self, id:i32, age:i32, name: String) -> bool {
        let mut user = User {
            id:id,
            age:age,
            name:name,
        };

        self.user_map.entry(user.id).or_insert(user);
        true
    }

    // 사용자 제거 메서드
    pub fn remove_user(&mut self, id:i32) -> bool {
        // 해당 id가 없으면 false 반환
        if self.user_map.contains_key(&id) == false {
            return false;
        }
        // 해당 id로 사용자 제거
        self.user_map.remove(&id);
        true
    }

    pub fn get_user(&mut self, id:i32) -> Option<&User> {
        self.user_map.get(&id)
    }

    pub fn get_all(&mut self) -> Vec<&User> {
        let mut v : Vec<&User> = Vec::new();

        for u in self.user_map.values() {
            v.push(&u);
        }

        return v
    }

    pub fn save(&mut self) {
        file_db::save(String::from("file.db"), self.get_all());
    }

    pub fn load(&mut self){
        let user_vec = file_db::load(String::from("file.db"));
        self.user_map = HashMap::new();

        for user in user_vec.iter() {
            self.add_user(user.id, user.age, user.name.clone());
        }
    }

}