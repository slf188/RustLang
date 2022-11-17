use std::collections::LinkedList;

fn main(){
    // creacion de una lista enlazada
    let mut list = LinkedList::new();
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);
    list.push_back(4);
    list.push_back(5);
    println!("La longitud de la lista es: {}", list.len());
    println!("El primer elemento de la lista es: {}", list.front().unwrap());
    // check the last element
    println!("El ultimo elemento de la lista es: {}", list.back().unwrap());
    // eliminar el primer elemento
    list.pop_front();
    // eliminar el ultimo elemento
    list.pop_back();
    println!("La lista contiene el elemento 3?: {}", list.contains(&3));
}
