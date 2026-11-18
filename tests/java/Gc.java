import java.util.*;

public class Gc {
    static class Node { Node next; int[] payload; Node(Node n, int size) { next = n; payload = new int[size]; } }
    public static void main(String[] args) {
        long total = 0;
        for (int round = 0; round < 200; round++) {
            Node head = null;
            for (int i = 0; i < 500; i++) head = new Node(head, 64);
            int len = 0;
            for (Node n = head; n != null; n = n.next) { len++; n.payload[3] = len; }
            total += len;
        }
        System.out.println("allocated nodes: " + total);
        List<String> keep = new ArrayList<>();
        for (int i = 0; i < 20000; i++) { String s = "item-" + i; if (i % 1000 == 0) keep.add(s); }
        System.out.println(keep.size() + " " + keep.get(5) + " " + keep.get(19));
        Map<Integer, String> map = new HashMap<>();
        for (int i = 0; i < 50000; i++) { map.put(i % 997, "v" + i); }
        System.out.println(map.size() + " " + map.get(500));
        StringBuilder sb = new StringBuilder();
        for (int i = 0; i < 2000; i++) sb.append((char) ('a' + i % 26));
        System.out.println(sb.length() + " " + sb.substring(1990));
    }
}
