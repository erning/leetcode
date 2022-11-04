#include <iostream>
#include <vector>

using namespace std;

class Solution {
public:
    vector<int> countBits(int n) {
        vector<int> ans;
        ans.push_back(0);
        int m = 0, k = 1;
        for (int i = 1; i <= n; ++i) {
            if (i > k) { m = k; k = m << 1 | 1; }
            ans.push_back(1 + ans[i & m]);
        }
        return ans;
    }
};

int main() {
    Solution s;
    auto ans = s.countBits(10);
    cout << '[';
    if (ans.size() > 0) {
        cout << ans[0];
        for (int i = 1; i < ans.size(); ++i) {
            cout << ", " << ans[i];
        }
    }
    cout << ']' << endl;
    return 0;
}
