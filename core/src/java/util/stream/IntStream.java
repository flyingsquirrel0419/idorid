package java.util.stream;
import java.util.function.IntConsumer;
public interface IntStream extends AutoCloseable { void forEach(IntConsumer action); int[] toArray(); long count(); int sum(); void close(); }
