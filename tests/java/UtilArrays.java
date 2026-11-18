import java.util.*;
public class UtilArrays {
 public static void main(String[] args) {
  int[] a={9,3,-1,3,0}; Arrays.sort(a,1,4); System.out.println(Arrays.toString(a)); System.out.println(Arrays.binarySearch(a,1,4,2));
  double[] d={Double.NaN,0.0,-0.0,2,-1,Double.NaN}; Arrays.sort(d); System.out.println(Arrays.toString(d)); System.out.println(Arrays.binarySearch(d,-0.0));
  String[] s={"bbb","a","cc","dd"}; Arrays.sort(s,new Comparator<String>(){public int compare(String a,String b){return a.length()-b.length();}}); System.out.println(Arrays.toString(s));
  System.out.println(Arrays.toString(Arrays.copyOf(s,6))); System.out.println(Arrays.toString(Arrays.copyOfRange(a,3,7)));
  Object[] x={new int[]{1,2},new Object[]{"x",new boolean[]{true,false}}};System.out.println(Arrays.deepToString(x));System.out.println(Arrays.deepEquals(x,new Object[]{new int[]{1,2},new Object[]{"x",new boolean[]{true,false}}}));System.out.println(Arrays.deepHashCode(x));
  List<String> l=Arrays.asList(s);l.set(0,"z");System.out.println(Arrays.toString(s));System.out.println(l instanceof RandomAccess);System.out.println(Arrays.toString(l.toArray(new String[0])));
  Arrays.setAll(a,i->i*i);System.out.println(Arrays.toString(a));
  byte[] b={3,-2,1};short[] h={4,-2,1};long[] g={9,-8,2};char[] c={'z','a','m'};float[] f={Float.NaN,0f,-0f,1f};
  Arrays.parallelSort(b);Arrays.sort(h);Arrays.sort(g);Arrays.sort(c);Arrays.sort(f);System.out.println(Arrays.toString(b)+Arrays.toString(h)+Arrays.toString(g)+Arrays.toString(c)+Arrays.toString(f));
  System.out.println(Arrays.binarySearch(b,(byte)1)+" "+Arrays.binarySearch(h,(short)0)+" "+Arrays.binarySearch(g,9)+" "+Arrays.binarySearch(c,'m')+" "+Arrays.binarySearch(f,-0f));
  boolean[] z={true,false};System.out.println(Arrays.toString(Arrays.copyOf(z,4))+" "+Arrays.hashCode(z)+" "+Arrays.equals(z,new boolean[]{true,false}));
  Object[] cycle=new Object[1];cycle[0]=cycle;System.out.println(Arrays.deepToString(cycle));
  try{Arrays.sort(a,4,2);}catch(Exception e){System.out.println(e.getClass().getSimpleName());}

 }
}
