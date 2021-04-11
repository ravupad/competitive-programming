// https://leetcode.com/problems/bulls-and-cows/
class Solution {
    public String getHint(String secret, String guess) {
        int len = secret.length();
        int a = 0, b = 0;
        int[] secret_frequency = new int[10];
        int[] guess_frequency = new int[10];
        for (int i = 0; i < 10; i++) {
            secret_frequency[i] = 0;
            guess_frequency[i] = 0;
        }
        for (int i = 0; i < len; i++) {
            if (secret.charAt(i) == guess.charAt(i)) {
                a += 1;
            } else {
                secret_frequency[secret.charAt(i) - '0'] += 1;
                guess_frequency[guess.charAt(i) - '0'] += 1;
            }
        }
        for (int i = 0; i < 10; i++) {
            b += Math.min(secret_frequency[i], guess_frequency[i]);
        }
        return "" + a + "A" + b + "B";
    }
}