#include <stdio.h>

int main() {
    int numero;

    printf("Digite um número: ");
    scanf("%d", &numero);

    printf("\nConversões\n");

    printf("\nBinário: ");
    int binario[32];
    int i = 0, n = numero;

    if(n == 0) {
        printf("0");
    } else {
        while(n > 0) {
            binario[i] = n % 2;
            n = n / 2;
            i++;
        }

        for(int j = i - 1; j >= 0; j--) {
            printf("%d", binario[j]);
        }
    }

    printf("\nOctal: %o", numero);

    printf("\nHexadecimal: %X\n", numero);

    return 0;
}