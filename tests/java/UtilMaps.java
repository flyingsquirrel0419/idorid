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
  m.put("nil",null);System.out.println(m.getOrDefault("nil",42));System.out.println(m.putIfAbsent("nil",42));System.out.println(m.get("nil"));
 }
}
