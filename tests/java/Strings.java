public class Strings {
    public static void main(String[] args) {
        String s = "Hello, World";
        System.out.println(s.length() + " " + s.charAt(4) + " " + s.indexOf("World") + " " + s.indexOf('z') + " " + s.lastIndexOf('o'));
        System.out.println(s.substring(7) + "|" + s.substring(0, 5) + "|" + s.toUpperCase() + "|" + s.toLowerCase());
        System.out.println(s.replace('l', 'L') + " " + s.replace("World", "Java") + " " + s.contains("lo,") + " " + s.startsWith("Hell") + " " + s.endsWith("ld"));
        System.out.println("  trim me  ".trim() + "|" + "a,b,,c,".split(",").length + "|" + String.join("-", "x", "y", "z"));
        for (String p : "one two  three".split("\\s+")) System.out.print("[" + p + "]");
        System.out.println();
        StringBuilder sb = new StringBuilder();
        for (int i = 0; i < 5; i++) sb.append(i).append(',');
        sb.setLength(sb.length() - 1);
        sb.insert(0, "<").append('>').reverse();
        System.out.println(sb + " " + sb.length() + " " + sb.indexOf("3"));
        System.out.println("abc".compareTo("abd") + " " + "b".compareTo("a") + " " + "Abc".equalsIgnoreCase("aBC") + " " + "abc".hashCode() + " " + "".isEmpty());
        System.out.println(String.valueOf(3.5) + String.valueOf(true) + String.valueOf('c') + String.valueOf(42L) + null);
        String k = "한국어 텍스트";
        System.out.println(k + " " + k.length() + " " + k.charAt(1) + " " + k.substring(4) + " " + (int) k.charAt(0));
        System.out.println(String.format("%d|%5d|%-5d|%05d|%,d|%x|%X|%s|%.2f|%8.3f|%b|%c|%%|%10s|%-6s|", 42, 42, 42, 42, 1234567, 255, 255, "str", 3.14159, 2.5, true, 'z', "right", "left"));
        System.out.println(String.format("%s scored %d points (%.1f%%)", "Ann", 90, 92.5));
        char[] cs = s.toCharArray(); java.util.Arrays.sort(cs); System.out.println(new String(cs).trim());
        System.out.println("a1b2c3".replaceAll("[0-9]", "#") + " " + "x.y.z".split("\\.").length + " " + "2024-01-05".matches("\\d{4}-\\d{2}-\\d{2}"));
        String t = null;
        System.out.println("null concat: " + t);
        switch (s.substring(0, 5)) {
            case "Hello": System.out.println("switch hello"); break;
            case "World": System.out.println("switch world"); break;
            default: System.out.println("switch default");
        }
        System.out.println("abc" + "abc" + " " + "Mississippi".lastIndexOf("ss") + " " + "a-b-c".indexOf('-', 2));
        String interned = new String("abc").intern();
        System.out.println((interned == "abc") + " " + (new String("abc") == "abc") + " " + new String("abc").equals("abc"));
        System.out.println(Character.isDigit('7') + " " + Character.isLetter('x') + " " + Character.toUpperCase('q') + " " + Character.isWhitespace(' ') + " " + (int) Character.forDigit(11, 16));
    }
}
