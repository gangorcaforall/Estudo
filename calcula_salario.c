#include <stdio.h>

int main() {
    float valorHora, horasTrabalhadas, salario;

    printf("Quanto voce ganha por hora? ");
    scanf("%f", &valorHora);

    printf("Quantas horas trabalhou no mes? ");
    scanf("%f", &horasTrabalhadas);

    salario = valorHora * horasTrabalhadas;

    printf("O salario do mes é: R$ %.2f\n", salario);

    return 0;
}