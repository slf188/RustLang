fn tower_of_hanoi(x: i32, from: i32, aux: i32, to: i32) {
    if x > 0 {
        tower_of_hanoi(x - 1, from, to, aux);
        println!("From {} to {}", from, to);
        tower_of_hanoi(x - 1, aux, from, to);
    }
}

fn main() {
    let x = 3;
    tower_of_hanoi(x, 1, 2, 3);
}

/*
The equivalent in C++ would be:
#include <iostream>
using namespace std;

void towerOfHanoi(int x, int from, int aux, int to){
    if(x > 0){
        towerOfHanoi(x-1, from, to, aux);
        printf("From %d to %d\n", from, to);
        towerOfHanoi(x-1, aux, from, to);
    }
}

int main(){
    int n = 3;
    towerOfHanoi(n, 1, 2, 3);
    return 0;
}
*/
