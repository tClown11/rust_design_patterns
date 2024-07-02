use prost::Message;
use prost_demo::pb::*;

fn main() {
    let phones = vec![PhoneNumber::new("111-222-3334", PhoneType::Mobile)];
    let person = Person::new("clown", 1, "clown@outlook.com", phones);
    let v1 = person.encode_to_vec();
    let person1 = Person::decode(v1.as_ref()).unwrap();
    assert_eq!(person1, person);

    let json = serde_json::to_string_pretty(&person).unwrap();

    println!("{}", json)
}