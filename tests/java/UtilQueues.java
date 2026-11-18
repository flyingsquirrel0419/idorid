import java.util.*;
public class UtilQueues {
 public static void main(String[] args) {
  ArrayDeque<Integer> d=new ArrayDeque<Integer>(3);for(int i=0;i<20;i++)d.addLast(i);for(int i=0;i<13;i++)d.removeFirst();for(int i=20;i<40;i++)d.addLast(i);d.addFirst(-1);d.removeLastOccurrence(24);System.out.println(d);
  Iterator<Integer> it=d.descendingIterator();while(it.hasNext()){int n=it.next();if(n%7==0)it.remove();}System.out.println(d);
  PriorityQueue<Integer> q=new PriorityQueue<Integer>();int[] a={9,2,5,1,8,3,7,4,6,0,5,3};for(int x:a)q.add(x);System.out.println(q);q.remove(5);System.out.println(q);while(!q.isEmpty())System.out.print(q.poll()+" ");System.out.println();
  LinkedList<String> l=new LinkedList<String>();l.add("b");l.addFirst("a");l.addLast("c");l.push("x");System.out.println(l);System.out.println(l.pop()+" "+l.removeLast()+" "+l);
  Stack<Integer> s=new Stack<Integer>();s.push(2);s.push(5);s.push(2);System.out.println(s+" "+s.search(2)+" "+s.pop());
 }
}
