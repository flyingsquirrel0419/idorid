import java.util.*;

public class Collections1 {
    public static void main(String[] args) {
        Map<String, Integer> m = new HashMap<>();
        String text = "the quick brown fox jumps over the lazy dog the end";
        for (String w : text.split(" ")) m.put(w, m.getOrDefault(w, 0) + 1);
        System.out.println(m);
        System.out.println(m.get("the") + " " + m.containsKey("fox") + " " + m.size() + " " + m.remove("dog") + " " + m.size());
        TreeMap<String, Integer> tm = new TreeMap<>(m);
        System.out.println(tm + " first=" + tm.firstKey() + " last=" + tm.lastKey() + " head=" + tm.headMap("fox"));
        LinkedHashMap<Integer, String> lhm = new LinkedHashMap<>();
        for (int i = 5; i > 0; i--) lhm.put(i, "v" + i);
        System.out.println(lhm.keySet() + " " + lhm.values());
        Set<Integer> hs = new HashSet<>();
        for (int i = 0; i < 20; i += 3) hs.add(i % 7);
        System.out.println(hs + " " + hs.contains(6));
        TreeSet<String> ts = new TreeSet<>(Arrays.asList("banana", "apple", "cherry", "apple"));
        System.out.println(ts + " " + ts.first() + " " + ts.ceiling("b") + " " + ts.descendingSet());
        ArrayList<Integer> al = new ArrayList<>();
        for (int i = 0; i < 10; i++) al.add(i * i);
        al.remove(Integer.valueOf(16)); al.remove(0); al.add(2, 99);
        System.out.println(al + " " + al.indexOf(99) + " " + al.subList(2, 5) + " " + al.contains(81));
        Iterator<Integer> it = al.iterator();
        while (it.hasNext()) if (it.next() % 2 == 1) it.remove();
        System.out.println(al);
        LinkedList<String> ll = new LinkedList<>();
        ll.add("b"); ll.addFirst("a"); ll.addLast("c"); ll.push("z");
        System.out.println(ll + " " + ll.pop() + " " + ll.peekLast() + " " + ll);
        ArrayDeque<Integer> dq = new ArrayDeque<>();
        for (int i = 0; i < 5; i++) { dq.offerFirst(i); dq.offerLast(-i); }
        System.out.println(dq + " " + dq.pollFirst() + " " + dq.pollLast());
        PriorityQueue<Integer> pq = new PriorityQueue<>(Arrays.asList(5, 1, 8, 3, 2, 9));
        StringBuilder sb = new StringBuilder();
        while (!pq.isEmpty()) sb.append(pq.poll()).append(' ');
        System.out.println(sb);
        List<Integer> nums = new ArrayList<>(Arrays.asList(3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5));
        Collections.sort(nums); System.out.println(nums);
        Collections.reverse(nums); System.out.println(nums + " max=" + Collections.max(nums) + " freq5=" + Collections.frequency(nums, 5));
        Collections.shuffle(nums, new Random(42)); System.out.println(nums);
        Random r = new Random(12345);
        System.out.println(r.nextInt() + " " + r.nextInt(100) + " " + r.nextLong() + " " + r.nextDouble() + " " + r.nextBoolean() + " " + r.nextFloat());
        Map<Character, List<String>> groups = new TreeMap<>();
        for (String w : new String[]{"apple", "avocado", "banana", "blueberry", "cherry"})
            groups.computeIfAbsent(w.charAt(0), k -> new ArrayList<>()).add(w);
        System.out.println(groups);
        int[] arr = {9, 4, 7, 1};
        Arrays.sort(arr); System.out.println(Arrays.toString(arr) + " " + Arrays.binarySearch(arr, 7));
        List<String> unmod = Collections.unmodifiableList(Arrays.asList("x", "y"));
        try { unmod.add("z"); } catch (UnsupportedOperationException e) { System.out.println("unmodifiable"); }
        for (Map.Entry<String, Integer> e : tm.entrySet()) if (e.getValue() > 1) System.out.println(e.getKey() + "=" + e.getValue());
        List<Integer> big = new ArrayList<>();
        for (int i = 0; i < 100000; i++) big.add(i);
        long sum = 0; for (int v : big) sum += v;
        System.out.println(sum);
        StringJoiner sj = new StringJoiner(", ", "{", "}"); sj.add("a").add("b"); System.out.println(sj);
        Optional<String> op = Optional.of("val"); System.out.println(op.map(String::length).orElse(-1) + " " + Optional.empty().isPresent());
        nums.removeIf(n -> n > 4); System.out.println(nums);
        nums.forEach(n -> System.out.print(n + ";")); System.out.println();
        Comparator<String> cmp = Comparator.comparing(String::length).thenComparing(Comparator.reverseOrder());
        List<String> ws = new ArrayList<>(Arrays.asList("bb", "a", "ccc", "aa", "c"));
        ws.sort(cmp); System.out.println(ws);
    }
}
