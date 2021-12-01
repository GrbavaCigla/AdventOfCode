#include <bits/stdc++.h>

#define ll long long

using namespace std;

int main() {

    int n = 323, m = 31;
    char mat[n][m + 1];
    for(int i = 0; i < n; i++)
        scanf("%s", mat[i]);
    
    ll sol = 1;
    pair<int, int> slopes[5] = {{1, 1}, {1, 3}, {1, 5}, {1, 7}, {2, 1}};
    for(pair<int, int> d: slopes) {
        int i = 0, j = 0;
        ll ans = 0;
        while(i < n) {
            if(mat[i][j] == '#')
                ans++;
            i += d.first;
            j = (j + d.second) % m;
        }
        sol *= ans;
    }
    printf("%lld\n", sol);
    return 0;
}
