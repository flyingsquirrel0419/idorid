import java.lang.reflect.*;
import java.util.*;

public class Reflect1 {
    static class Base { public int base = 1; public String hello() { return "base"; } }
    public static class Thing extends Base implements Comparable<Thing> {
        public static int created;
        private final String name;
        public long big = 5L;
        public double ratio;
        public Thing() { this("default"); }
        public Thing(String n) { name = n; created++; }
        Thing(String n, int times) { StringBuilder b = new StringBuilder(); for (int i = 0; i < times; i++) b.append(n); name = b.toString(); created++; }
        public String hello() { return "thing:" + name; }
        public int add(int a, long b) { return (int) (a + b); }
        public static String shout(String s) { return s.toUpperCase(); }
        private void boom() { throw new IllegalStateException("boom"); }
        public int compareTo(Thing o) { return name.compareTo(o.name); }
    }

    public static void main(String[] args) throws Exception {
        Constructor<Thing> c1 = Thing.class.getConstructor(String.class);
        Thing t = c1.newInstance("x");
        System.out.println(t.hello());
        Constructor<Thing> c2 = Thing.class.getDeclaredConstructor(String.class, int.class);
        System.out.println(c2.newInstance("ab", 3).hello());
        System.out.println(Thing.class.getDeclaredConstructor().newInstance().hello());
        System.out.println(Thing.class.getConstructors().length + " public ctors");
        Method add = Thing.class.getMethod("add", int.class, long.class);
        System.out.println(add.invoke(t, 40, 2L) + " " + add.getReturnType());
        System.out.println(Thing.class.getMethod("shout", String.class).invoke(null, "hi"));
        Method hello = Base.class.getMethod("hello");
        System.out.println(hello.invoke(t));
        Method boom = Thing.class.getDeclaredMethod("boom");
        boom.setAccessible(true);
        try { boom.invoke(t); } catch (InvocationTargetException e) { System.out.println("ite: " + e.getCause().getMessage()); }
        try { add.invoke(t, "nope", 1L); } catch (IllegalArgumentException e) { System.out.println("iae"); }
        Field big = Thing.class.getField("big");
        System.out.println(big.get(t) + " " + big.getType());
        big.setLong(t, 77L);
        System.out.println(t.big);
        Field ratio = Thing.class.getField("ratio");
        ratio.set(t, 2.5);
        System.out.println(t.ratio + " " + ratio.getDouble(t));
        Field created = Thing.class.getField("created");
        System.out.println("created=" + created.getInt(null) + " static=" + Modifier.isStatic(created.getModifiers()));
        Field name = Thing.class.getDeclaredField("name");
        name.setAccessible(true);
        System.out.println(name.get(t));
        System.out.println(Base.class.getField("base").getInt(t));
        List<String> names = new ArrayList<>();
        for (Method m : Thing.class.getDeclaredMethods()) if (!m.isSynthetic() && !m.isBridge()) names.add(m.getName());
        Collections.sort(names);
        System.out.println(names);
        System.out.println(Arrays.toString(Thing.class.getInterfaces()));
        System.out.println(Modifier.toString(add.getModifiers()));
        try { Thing.class.getMethod("missing"); } catch (NoSuchMethodException e) { System.out.println("nsme"); }
        Object list = ArrayList.class.getConstructor().newInstance();
        List.class.getMethod("add", Object.class).invoke(list, "via-interface");
        System.out.println(list);
        System.out.println(Integer.class.getMethod("parseInt", String.class).invoke(null, "123"));
    }
}
