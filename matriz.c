#include <stdio.h>

int main() {
    int linhas, colunas;

    scanf("%d %d", &linhas, &colunas);

    int matriz[100][100];
    int a, b;

    for(a = 0; a < linhas; a++) {
        for(b = 0; b < colunas; b++) {
            scanf("%d", &matriz[a][b]);
        }
    }

    for(b = 0; b < colunas; b++) {
        for(a = 0; a < linhas; a++) {
            printf("%d ", matriz[a][b]);
        }
        printf("\n");
    }

    return 0;
}