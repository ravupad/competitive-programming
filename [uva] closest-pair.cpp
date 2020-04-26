#include <cstdio>
#include <vector>
#include <cmath>
#include <bits/stdc++.h>

using namespace std;

typedef pair<double, double> point;

point p[10000], l[10000], r[10000], buffer[10000];

inline double distance2(point p1, point p2) {
  double dx = p1.first-p2.first;
  double dy = p1.second-p2.second;
  return (dx*dx+dy*dy);
}

inline double distance(point p1, point p2) {
  return sqrt(distance2(p1, p2));
}

bool compareX(point p1, point p2) {
  return p1.first < p2.first;
}

bool compareY(point p1, point p2) {
  return p1.second < p2.second;
}

// start and end inclusive
// end-(start-1) should be >= 2
// p should be sorted by x co-ordinate beforehand
// this function also sorts the segment [start, end] by
// y co-ordinate
double solve(int start, int end) {
  int mid = (start+end)/2, lcount = 0, rcount = 0, lcounter, rcounter;
  double min_distance = numeric_limits<double>::max();
  point lmax, rmin;
  // base conditions of size 1, 2 and 3
  if (start == end) {
    return min_distance;
  } else if (start+1 == end) {
    sort(p+start, p+end+1, compareY);
    return distance(p[start], p[end]);
  } else if (start+2 == end) {
    sort(p+start, p+end+1, compareY);
    min_distance = min(min_distance, distance(p[start], p[start+1]));
    min_distance = min(min_distance, distance(p[start], p[start+2]));
    min_distance = min(min_distance, distance(p[start+1], p[start+2]));
    return min_distance;
  }
  // the right most point in left section
  lmax = p[mid];
  // the left most point in right section
  rmin = p[mid+1];
  // the solution for l section. also [start, mid]
  // will be sorted by y co-ordinate afterwards
  min_distance = min(solve(start, mid), min_distance);
  // the solution for r section. also [mid+1, end]
  // will be sorted by y co-ordinate afterwards
  min_distance = min(solve(mid+1, end), min_distance);
  // collect all the p in [start, mid]
  // which are min_distance away from
  // vertical line passing through point[mid+1]
  for (int i = start; i <= mid; i++) {
    if (rmin.first - p[i].first <= min_distance) {
      l[lcount++] = p[i];
    }
  }
  // collect all the p in [mid+1, start]
  // which are min_distance away from
  // vertical line passing through point[mid]
  for (int i = mid+1; i <= end; i++) {
    if (p[i].first - lmax.first <= min_distance) {
      r[rcount++] = p[i];
    }
  }
  // now we will search for a pair with one in l
  // and one point in r such that their distance
  // is less than min_distance.
  rcounter = 0;
  for (int i = 0; i < lcount; i++) {
    // increase rcounter till the point at rcounter
    // has y co-ordinate that is not out of range
    // compared to y co-ordinate of l[i]
    while (rcounter < rcount && l[i].second - r[rcounter].second > min_distance) {
      rcounter++;
    }
    for (int j = rcounter; j < rcount; j++) {
      // break the loop if r[j]'s y co-ordinate
      // is more than min_distance away from y
      // co-ordinate of l[i]
      if (r[j].second - l[i].second > min_distance) {
        break;
      }
      min_distance = min(distance(r[j], l[i]), min_distance);
    }
  }
  // now sort the range [start, mid] and [mid+1, end]
  // by y co-ordinate using merge of these two ranges
  lcounter = start;
  rcounter = mid+1;
  lcount = 0;
  while (lcounter <= mid || rcounter <= end) {
    if (lcounter > mid) {
      buffer[lcount++] = p[rcounter++];
    } else if (rcounter > end) {
      buffer[lcount++] = p[lcounter++];
    } else if (p[lcounter].second <= p[rcounter].second) {
      buffer[lcount++] = p[lcounter++];
    } else {
      buffer[lcount++] = p[rcounter++];
    }
  }
  for (int i = start; i <= end; i++) {
    p[i] = buffer[i-start];
  }
  return min_distance;
}

int main() {
  int n;
  double p1, p2, ans;
  while (true) {
    scanf("%d", &n);
    if (n == 0) {
      break;
    }
    for (int i = 0; i < n; i++) {
      scanf("%lf %lf", &p1, &p2);
      p[i] = point(p1, p2);
    }
    sort(p, p+n, compareX);
    ans = solve(0, n-1);
    if (ans >= 10000) {
      printf("INFINITY\n");
    } else {
      printf("%0.4lf\n", ans);
    }
  }
  return 0;
}
