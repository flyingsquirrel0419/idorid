package java.util.concurrent;
import java.util.function.*;
public interface CompletionStage<T> {
 <U> CompletionStage<U> thenApply(Function<? super T,? extends U> fn);
 CompletionStage<Void> thenAccept(Consumer<? super T> action);
 <U> CompletionStage<U> thenCompose(Function<? super T,? extends CompletionStage<U>> fn);
 <U,V> CompletionStage<V> thenCombine(CompletionStage<? extends U> other,BiFunction<? super T,? super U,? extends V> fn);
 CompletionStage<T> exceptionally(Function<Throwable,? extends T> fn);
 <U> CompletionStage<U> handle(BiFunction<? super T,Throwable,? extends U> fn);
 CompletionStage<T> whenComplete(BiConsumer<? super T,? super Throwable> fn);
 CompletableFuture<T> toCompletableFuture();
 <U> CompletionStage<U> thenApplyAsync(Function<? super T,? extends U> f); <U> CompletionStage<U> thenApplyAsync(Function<? super T,? extends U> f,Executor e);
 CompletionStage<Void> thenAcceptAsync(Consumer<? super T> f); CompletionStage<Void> thenAcceptAsync(Consumer<? super T> f,Executor e);
 CompletionStage<Void> thenRun(Runnable r); CompletionStage<Void> thenRunAsync(Runnable r); CompletionStage<Void> thenRunAsync(Runnable r,Executor e);
 <U> CompletionStage<U> thenComposeAsync(Function<? super T,? extends CompletionStage<U>> f); <U> CompletionStage<U> thenComposeAsync(Function<? super T,? extends CompletionStage<U>> f,Executor e);
 <U,V> CompletionStage<V> thenCombineAsync(CompletionStage<? extends U> o,BiFunction<? super T,? super U,? extends V> f); <U,V> CompletionStage<V> thenCombineAsync(CompletionStage<? extends U> o,BiFunction<? super T,? super U,? extends V> f,Executor e);
 <U> CompletionStage<Void> thenAcceptBoth(CompletionStage<? extends U> o,BiConsumer<? super T,? super U> f); <U> CompletionStage<Void> thenAcceptBothAsync(CompletionStage<? extends U> o,BiConsumer<? super T,? super U> f); <U> CompletionStage<Void> thenAcceptBothAsync(CompletionStage<? extends U> o,BiConsumer<? super T,? super U> f,Executor e);
 <U> CompletionStage<U> applyToEither(CompletionStage<? extends T> o,Function<? super T,U> f); <U> CompletionStage<U> applyToEitherAsync(CompletionStage<? extends T> o,Function<? super T,U> f); <U> CompletionStage<U> applyToEitherAsync(CompletionStage<? extends T> o,Function<? super T,U> f,Executor e);
 <U> CompletionStage<U> handleAsync(BiFunction<? super T,Throwable,? extends U> f); <U> CompletionStage<U> handleAsync(BiFunction<? super T,Throwable,? extends U> f,Executor e); CompletionStage<T> whenCompleteAsync(BiConsumer<? super T,? super Throwable> f); CompletionStage<T> whenCompleteAsync(BiConsumer<? super T,? super Throwable> f,Executor e);
}
