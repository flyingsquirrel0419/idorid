public class Arith {
    static int fib(int n) { return n < 2 ? n : fib(n - 1) + fib(n - 2); }
    static long fact(int n) { long r = 1; for (int i = 2; i <= n; i++) r *= i; return r; }
    public static void main(String[] args) {
        System.out.println(fib(20));
        System.out.println(fact(20));
        int a = Integer.MAX_VALUE; a++;
        System.out.println(a);
        System.out.println(Integer.MIN_VALUE / -1);
        System.out.println(-7 / 2 + " " + (-7 % 2) + " " + (7 >> 1) + " " + (-7 >>> 28) + " " + (1 << 33));
        long l = 1L << 40; System.out.println(l + " " + (l >>> 3) + " " + (-l >> 2) + " " + (l * l));
        System.out.println(0.1 + 0.2);
        System.out.println(1.0 / 3 + " " + (float) (1.0 / 3) + " " + 100.0 + " " + 1e10 + " " + 1.5e-5 + " " + -0.0);
        System.out.println((int) 3.99 + " " + (int) -3.99 + " " + (int) Double.NaN + " " + (long) 1e30 + " " + (int) Float.POSITIVE_INFINITY);
        System.out.println(Math.sqrt(2) + " " + Math.pow(2, 10) + " " + Math.abs(-5) + " " + Math.max(3, 9) + " " + Math.round(2.5) + " " + Math.round(-2.5));
        System.out.println(5 % 3 + " " + 5.5 % 2 + " " + (char) ('a' + 2) + " " + (byte) 200 + " " + (short) 70000);
        double d = 0; for (int i = 0; i < 1000; i++) d += i * 0.5; System.out.println(d);
        System.out.println(Long.MAX_VALUE + " " + Long.MIN_VALUE + " " + Integer.toHexString(-1) + " " + Long.toBinaryString(10));
        System.out.println(Integer.parseInt("-1234") + Integer.parseInt("ff", 16) + " " + Long.parseLong("9999999999"));
        System.out.println(Double.parseDouble("3.25") + " " + Float.parseFloat("1.1") + " " + Integer.valueOf(127).equals(127));
        try { System.out.println(1 / (args.length)); } catch (ArithmeticException e) { System.out.println("caught " + e.getClass().getName()); }
        System.out.println(Integer.bitCount(255) + " " + Integer.numberOfLeadingZeros(1) + " " + Long.numberOfTrailingZeros(64) + " " + Integer.reverse(1));
        System.out.println(Float.MAX_VALUE + " " + Double.MIN_VALUE + " " + (0.0f / 0.0f) + " " + Float.compare(1f, 2f));
        int x = 10; x += 5; x *= 2; x -= 3; x /= 2; x %= 7; x <<= 3; x >>= 1; x ^= 0xff; x |= 1; x &= 0x7f;
        System.out.println(x);
    }
}
