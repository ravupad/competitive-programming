// codechef ACM14KP1
// a magical length
// closest pair, divide and conquer
#include <bits/stdc++.h>
#define x first
#define y second

using namespace std;

typedef pair<double, double> point;

point p[100000], l[100000], r[100000], b[100000];
double min_d;

inline bool compareX(const point &p1, const point &p2) {
  return p1.first < p2.first;
}

inline bool compareY(const point &p1, const point &p2) {
  return p1.second < p2.second;
}

inline double distance(const point p1, const point p2) {
  return sqrt((p1.x-p2.x)*(p1.x-p2.x)+(p1.y-p2.y)*(p1.y-p2.y));
}

inline void update_min_d(const point p1, const point p2, const point p3) {
  double d1 = distance(p1, p2);
  if (d1 > min_d) return;
  double d2 = distance(p1, p3);
  if (d1+d2 > min_d) return;
  double d3 = distance(p3, p2);
  min_d = min(min_d, d1+d2+d3);
}

void merge(int s, int m, int e) {
  int lcounter = s;
  int rcounter = m+1;
  for (int i = s; i <= e; i++) {
    if (lcounter == m+1) {
      b[i-s] = p[rcounter++];
    } else if (rcounter == e+1) {
      b[i-s] = p[lcounter++];
    } else if (p[lcounter].y <= p[rcounter].y) {
      b[i-s] = p[lcounter++];
    } else {
      b[i-s] = p[rcounter++];
    }        
  }
  for (int i = s; i <= e; i++) {
    p[i] = b[i-s];
  }
}

// 1 point in x and 2 point in y
void find(point *ls, int lsl, point *rs, int rsl) {
  int rcounter = 0;
  for (int i = 0; i < lsl; i++) {
    while (rcounter < rsl && ls[i].y-rs[rcounter].y > min_d) rcounter++;
    for (int j = rcounter; j < rsl-1 && rs[j].y-ls[i].y < min_d; j++) {
      double d = distance(ls[i], rs[j]);
      if (d > min_d) continue;
      for (int k = j+1; k < rsl && d+rs[k].y-rs[j].y < min_d; k++) {
        update_min_d(ls[i], rs[j], rs[k]);
      }
    }
  }
}

void solve(int start, int end) {
  int lcount;
  int rcount;
  int mid = (start+end)/2;
  double mx1;
  double mx2;
  if (end-(start-1) <= 10) {
    sort(p+start, p+end+1, compareY);
    for (int i = start; i <= end; i++) {
      for (int j = i+1; j <= end && p[j].y-p[i].y < min_d/2; j++) {
        double d = distance(p[i], p[j]);
        for (int k = j+1; k <= end && d + 2*p[k].y-p[j].y-p[i].y < min_d; k++) {
          update_min_d(p[i], p[j], p[k]);
        }
      }      
    }
    return;
  }
  mx1 = p[mid].x;
  mx2 = p[mid+1].x;
  solve(start, mid);
  solve(mid+1, end);
  if (mx2-mx1 > min_d/2) {
    merge(start, mid, end);
    return;
  }
  lcount = 0;
  for (int i = start; i <= mid; i++)
    if (mx2-p[i].x < min_d/2) l[lcount++] = p[i];
  rcount = 0;
  for (int i = mid+1; i <= end; i++)
    if (p[i].x-mx1 < min_d/2) r[rcount++] = p[i];  
  find(l, lcount, r, rcount);
  find(r, rcount, l, lcount);
  merge(start, mid, end);
  return;
}

int main() {
  ios_base::sync_with_stdio(false);
  cin.tie(0);
  cout.tie(0);
  cout.precision(15);
  int tc, n;
  cin >> tc;
  for (int t = 1; t <= tc; t++) {
    cin >> n;
    for (int i = 0; i < n; i++) cin >> p[i].x >> p[i].y;
    sort(p, p+n, compareX);
    min_d = numeric_limits<double>::max();
    solve(0, n-1);
    cout << "Case " << t << ": " << min_d << endl;
  }
  return 0;
}

