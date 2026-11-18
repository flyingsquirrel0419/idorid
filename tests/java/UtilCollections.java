import java.util.*;
public class UtilCollections {
 public static void main(String[] args){
  List<Integer> a=new ArrayList<Integer>(Arrays.asList(3,1,2,1));Collections.sort(a);System.out.println(a);Collections.reverse(a);Collections.rotate(a,1);System.out.println(a);Collections.shuffle(a,new Random(123));System.out.println(a);System.out.println(Collections.frequency(a,1)+" "+Collections.min(a)+" "+Collections.max(a));
  System.out.println(Collections.indexOfSubList(a,Arrays.asList(1,2)));System.out.println(Collections.list(Collections.enumeration(a)));System.out.println(Collections.nCopies(3,"x"));
  List<Integer> u=Collections.unmodifiableList(a);try{u.set(0,9);}catch(Exception e){System.out.println(e.getClass().getSimpleName());}a.set(0,7);System.out.println(u);System.out.println(u instanceof RandomAccess);
  Map<String,Integer> m=Collections.singletonMap("a",1);try{m.entrySet().iterator().next().setValue(2);}catch(Exception e){System.out.println(e.getClass().getSimpleName());}System.out.println(m);
  Set<String> set=Collections.newSetFromMap(new HashMap<String,Boolean>());set.add("a");set.add("b");System.out.println(set.size()+" "+set.contains("a"));
  List<Integer> sub=a.subList(1,3);sub.remove(0);System.out.println(a+" "+sub);Iterator<Integer> it=a.iterator();a.add(4);try{it.next();}catch(Exception e){System.out.println(e.getClass().getSimpleName());}
 }
}
