struct Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        if strs.len() == 0 {
            return Vec::new();
        }
        let mut pairs = Vec::new();
        for s in strs.into_iter() {
            let mut chars: Vec<_> = s.chars().collect();
            chars.sort();
            pairs.push((chars, s));
        }
        pairs.sort();
        let mut pairs_iter = pairs.into_iter();
        let (mut sorted, string) = pairs_iter.next().unwrap();
        let mut result = vec![vec![string]];
        for pair in pairs_iter {
            let (sorted2, string2) = pair;
            if sorted == sorted2 {
                result.last_mut().unwrap().push(string2);
            } else {
                sorted = sorted2;
                result.push(vec![string2]);
            }
        }
        result        
    }
}

fn main() { 
    println!("{:?}", Solution::group_anagrams(
        vec!["ate", "eat", "tea", "tan", "nat", "bat", "tab"]
            .iter()
            .map(|x| x.to_string())
            .collect()));
}
