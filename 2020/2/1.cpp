#include <bits/stdc++.h>

using namespace std;

int main() {

    int n = 1000;

    int ans = 0;
    for(int i = 0; i < n; i++) {
        int a, b;
        char c, s[100];
        scanf("%d-%d %c: %s", &a, &b, &c, s);

        int m = strlen(s);
        int br = 0;
        for(int j = 0; j < m; j++)
            if(s[j] == c)
                br++;
        if(a <= br && br <= b)
            ans++;
    }
    printf("%d\n", ans);
    return 0;
}