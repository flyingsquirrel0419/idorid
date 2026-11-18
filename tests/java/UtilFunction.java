import java.util.function.*;
public class UtilFunction {
 public static void main(String[] args) {
  Function<Integer,Integer> f = x -> x + 2;
  Function<Integer,Integer> g = x -> x * 3;
  System.out.println(f.compose(g).apply(4));
  System.out.println(f.andThen(g).apply(4));
  Predicate<Integer> p = x -> x > 2;
  System.out.println(p.and(x -> x < 10).test(5));
  System.out.println(p.negate().test(1));
  System.out.println(IntUnaryOperator.identity().andThen(x -> x + 1).applyAsInt(6));
  System.out.println(((LongToDoubleFunction)(x -> x * 2.0)).applyAsDouble(4));
 }
}
