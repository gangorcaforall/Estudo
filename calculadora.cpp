#include <iostream>
using namespace std;

int main() {
    string nome;
    float nota1, nota2, nota3, media;
    float maior, menor;

    cout << "Digite o nome do aluno: ";
    cin >> nome;

    cout << "Digite a primeira nota: ";
    cin >> nota1;

    cout << "Digite a segunda nota: ";
    cin >> nota2;

    cout << "Digite a terceira nota: ";
    cin >> nota3;

    media = (nota1 + nota2 + nota3) / 3;

    maior = nota1;

    if (nota2 > maior) {
        maior = nota2;
    }

    if (nota3 > maior) {
        maior = nota3;
    }

    menor = nota1;

    if (nota2 < menor) {
        menor = nota2;
    }

    if (nota3 < menor) {
        menor = nota3;
    }

    cout << "\nAluno: " << nome << endl;
    cout << "Media: " << media << endl;
    cout << "Maior nota: " << maior << endl;
    cout << "Menor nota: " << menor << endl;

    if (media >= 7) {
        cout << "Resultado: Aprovado" << endl;
    } else {
        cout << "Resultado: Reprovado" << endl;
    }

    return 0;
}