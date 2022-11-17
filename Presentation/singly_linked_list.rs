struct Node {
    data: i32,
    // Tenemos dos opciones, o utilizamos Some() o None()
    // lo que se encarga de hacer Option() es probar si el puntero posee un valor o no
    // Option() es la mezcla de Some y None
    next: Option<Box<Node>>,
}

impl Node {
    // constructor
    fn new(data: i32) -> Node {
        Node {
            data: data,
            next: None,
        }
    }
    
    // dos argumentos: referencia al nodo y un vector i32
    fn create(node: &mut Node, data: Vec<i32>) {
        // nuevo nodo
        let new_node = Node::new(data[0]);
        // asignar el siguiente nodo al nuevo nodo
        node.next = Some(Box::new(new_node));
        // si la longitud del vector es mayor que 1
        if data.len() > 1 {
            // crear un nuevo vector con los datos del anterior vector
            let mut new_data = Vec::new();
            for i in 1..data.len() {
                new_data.push(data[i]);
            }
            // llamar la función create de nuevo
            Node::create(&mut node.next.as_mut().unwrap(), new_data);
        }
    }

    // imprimir la lista
    fn display(node: &Node) {
        print!("{} ", node.data);
        if node.next.is_some() {
            Node::display(&node.next.as_ref().unwrap());
        }
    }
}

fn main(){
    let mut node = Node::new(1);
    let data = vec![2, 3, 4, 5, 6, 7, 8, 9, 10];
    Node::create(&mut node, data);
    Node::display(&node);
    println!();
}
