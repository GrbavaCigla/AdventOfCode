#include <bits/stdc++.h>

using namespace std;

int main() {

    int n = 1000;

    int ans = 0;
    for(int i = 0; i < n; i++) {
        int a, b;
        char c, s[100];
        scanf("%d-%d %c: %s", &a, &b, &c, s);
        a--; b--;

        int m = strlen(s);
        bool val = false;
        if(s[a] == c)
            val = !val;
        if(s[b] == c)
            val = !val;
        if(val)
            ans++;
    }
    printf("%d\n", ans);
    return 0;
}