// min-api: 26
// d8-flags: --no-desugaring
import java.util.*;
import java.util.function.*;

public class Lambdas {
    interface IntOp { int apply(int a, int b); }
    interface Maker<T> { T make(String s); }
    static int twice(int x) { return x * 2; }
    int base = 10;
    int addBase(int x) { return x + base; }

    public static void main(String[] args) {
        Runnable r = () -> System.out.println("runnable");
        r.run();
        int k = 7;
        long big = 1L << 40;
        double dd = 2.5;
        IntOp add = (a, b) -> a + b + k;
        System.out.println(add.apply(1, 2));
        Function<Integer, Integer> f = x -> x * 3;
        System.out.println(f.apply(14));
        Function<Integer, Integer> m = Lambdas::twice;
        System.out.println(m.andThen(f).apply(5));
        Supplier<Long> sl = () -> big + 1;
        System.out.println(sl.get());
        DoubleUnaryOperator du = x -> x * dd;
        System.out.println(du.applyAsDouble(4));
        Lambdas self = new Lambdas();
        IntUnaryOperator bound = self::addBase;
        System.out.println(bound.applyAsInt(5));
        Function<Lambdas, Integer> unbound = o -> o.base;
        System.out.println(unbound.apply(self));
        Maker<StringBuilder> ctor = StringBuilder::new;
        System.out.println(ctor.make("ctor").append('!'));
        Supplier<ArrayList<String>> listMaker = ArrayList::new;
        ArrayList<String> list = listMaker.get();
        list.add("pear"); list.add("apple"); list.add("fig");
        list.sort(Comparator.comparing(String::length).thenComparing(Comparator.reverseOrder()));
        System.out.println(list);
        list.forEach(s -> System.out.print(s.toUpperCase() + " "));
        System.out.println();
        BiFunction<Integer, Integer, Integer> sum = Integer::sum;
        System.out.println(sum.apply(40, 2));
        ToIntFunction<String> len = String::length;
        System.out.println(len.applyAsInt("hello"));
        Predicate<String> empty = String::isEmpty;
        System.out.println(empty.negate().test("") + " " + empty.test(""));
        Map<String, Integer> counts = new TreeMap<>();
        for (String w : "a b a c b a".split(" ")) counts.merge(w, 1, Integer::sum);
        System.out.println(counts);
        counts.computeIfAbsent("z", key -> key.length() * 100);
        counts.replaceAll((key, v) -> v + 1);
        System.out.println(counts);
        List<Integer> nums = new ArrayList<>(Arrays.asList(5, 3, 8, 1));
        nums.removeIf(n -> n % 2 == 1);
        System.out.println(nums);
        Runnable nested = () -> {
            Runnable inner = () -> System.out.println("inner " + k);
            inner.run();
        };
        nested.run();
        Runnable same1 = Lambdas::hello, same2 = Lambdas::hello;
        same1.run(); same2.run();
        try {
            Function<Object, String> cast = o -> ((String) o).trim();
            cast.apply(null);
        } catch (NullPointerException e) {
            System.out.println("npe caught");
        }
        Comparator<int[]> byFirst = (x, y) -> Integer.compare(x[0], y[0]);
        int[][] arr = {{3}, {1}, {2}};
        Arrays.sort(arr, byFirst);
        System.out.println(arr[0][0] + "" + arr[1][0] + arr[2][0]);
        BooleanSupplier bs = () -> k > 3;
        System.out.println(bs.getAsBoolean());
        IntFunction<char[]> charArr = char[]::new;
        System.out.println(charArr.apply(3).length);
    }
    static void hello() { System.out.println("hello ref"); }
}
