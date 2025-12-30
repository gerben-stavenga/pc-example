use allocator_api2::alloc::Global;
use pc_example::example::{Person, AddressBook};
use protocrap::{ProtobufMut, ProtobufRef};

fn main() {
    // Create an arena for allocations
    let mut arena = protocrap::arena::Arena::new(&Global);

    // Create a person
    let mut person = Person::ProtoType::default();
    person.set_name("Alice", &mut arena);
    person.set_age(30);
    person.emails_mut().push(
        protocrap::containers::String::from_str("alice@example.com", &mut arena),
        &mut arena,
    );

    println!("Person: {:?}", person);
    println!("Name: {}", person.name());
    println!("Age: {}", person.age());
    println!("Emails: {:?}", person.emails());

    // Serialize to bytes
    let bytes = person.encode_vec::<32>().unwrap();
    println!("Serialized ({} bytes): {:?}", bytes.len(), bytes);

    // Deserialize
    let mut decoded = Person::ProtoType::default();
    assert!(decoded.decode_flat::<32>(&mut arena, &bytes));
    println!("Decoded: {:?}", decoded);

    // Create an address book with multiple people
    let mut book = AddressBook::ProtoType::default();
    let p1 = book.add_people(&mut arena);
    p1.set_name("Bob", &mut arena);
    p1.set_age(25);

    let p2 = book.add_people(&mut arena);
    p2.set_name("Charlie", &mut arena);
    p2.set_age(35);

    println!("\nAddress book: {:#?}", book);
}
