#include <bits/stdc++.h>

using namespace std;

int main() {

    int n = 323, m = 31;
    char mat[n][m + 1];
    for(int i = 0; i < n; i++)
        scanf("%s", mat[i]);
   
    int i = 0, j = 0, ans = 0;
    while(i < n) {
        if(mat[i][j] == '#')
            ans++;
        i++;
        j = (j + 3) % m;
    }
    printf("%d\n", ans);
    return 0;
}
