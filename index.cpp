#include <iostream>
using namespace std;
int main() {
    
    int opcao;
    float saldo = 1000;
    float deposito;
    
    cout << "Bem vindo ao banco!" << endl;
    cout << "1 - Ver saldo" << endl << "2 - Depositar" << endl << "3 - Sacar" << endl << "4 - Sair" << endl;
    cin >> opcao;
    
    while (opcao != 4) {
        
        if (opcao == 1) {
            
           cout << "O seu saldo atual é: " << saldo;
        } else if (opcao == 2) {
            
            cout << "Digite o valor do deposito: ";
            cin >> deposito;
            
            saldo += deposito;
        } 
        
        cout << "1 - Ver saldo" << endl << "2 - Depositar" << endl << "3 - Sacar" << endl << "4 - Sair" << endl;
        cin >> opcao;
    }
    
    return 0;
}