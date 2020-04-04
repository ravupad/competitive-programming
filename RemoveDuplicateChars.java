import java.util.*;

public class RemoveDuplicateChars {
	public static void main(String[] args) {
		System.out.println(removeDuplicateChars("abaaabc"));
	}

	static String removeDuplicateChars(String str) {
		boolean isPresent[] = new boolean[256];
		int str2[] = new int[str.length()];
		int str2Counter = 0;
		for (int i = 0; i < 256; i++) {
			isPresent[i] =  false;
		}
		for (int i = 0; i < str.length(); i++) {
			if (!isPresent[str.codePointAt(i)]) {
				str2[str2Counter++] = str.codePointAt(i);
				isPresent[str.codePointAt(i)] = true;
			}
		}
		return new String(str2, 0, str2Counter);
	}
}