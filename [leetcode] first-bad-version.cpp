// leetcode https://leetcode.com/explore/featured/card/may-leetcoding-challenge/534/week-1-may-1st-may-7th/3316/
// binary search; binary-search

// The API isBadVersion is defined for you.
// bool isBadVersion(int version);

class Solution {
public:
    int firstBadVersion(int n) {
        int start = 1;
        int end = n;
        int mid;
        while (end != start) {
            mid = start+((end-start)/2);
            if (isBadVersion(mid)) {
                end = mid;
            } else {
                start = mid+1;
            }
        }
        return start;
    }
};
