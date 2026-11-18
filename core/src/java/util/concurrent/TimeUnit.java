package java.util.concurrent;
public enum TimeUnit {
 NANOSECONDS(1L), MICROSECONDS(1000L), MILLISECONDS(1000000L), SECONDS(1000000000L), MINUTES(60000000000L), HOURS(3600000000000L), DAYS(86400000000000L);
 private final long nanos; TimeUnit(long n){nanos=n;}
 public long convert(long value,TimeUnit source){if(source==null)throw new NullPointerException(); if(source.nanos<=nanos)return value/(nanos/source.nanos);long factor=source.nanos/nanos;if(value>Long.MAX_VALUE/factor)return Long.MAX_VALUE;if(value<Long.MIN_VALUE/factor)return Long.MIN_VALUE;return value*factor;}
 public long toNanos(long n){return NANOSECONDS.convert(n,this);} public long toMicros(long n){return MICROSECONDS.convert(n,this);} public long toMillis(long n){return MILLISECONDS.convert(n,this);} public long toSeconds(long n){return SECONDS.convert(n,this);} public long toMinutes(long n){return MINUTES.convert(n,this);} public long toHours(long n){return HOURS.convert(n,this);} public long toDays(long n){return DAYS.convert(n,this);}
 public void sleep(long timeout) throws InterruptedException {} public void timedJoin(Thread thread,long timeout)throws InterruptedException {if(timeout>0)thread.join();} public void timedWait(Object obj,long timeout)throws InterruptedException {if(timeout>0)obj.wait(toMillis(timeout));}
}
