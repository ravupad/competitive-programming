import java.util.*;

class LinkSameHeightNode {
	int n;
	LinkSameHeightNode left;
	LinkSameHeightNode right;
	LinkSameHeightNode next;
	int height;

	public LinkSameHeightNode(int n, LinkSameHeightNode left, LinkSameHeightNode right) {
		this.n = n;
		this.left = left;
		this.right = right;
		this.next = null;
	}

	void linkSameHeightNode() {
		Queue<LinkSameHeightNode> q = new ArrayDeque<LinkSameHeightNode>();
		ArrayList<LinkSameHeightNode> nodeAtHeight = new ArrayList<>();
		this.height = 0;
		q.add(this);
		while(q.peek() != null) {
			LinkSameHeightNode node = q.remove();
			if (node.left != null) {
				node.left.height = node.height + 1;
				q.add(node.left);
			}
			if (node.right != null) {
				node.right.height = node.height + 1;
				q.add(node.right);
			}
			if (nodeAtHeight.size() == node.height) {
				nodeAtHeight.add(node);
			} else {
				nodeAtHeight.get(node.height).next = node;
				nodeAtHeight.set(node.height, node);
			}
		}
	}

	public static void main(String[] args) {
		LinkSameHeightNode root = new LinkSameHeightNode(
			0,
			new LinkSameHeightNode(
				1,
				new LinkSameHeightNode(
					3, 
					new LinkSameHeightNode(
						7,
						null,
						null), 
					null),
				new LinkSameHeightNode(
					4, 
					null, 
					null)),
			new LinkSameHeightNode(
				2,
				new LinkSameHeightNode(
					5, 
					null, 
					null),
				new LinkSameHeightNode(
					6, 
					null, 
					null)));
		root.linkSameHeightNode();
		System.out.println(root.left.left.next.next.n);

	}	
}
