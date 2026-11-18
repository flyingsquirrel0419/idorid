import java.util.*;

public class Objects1 {
    interface Shape { double area(); default String describe() { return getClass().getSimpleName() + " area=" + area(); } }
    static abstract class Base implements Shape {
        static int count;
        final String name;
        Base(String name) { this.name = name; count++; }
        public String toString() { return name + "(" + describe() + ")"; }
    }
    static class Circle extends Base { final double r; Circle(double r) { super("circle"); this.r = r; } public double area() { return Math.PI * r * r; } }
    static class Rect extends Base {
        final int w, h;
        Rect(int w, int h) { super("rect"); this.w = w; this.h = h; }
        public double area() { return w * h; }
        @Override public String describe() { return "Rect " + w + "x" + h; }
    }
    static class Square extends Rect { Square(int s) { super(s, s); } public double area() { return super.area() + 0.5; } }
    enum Color { RED, GREEN, BLUE; Color next() { return values()[(ordinal() + 1) % values().length]; } }
    static int sinit = initStatic();
    static int initStatic() { System.out.println("static init"); return 7; }
    static class MyException extends RuntimeException { MyException(String m) { super(m); } }

    static int depth(int n) { return n == 0 ? 0 : 1 + depth(n - 1); }

    public static void main(String[] args) throws Exception {
        List<Shape> shapes = new ArrayList<>();
        shapes.add(new Circle(1));
        shapes.add(new Rect(2, 3));
        shapes.add(new Square(4));
        for (Shape s : shapes) System.out.println(s);
        System.out.println("count=" + Base.count + " sinit=" + sinit);
        Object o = shapes.get(2);
        System.out.println((o instanceof Rect) + " " + (o instanceof Circle) + " " + (o instanceof Shape) + " " + o.getClass().getName());
        try { Circle c = (Circle) o; System.out.println(c); } catch (ClassCastException e) { System.out.println("CCE ok"); }
        Color c = Color.RED;
        for (int i = 0; i < 4; i++) { System.out.print(c + ":" + c.ordinal() + " "); c = c.next(); }
        System.out.println(Color.valueOf("BLUE").name());
        switch (c) { case GREEN: System.out.println("is green"); break; default: System.out.println("not green " + c); }
        try { throw new MyException("boom"); }
        catch (IllegalStateException e) { System.out.println("wrong"); }
        catch (RuntimeException e) { System.out.println("caught " + e.getMessage() + " " + e.getClass().getSimpleName()); }
        finally { System.out.println("finally"); }
        try { Object n = null; n.toString(); } catch (NullPointerException e) { System.out.println("NPE caught"); }
        try { int[] arr = new int[2]; arr[5] = 1; } catch (ArrayIndexOutOfBoundsException e) { System.out.println("AIOOBE caught"); }
        System.out.println(depth(3000));
        int[][] grid = new int[3][4];
        for (int i = 0; i < 3; i++) for (int j = 0; j < 4; j++) grid[i][j] = i * j;
        System.out.println(Arrays.toString(grid[2]) + " " + Arrays.deepToString(grid).length());
        long[] ls = {5, 3, 9, 1}; Arrays.sort(ls); System.out.println(Arrays.toString(ls));
        String[] strs = {"pear", "apple", "fig"}; Arrays.sort(strs); System.out.println(Arrays.toString(strs));
        Runnable r = new Runnable() { public void run() { System.out.println("anon run " + sinit); } };
        r.run();
        Comparator<String> byLen = new Comparator<String>() { public int compare(String a, String b) { return a.length() - b.length(); } };
        List<String> words = new ArrayList<>(Arrays.asList("ccc", "a", "bb", "dddd"));
        Collections.sort(words, byLen);
        System.out.println(words);
        Object[] objs = {1, "two", 3.0, 'c', null, 4L, true};
        for (Object x : objs) System.out.print((x == null ? "null" : x.getClass().getSimpleName()) + " ");
        System.out.println();
        Integer big1 = 1000, big2 = 1000, small1 = 100, small2 = 100;
        System.out.println((big1.equals(big2)) + " " + (small1 == small2) + " " + (big1 + big2));
        System.out.println(new int[0].getClass().getName() + " " + new String[0].getClass().getName() + " " + int.class.getName());
        StringBuilder sb = new StringBuilder();
        outer:
        for (int i = 0; i < 5; i++) { for (int j = 0; j < 5; j++) { if (j == 3) continue outer; if (i == 3) break outer; sb.append(i).append(j).append(' '); } }
        System.out.println(sb);
        System.out.println(Objects.hash(1, 2, 3) + " " + Objects.equals(null, null) + " " + Objects.toString(null, "dflt"));
        try { Class.forName("does.not.Exist"); } catch (ClassNotFoundException e) { System.out.println("CNFE " + e.getMessage()); }
        Object clone = new int[]{1, 2, 3}.clone(); System.out.println(((int[]) clone)[2]);
    }
}
