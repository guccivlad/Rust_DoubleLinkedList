use test_project::doublelinkedlist::DoubleLinkedList;


fn main() {
    let mut list: DoubleLinkedList<String> = DoubleLinkedList::new();

    list.push_back("Hello".to_string());
    list.push_back(" world,".to_string());
    list.push_back("I am ".to_string());
    list.push_back("aaaa".to_string());
    list.push_back(" super mega developer".to_string());

    for value in list.iter() {
        print!("{}", value);
    }
    println!();

    println!("List len: {}", list.len());
    list.remove(3);
    println!("After:");

    for value in list.iter() {
        print!("{}", value);
    }
    println!();
    println!("{}", list.contains(&"Hello".to_string()));
}