public class Bench {
    static int fib(int n) { return n < 2 ? n : fib(n - 1) + fib(n - 2); }
    static long loop(int n) { long s = 0; for (int i = 0; i < n; i++) { s += i ^ (i >> 3); } return s; }
    static int sieve(int n) {
        boolean[] c = new boolean[n + 1]; int count = 0;
        for (int i = 2; i <= n; i++) { if (!c[i]) { count++; for (long j = (long) i * i; j <= n; j += i) c[(int) j] = true; } }
        return count;
    }
    static String strings(int n) { StringBuilder sb = new StringBuilder(); for (int i = 0; i < n; i++) { sb.append(i % 10); if (sb.length() > 1000) sb.setLength(0); } return sb.toString(); }
    static int objects(int n) { int s = 0; for (int i = 0; i < n; i++) { int[] a = new int[8]; a[i & 7] = i; s += a[i & 7] & 1; } return s; }
    interface Op { int apply(int x); }
    static int virt(int n) {
        Op a = new Op() { public int apply(int x) { return x + 1; } };
        Op b = new Op() { public int apply(int x) { return x * 2 & 0xffff; } };
        int s = 0; for (int i = 0; i < n; i++) s = (i & 1) == 0 ? a.apply(s) : b.apply(s); return s;
    }
    static void time(String name, Runnable r) {
        long t = System.nanoTime(); r.run(); long ms = (System.nanoTime() - t) / 1000000;
        System.out.println(name + ": " + ms + " ms");
    }
    public static void main(String[] args) {
        time("fib(27)", new Runnable() { public void run() { System.out.println("  " + fib(27)); } });
        time("loop 10M", new Runnable() { public void run() { System.out.println("  " + loop(10000000)); } });
        time("sieve 2M", new Runnable() { public void run() { System.out.println("  " + sieve(2000000)); } });
        time("strings 1M", new Runnable() { public void run() { System.out.println("  " + strings(1000000).length()); } });
        time("alloc 1M", new Runnable() { public void run() { System.out.println("  " + objects(1000000)); } });
        time("virtual 5M", new Runnable() { public void run() { System.out.println("  " + virt(5000000)); } });
    }
}
