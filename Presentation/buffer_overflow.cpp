#include <iostream>
using namespace std;

int main(){
    char buffer[5];
    char *p;
    p = buffer;
    for(int i=0; i<10; i++){
        *p = 'A';
        p++;
    }
    cout << buffer << endl;
    return 0;    
}
