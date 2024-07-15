#include <iostream>
#include <fstream>
#include <queue>
#include <vector>
#include <string>
#include <algorithm>
#include <set>
#include <cstring>
#include <unordered_map>
#define ll long long
#define ull unsigned long long
#define ld long double
using namespace std;

ll n, k1, k2;
ll a[500005];

int main(){
	ios::sync_with_stdio(0);
	cin.tie(0);
	cout.tie(0);
	cin >> n >> k1 >> k2;
	k2 = min(60ll, k2);
	k1 = min(k2, k1);
	for(int i = 1; i <= n; i++) {
		cin >> a[i];
	}
	ll ansk2 = 0, ansk1 = 0;

	ll last = 1;
	for(int i = 2; i <= n; i++) {
		if((a[i] >> k2) != (a[i - 1] >> k2)) {
			ansk2 += (i - last) * (i - last - 1) / 2;
			last = i;
		}
	}
	ansk2 += (n - last + 1) * (n - last) / 2;
	last = 1;
	for(int i = 2; i <= n; i++) {
		if((a[i] >> k1) != (a[i - 1] >> k1)) {
			ansk1 += (i - last) * (i - last - 1) / 2;
			last = i;
		}
	}
	ansk1 += (n - last + 1) * (n - last) / 2;

	cout << ansk2 - ansk1 << "\n";
	return 0;

	ll T;
	cin >> T;
	while(T--){
	}
	return 0;
}
