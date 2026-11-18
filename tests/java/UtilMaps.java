import java.util.*;
public class UtilMaps {
 public static void main(String[] args) {
  HashMap<String,Integer> m=new HashMap<String,Integer>();
  for(int i=0;i<60;i++)m.put("key"+i,i);
  for(int i=0;i<60;i+=4)m.remove("key"+i);
  System.out.println(m); System.out.println(new HashMap<String,Integer>(m));
  HashMap<Integer,String> n=new HashMap<Integer,String>(2);
  for(int i=-20;i<40;i++)n.put(i*37,"v"+i);
  for(int i=-20;i<40;i+=3)n.remove(i*37);
  System.out.println(n.keySet());
  HashSet<String> s=new HashSet<String>();for(int i=0;i<25;i++)s.add("item"+i);s.remove("item4");System.out.println(s);
  LinkedHashMap<String,Integer> linked=new LinkedHashMap<String,Integer>(4,.75f,true);linked.put("a",1);linked.put("b",2);linked.put("c",3);linked.get("a");linked.put("b",5);System.out.println(linked);linked.remove("c");linked.put("d",4);System.out.println(linked.keySet());
  LinkedHashMap<Integer,String> bounded=new LinkedHashMap<Integer,String>(){protected boolean removeEldestEntry(Map.Entry<Integer,String> e){return size()>3;}};for(int i=0;i<6;i++)bounded.put(i,"v"+i);System.out.println(bounded);
  LinkedHashSet<String> ordered=new LinkedHashSet<String>();ordered.add("z");ordered.add("a");ordered.add("b");ordered.remove("a");ordered.add("a");System.out.println(ordered);
  m.put("nil",null);System.out.println(m.getOrDefault("nil",42));System.out.println(m.putIfAbsent("nil",42));System.out.println(m.get("nil"));
 }
}
